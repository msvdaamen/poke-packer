mod auth;
mod card;
mod config;
mod cron;
mod instrumentation;
mod pkg;
mod router;
mod user;

use std::sync::Arc;

use sqlx::postgres::PgPoolOptions;
use tokio_util::sync::CancellationToken;

use crate::{
    config::FromEnv, cron::CronScheduler, instrumentation::Instrumentation, router::create_router,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::from_env()?;

    let instrumentation = Instrumentation::new(false);
    instrumentation.start();

    let token = CancellationToken::new();
    let db_pool = PgPoolOptions::new().connect(&config.database_url).await?;
    let cron_manager = Arc::new(CronScheduler::new(token.clone()));

    let router = create_router(&config, db_pool, cron_manager);
    let addr = format!("0.0.0.0:{}", config.port.clone());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    let signal = shutdown_signal();
    let test_token = token.clone();
    tokio::spawn(async move {
        signal.await;
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        test_token.cancel();
    });
    let http_task = axum::serve(listener, router)
        .with_graceful_shutdown(async move { token.cancelled().await });

    println!("Server running on: http://{}", addr);

    http_task.await.unwrap();

    instrumentation.stop();

    Ok(())
}

async fn shutdown_signal() -> () {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
