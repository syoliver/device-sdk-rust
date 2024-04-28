#![allow(unused_imports)]

use std::vec::Vec;
use std::error::Error;
use tokio::task::JoinSet;
use std::pin::Pin;
use futures::future::BoxFuture;
use std::sync::{Arc, RwLock};
use std::ops::FnOnce;

pub use self::bootstrap_error::BootstrapError;
mod bootstrap_error;

pub use self::registry::Registry;
mod registry;

// type AsyncModuleFactory = Fn(Arc<RwLock<Registry>>) -> Future<Result<Any, Box<dyn Error>>>;

pub struct Bootstrapper {
    // registry: Arc<Mutex<Registry>>,
    // module_factories: Vec<Pin<Arc<AsyncModuleFactory>>>,
}


impl Bootstrapper {
    pub fn new(registry: Arc<RwLock<Registry>>) -> Self {
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
                    self.registry.borrow_mut().register::<Module>(module);
                    Ok(())
                }
            }
        });
        */
        self
    }
    */
    pub async fn run<'a>(&'a self) -> Result<(), Box<dyn Error>> {
        /*
        let mut set: Box<JoinSet<Result<(), Box<dyn Error + Send + Sync>>>> = Box::new(JoinSet::new());

        let task_handles: Vec<_> = self.module_factories
                .iter()
                .map(|factory: &Pin<Arc<AsyncModuleFactory>>|{
                    set.spawn(factory(self.registry))
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
}