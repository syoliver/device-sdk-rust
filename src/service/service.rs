#![allow(dead_code)]

use std::error::Error;
use crate::protocol::Driver;
use std::sync::Arc;

pub struct Service {
    service_key: String,
    service_version: String,
    driver: Box<dyn Driver>,
}

impl Service {
    pub fn new(service_key: &str, service_version: &str, driver: Box<dyn Driver>) -> Result<Arc<Self>, Box<dyn Error>> {
        Ok(Arc::new(Self{
            service_key: service_key.to_string(),
            service_version: service_version.to_string(),
            driver: driver,
        }))
    }


    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

