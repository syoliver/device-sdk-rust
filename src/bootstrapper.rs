#![allow(unused_imports)]

use std::error::Error;
use tokio::{task::JoinSet, runtime::{Handle, Runtime}, time::{sleep, Duration}, sync::{Mutex, Barrier}};
use std::pin::Pin;
use futures::{FutureExt, TryFutureExt, future::{self, BoxFuture}};
use std::future::Future;
use std::sync::Arc;
use std::ops::FnOnce;
use std::any::Any;
use async_trait::async_trait;
use futures;
use std::rc::Rc;
use im_rc::vector::Vector;

pub use self::bootstrap_error::BootstrapError;
mod bootstrap_error;

pub use self::registry::{Registry, Provided, AnyProvided, RegistryError};
mod registry;

#[async_trait]
pub trait Factory {
    type Module;
    async fn create(&self, registry: Arc<Mutex<Registry<'_>>>) -> Provided<Self::Module>;
}


#[async_trait]
trait AnyFactoryHolder<'a> {
    async fn create_module(&'a self, registry: Arc<Mutex<Registry<'a>>>);
    async fn do_run(&self);
}

struct FactoryHolder<F: Factory + Send + Sync> {
    factory: Rc<F>,
    sync_run: Barrier,
}

impl<F: Factory + Send + Sync + 'static> FactoryHolder<F> where F::Module: Any + Sync + Send {
    fn new(factory: Rc<F>) -> Rc<dyn AnyFactoryHolder<'static>> {
        Rc::new(Self {  
            factory: factory,
            sync_run: Barrier::new(2),
        })
    }

    async fn create_any_with_sync(&self, registry: Arc<Mutex<Registry<'_>>>) -> Provided<F::Module> {
        self.sync_run.wait().await;
        self.factory.create(registry).await
    }

}


#[async_trait]
impl<'a, F: Factory + Send + Sync + Any> AnyFactoryHolder<'a> for FactoryHolder<F> where F::Module: Any + Sync + Send {
    async fn do_run(&self) {
        self.sync_run.wait().await;
    }

    async fn create_module(&'a self, registry: Arc<Mutex<Registry<'a>>>) {
        let any_future_module: Pin<Box<dyn Future<Output = AnyProvided> + Send>>
            = self.create_any_with_sync(registry.clone())
                .boxed()
                .map(|result| {
                    result.map(|module| {
                        let any_module: Arc<(dyn std::any::Any + std::marker::Send + Sync + 'static)> = module; //module as Arc<dyn std::any::Any>;
                        any_module
                    })
                })
                .boxed();

        registry.lock().await.register::<F::Module>(any_future_module);
    }
}

unsafe impl<F: Factory + Send + Sync> Sync for FactoryHolder<F> {}
unsafe impl<F: Factory + Send + Sync> Send for FactoryHolder<F> {}

pub struct Bootstrapper<'a> {
    registry: Rc<Registry<'a>>,
    module_factories: Vector<Rc<dyn AnyFactoryHolder<'a>>>,
}


impl<'a> Bootstrapper<'a> {
    pub fn new() -> Self {
        Bootstrapper{
            registry: Rc::new(Registry::new()),
            module_factories: Vector::new(),
        }
    }
    
    pub fn register<Module>(&self, bootstrappable: Rc<dyn Factory<Module = Module> + Send + Sync>) -> Self
        where Module: Send + Sync
    {
        // let module = bootstrappable.create(&self.registry);
        // self.module_factories.push(bootstrappable);

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
        let mut factories = self.module_factories.clone();
        let holder = FactoryHolder::new(bootstrappable);
        factories.push_back(holder);

        Self{
            registry: self.registry.clone(),
            module_factories: factories,
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

            async fn create(&self, _registry: Arc<Mutex<Registry<'_>>>) -> Provided<Self::Module> {
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


            async fn create(&self, registry: Arc<Mutex<Registry<'_>>>) -> Provided<Self::Module> {
                if let Some(provided) = registry.lock().await.provide::<First>().await {
                    Ok(
                        Arc::new(
                            Mutex::new(
                                Second{
                                    value: provided.unwrap().lock().await.value.to_string()
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
            .register(Rc::new(FirstFactory{}))
            .register(Rc::new(SecondFactory{}));

    }
}

