use std::error::Error;
use log;
use crate::service::Service;
use crate::protocol::Driver;
use crate::server::RestServer;
use tokio::sync::Notify;
use std::sync::Arc;

pub mod command;
pub mod protocol;
pub mod service;
pub mod device;
mod server;

pub fn bootstrap(servicekey: &str, serviceversion: &str, driver: Box<dyn Driver>) -> Result<Arc<Notify>, Box<dyn Error>> {
	match Service::new(servicekey, serviceversion, driver) {
		Err(err) => {
			log::error!("{}", err);
			Err(err.into())
		},
		Ok(service) => {
			let notify_close = Arc::new(Notify::new());
			RestServer::new(service.clone()).run("0.0.0.0:8080".to_string(), notify_close.clone());

			match service.run() {
				Err(err) => {
					log::error!("Device Service {}", err);
					Err(err.into())
				}
				Ok(()) => Ok(notify_close)
			}
		}
	}
}

pub async fn shutdown_signal() {
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

    log::info!("Graceful shutdown of rest server");
}
