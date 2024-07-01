use std::{error::Error, net::SocketAddr};
use tokio::{signal, select};
mod api;
mod models;
use api::router::get_router;

 
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>>{
    let app = get_router();

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
            signal::unix::signal(signal::unix::SignalKind::interrupt()).expect("Failed listening for SIGTERM").recv().await
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