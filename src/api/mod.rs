use actix_web::{App, HttpServer, dev::ServerHandle, web};
use error_mapper::{TheResult, create_new_error};
use std::sync::OnceLock;
use tokio::sync::mpsc::{Receiver, Sender};

mod life_services;

static STOP_SENDER: OnceLock<Sender<()>> = OnceLock::new();
const SERVER_BIND: (&str, u16) = ("127.0.0.1", 8090);

pub async fn init_api() -> TheResult<()> {
    //  Init stop channel
    let (sender, receiver) = tokio::sync::mpsc::channel::<()>(5);
    let _ = STOP_SENDER.get_or_init(|| sender);

    let server = HttpServer::new(|| {
        App::new().service(web::scope("/api").configure(life_services::service_router))
    })
    .bind(SERVER_BIND)
    .map_err(|error| create_new_error!(error))?
    .run();

    //  Handle stop service
    tokio::task::spawn(stop_handler(server.handle(), receiver));

    let _result = server.await;

    Ok(())
}

async fn stop_handler(server: ServerHandle, mut stopper: Receiver<()>) {
    let _ = stopper.recv().await;
    server.stop(true).await;
}
