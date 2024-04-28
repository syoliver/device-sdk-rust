#![allow(unused_imports)]

use std::vec::Vec;
use std::error::Error;
use tokio::task::JoinSet;
use std::pin::Pin;
use futures::future::BoxFuture;
use std::sync::Arc;
use std::ops::FnOnce;

pub use self::bootstrap_error::BootstrapError;
mod bootstrap_error;

pub use self::registry::Registry;
mod registry;

// type AsyncModuleFactory = FnOnce() -> BoxFuture<'static, Result<(), Box<dyn Error + Send + Sync>>>;

pub struct Bootstrapper {
    // registry: Arc<Registry>,
    // module_factories: Vec<Pin<Arc<AsyncModuleFactory>>>,
}


impl Bootstrapper {
    pub fn new(registry: Arc<Registry>) -> Self {
        Bootstrapper{
            // registry: registry,
            // module_factories: vec!()
        }
    }
/*
    pub fn register<Module>(&mut self, factory: Pin<Arc<AsyncModuleFactory>>) -> &mut Self {
        /*
        self.module_factories.push(async move {
            match factory().await {
                Err(err) => Err(err),
                Ok(module) => {
                    self.registry.register::<Module>(module);
                    Ok(())
                }
            }
        });
        self
    }
        */

    pub async fn run<'a>(&'a self) -> Result<(), Box<dyn Error>> {
        /*
        let mut set: Box<JoinSet<Result<(), Box<dyn Error + Send + Sync>>>> = Box::new(JoinSet::new());

        let task_handles: Vec<_> = self.module_factories
                .iter()
                .map(|factory: &Pin<Arc<AsyncModuleFactory>>|{
                    set.spawn(factory())
                })
                .collect();

        while let Some(res) = set.join_next().await {
            if let Err(err) = res {
                task_handles.iter().for_each(|handle|{handle.abort();});
                return Err(Box::new(err))
            }
        }
        */
        Ok(())
    }
    */
}