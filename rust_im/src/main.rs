use std::{error::Error, net::SocketAddr};
use axum::{extract::ws::CloseFrame, http::header::CONNECTION};
use bootstrap::bootstrap::Bootstrap;
use config::config::Config;
use io::io::{Io};
use lazy_static::lazy_static;
use dotenv::dotenv;

use tokio::{signal, select};
mod bootstrap;
mod api_server;
mod ws_server; 
mod operations;
mod io;
mod operation;
mod repo;
mod config;
mod models;
use config::config::Configuration;
use api_server::controllers::router::get_router;
use tracing::info;
use ws_server::connection_manager::ConnectionManager;
mod test_setups;
mod utils;

lazy_static!{
    static ref CONFIG: Config = Config::new();
    static ref CONNECTION_MANAGER: ConnectionManager = ConnectionManager::new();
}
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>>{
    let _ = dotenv();
    if CONFIG.bootstrap.deploy_bootstrap{
        info!("Deploying bootstrap");
        Bootstrap::deploy(bootstrap::bootstrap::BootstrapMode::Prod).await?;
    }
    Io::init().await;
    let subscriber = tracing_subscriber::fmt()
        .json()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_max_level(CONFIG.tracing_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let app = get_router();
    println!("{:?}", app);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await?; 
    axum::serve(listener, app.into_make_service()).with_graceful_shutdown(shutdown_signal()).await?;
    Ok(())
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await
            .expect("Failed listening for ctrl+c handling");
    };
    let terminate = async{
        let sigterm = async {
            #[cfg(unix)]
            signal::unix::signal(signal::unix::SignalKind::terminate()).expect("Failed listening for SIGTERM").recv().await;
            #[cfg(windows)]
            signal::windows::ctrl_c().expect("Failed listening for ").recv().await;
        };
        let sigint = async {
            #[cfg(unix)]
            signal::unix::signal(signal::unix::SignalKind::interrupt()).expect("Failed listening for SIGINT").recv().await;
            #[cfg(windows)]
            signal::windows::ctrl_shutdown().expect("Failed listening for CTRL+shutdown").recv().await;

        };
        select!{
            _ = sigterm => {println!("Recived SIGTERM")},
            _ = sigint => {println!("Recived SIGINT")},
        };
    };
    select! {
        _ = terminate => {println!("Recieved termination signal - terminating")},
        _ = ctrl_c => {println!("Recieved CTRL + C - terminating")}
    }
    CONNECTION_MANAGER.handle_shutdown().await;
}