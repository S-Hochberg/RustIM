use std::{env, error::Error, net::SocketAddr};
use bootstrap::bootstrap::Bootstrap;
use config::config::Config;
use lazy_static::lazy_static;
use sqlx::postgres::PgPoolOptions;
use tokio::{signal, select};
mod bootstrap;
mod api;
mod config;
mod models;
use config::config::Configuration;
use api::router::get_router;

lazy_static!{
    static ref CONFIG: Config = Config::new();
}
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>>{
    let subscriber = tracing_subscriber::fmt()
        .json()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .with_max_level(CONFIG.tracing_level)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    if CONFIG.bootstrap.deploy_bootstrap{
        Bootstrap::deploy().await?;
    }
    let app = get_router();
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect("postgres://postgres:password@127.0.0.1:5432").await.unwrap();
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
            signal::unix::signal(signal::unix::SignalKind::terminate()).expect("Failed listening for SIGTERM").recv().await
        };
        let sigint = async {
            signal::unix::signal(signal::unix::SignalKind::interrupt()).expect("Failed listening for SIGINT").recv().await
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
}