mod app;
mod config;
mod cron;
mod instrumentation;
mod router;

use std::sync::Arc;

use shared::config::FromEnv;
use sqlx::postgres::PgPoolOptions;

use crate::{app::create_app, cron::CronScheduler, instrumentation::Instrumentation};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::from_env()?;

    let instrumentation = Instrumentation::new(false);
    instrumentation.start();

    let db_pool = PgPoolOptions::new().connect(&config.database_url).await?;
    let cron_manager = Arc::new(CronScheduler::new());

    let router = create_app(&config, db_pool, cron_manager);
    let addr = format!("0.0.0.0:{}", config.port.clone());
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    let http_task = axum::serve(listener, router)
        .with_graceful_shutdown(async move { shutdown_signal().await });
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
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
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
