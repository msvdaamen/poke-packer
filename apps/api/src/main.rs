mod app;
mod config;
mod cron;
mod grpc;
mod instrumentation;
mod router;

use std::sync::Arc;

use crate::{app::create_app, cron::CronScheduler, instrumentation::Instrumentation};
use shared::config::FromEnv;
use sqlx::postgres::PgPoolOptions;
use tokio::select;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::Config::from_env()?;

    let instrumentation = Instrumentation::new(config.is_production);
    instrumentation.start();

    let db_pool = PgPoolOptions::new().connect(&config.database_url).await?;
    let cron_manager = Arc::new(CronScheduler::new());

    let (http_router, grpc_router) = create_app(&config, db_pool, cron_manager);
    let http_addr = format!("[::1]:{}", config.port.clone());
    let listener = tokio::net::TcpListener::bind(&http_addr).await.unwrap();

    let http_task = axum::serve(listener, http_router)
        .with_graceful_shutdown(async move { shutdown_signal().await });

    let grpc_addr = "[::1]:50051".parse()?;

    let grpc_task = grpc_router.serve_with_shutdown(grpc_addr, shutdown_signal());

    println!("Http server running on: http://{}", http_addr);
    println!("Grpc server running on: http://{}", grpc_addr);

    select! {
        _ = http_task => {}
        _ = grpc_task => {}
    }

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
