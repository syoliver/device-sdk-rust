#[allow(unused_imports)]

use std::error::Error;
use env_logger;
use log;
use crate::service::Service;
use crate::protocol::Driver;
// use crate::server::RestServer;
// use crate::bootstrapper::Bootstrapper;
use tokio::sync::Notify;
use std::sync::Arc;

pub mod command;
pub mod protocol;
pub mod service;
pub mod device;
pub mod bootstrapper;
pub mod modules;
mod server;

pub async fn bootstrap(servicekey: &str, serviceversion: &str, driver: Box<dyn Driver>) -> Result<Arc<Notify>, Box<dyn Error>> {
    let env_variable = format!("{}_LOG", servicekey.to_uppercase());
    env_logger::Builder::new()
        .parse_env(env_variable)
        .init();

    match Service::new(servicekey, serviceversion, driver) {
		Err(err) => {
			log::error!("{}", err);
			Err(err.into())
		},
		Ok(_service) => {
			let notify_close = Arc::new(Notify::new());
            /*

            let service_rest_server = Arc::clone(&service);
            let notify_close_rest_server = Arc::clone(&notify_close);

            let registry = Arc::new(bootstrapper::Registry::new());
            let bootstrap = Bootstrapper::new(registry)
                .register::<modules::Configuration>(Arc::pin(async move {
                    modules::Configuration::load()
                }))
                .register::<RestServer>(Arc::pin(async move {
                    let config = registry.provide::<modules::Configuration>()?.await;
                    let service_port = config.get(&["Service", "Port"])?;
                    let rest_api = RestServer::new(Arc::clone(&service_rest_server))
                        .run(format!("0.0.0.0:{service_port}"), Arc::clone(&notify_close_rest_server));
                    Ok(rest_api)
                }));

            if let Err(err) = bootstrap.run().await {
                log::error!("Bootstrap failure {}", err);
                Err(err.into())
            } else {
                match service.run() {
                    Err(err) => {
                        log::error!("Device Service {}", err);
                        Err(err.into())
                    }
                    Ok(()) => Ok(notify_close)
                }
            }
            */
            Ok(notify_close)
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
