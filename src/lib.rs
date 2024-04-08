use std::error::Error;
use log;
use crate::service::Service;
use crate::protocol::Driver;

mod command;
mod protocol;
mod service;
mod device;

pub fn bootstrap(servicekey: &str, serviceversion: &str, driver: Box<dyn Driver>) -> Result<(), Box<dyn Error>> {
	match Service::new(servicekey, serviceversion, driver) {
		Err(err) => {
			log::error!("{}", err);
			Err(err.into())
		},
		Ok(service) => match service.run() {
			Err(err) => {
				log::error!("Device Service {}", err);
				Err(err.into())
			}
			Ok(()) => Ok(())
		}
	}
}
