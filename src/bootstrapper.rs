#![allow(unused_imports)]

use std::vec::Vec;
use std::error::Error;
use tokio::{task::JoinSet, runtime::{Handle, Runtime}, time::{sleep, Duration}};
use std::pin::Pin;
use futures::future::BoxFuture;
use std::future::Future;
use std::sync::{Arc, Mutex, RwLock};
use std::ops::FnOnce;
use std::any::Any;
use async_trait::async_trait;
use futures;
use std::rc::Rc;

pub use self::bootstrap_error::BootstrapError;
mod bootstrap_error;

pub use self::registry::{Registry, Provided, RegistryError};
mod registry;

#[async_trait]
pub trait Factory {
    type Module;

    async fn create(&self, registry: &Registry) -> Provided<Self::Module>;
}

struct AnyFactory {
    // ?!
}

pub struct Bootstrapper<'a> {
    registry: Rc<Registry<'a>>,
    module_factories: Vec<Rc<AnyFactory>>,
}


impl<'a> Bootstrapper<'a> {
    pub fn new() -> Self {
        
        Bootstrapper{
            registry: Rc::new(Registry::new()),
            module_factories: vec!()
        }
    }
    
    pub fn register<Module>(&self, bootstrappable: Box<dyn Factory<Module = Module>>) -> Self
    {
        let module = bootstrappable.create(&self.registry);

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
        Self{
            registry: self.registry.clone(),
            module_factories: self.module_factories.to_vec(),
        }
    }
    
    pub async fn run(&self) -> Result<(), Box<dyn Error>> {
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


#[cfg(test)]
mod tests {
    use super::*;

    
    #[tokio::test]
    async fn test_boostrap() {

        struct First {
            value: i32,
        }
        

        struct FirstFactory {}

        #[async_trait]
        impl Factory for FirstFactory {
            type Module = First;

            async fn create(&self, _registry: &Registry) -> Provided<Self::Module> {
                sleep(Duration::from_millis(1000)).await;
                Ok(Arc::new(Mutex::new(First{value: 42})))
            }
        }

        struct Second {
            value: String,
        }

        struct SecondFactory {
        }

        #[async_trait]
        impl Factory for SecondFactory {
            type Module = Second;


            async fn create(&self, registry: &Registry) -> Provided<Self::Module> {
                if let Some(provided) = registry.provide::<First>().await {
                    Ok(
                        Arc::new(
                            Mutex::new(
                                Second{
                                    value: provided.unwrap().lock().unwrap().value.to_string()
                                }
                            )
                        )
                    )
                } else {
                    Err(RegistryError::Error("Got an error".to_owned()))
                }
            }
        }

        let mut boostrap = Bootstrapper::new();

        boostrap
            .register::<First>(Box::new(FirstFactory{}))
            .register::<Second>(Box::new(SecondFactory{}));

    }
}

