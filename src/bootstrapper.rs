use std::vec::Vec;
use std::error::Error;
use tokio::task::JoinSet;
use std::pin::Pin;
use futures::future::FutureExt;

pub use self::bootstrap_error::BootstrapError;
mod bootstrap_error;

pub struct Bootstrapper {
    handlers: Vec<Pin<Box<dyn Fn() -> Result<(), Box<dyn Error + Send + Sync>>>>>
}


impl Bootstrapper {
    pub fn new() -> Box<Self> {
        Box::new(Bootstrapper{
            handlers: vec!()
        })
    }

    pub fn add(mut self: Box<Self>, handle: Pin<Box<dyn Fn() -> Result<(), Box<dyn Error + Send + Sync>>>>) -> Box<Self> {
        self.handlers.push(handle);
        self
    }

    pub async fn run(&self) -> Result<(), BootstrapError> {
        let mut set: JoinSet<Result<(), Box<dyn Error + Send + Sync>>> = JoinSet::new();
        for handler in &self.handlers {
            set.spawn(
                futures::future::ready(handler()).boxed()
            );
        }

        let mut err = BootstrapError::new();

        while let Some(res) = set.join_next().await {
            if let Err(bootstrap_err) = res {
                let bootstrap_err_box: Box<dyn Error> = Box::new(bootstrap_err);
                err.append(bootstrap_err_box);
            }
        }

        if err.contains_error() {
            Err(err)
        } else {
            Ok(())
        }
    }
}