use std::error::Error;
use log;
use crate::service::Service;
use crate::protocol::Driver;
use crate::server::RestServer;

mod command;
mod protocol;
mod service;
mod device;
mod server;

pub fn bootstrap(servicekey: &str, serviceversion: &str, driver: Box<dyn Driver>) -> Result<(), Box<dyn Error>> {
	match Service::new(servicekey, serviceversion, driver) {
		Err(err) => {
			log::error!("{}", err);
			Err(err.into())
		},
		Ok(service) => {
			RestServer::new(service).run("0.0.0.0:8080");

			match service.run() {
				Err(err) => {
					log::error!("Device Service {}", err);
					Err(err.into())
				}
				Ok(()) => Ok(())
			}
		}
	}
}
