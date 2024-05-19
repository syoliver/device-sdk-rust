#![allow(unused_imports)]

use std::error::Error;
use tokio::{task::{JoinSet, AbortHandle}, runtime::{Handle, Runtime}, time::{sleep, Duration}, sync::{Mutex, Barrier}};
use std::pin::Pin;
use futures::{FutureExt, TryFutureExt, future::{self, BoxFuture}};
use std::future::Future;
use std::sync::Arc;
use std::cell::RefCell;
use std::ops::FnOnce;
use std::any::Any;
use async_trait::async_trait;
use futures;
use std::rc::Rc;
use im_rc::vector::Vector;
use std::fmt;
use std::panic;
use std::borrow::BorrowMut;
use std::borrow::Borrow;
use std::mem;

pub use self::registry::{Registry, Provided, AnyProvided, RegistryError};
mod registry;


#[derive(Debug, Clone)]
pub enum BootstrapError {
    RegistryError(String),
    InternalError(String),
}

impl Error for BootstrapError {
    fn description(&self) -> &str {
        "Error in boostrap process"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl fmt::Display for BootstrapError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BootstrapError::RegistryError(s) => {
                write!(f, "Registry Error: {}", s)
            },
            BootstrapError::InternalError(s) => {
                write!(f, "Internal Error: {}", s)
            },
        }
    }
}

#[async_trait]
pub trait Factory {
    type Module;
    async fn create(&self, registry: Arc<Mutex<Box<Registry<'_>>>>) -> Provided<Self::Module>;
}


trait AnyFactoryHolder<'a> {
    fn create_module(&'a self, registry: Arc<Mutex<Box<Registry<'a>>>>) -> BoxFuture<Result<(), BootstrapError>>;
    // async fn run_module()
}

struct FactoryHolder<F: Factory + ?Sized> {
    factory: Arc<Mutex<Box<F>>>,
}

impl<'a, F: Factory + ?Sized + Send +'static> FactoryHolder<F> where F::Module: Any + Sync + Send {
    fn new(factory: Box<F>) -> Box<dyn AnyFactoryHolder<'a>> {
        Box::new(Self {  
            factory: Arc::new(Mutex::new(factory)),
        })
    }

    async fn create_any_with_sync(&self, registry: Arc<Mutex<Box<Registry<'_>>>>) -> Provided<F::Module> {
        self.factory.lock().await.create(registry).await
    }

}


impl<'a, F: Factory + Any + ?Sized + Send> AnyFactoryHolder<'a> for FactoryHolder<F> where F::Module: Any + Sync + Send {
    fn create_module(&'a self, registry: Arc<Mutex<Box<Registry<'a>>>>) -> BoxFuture<Result<(), BootstrapError>> {
        Box::pin(async move {
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

            let mut lock_registry = registry.lock().await;
            if let Err(err) = lock_registry.register::<F::Module>(any_future_module) {
                return Err(BootstrapError::RegistryError(err.to_string()))
            }

            Ok(())
        })
    }
}

pub struct Bootstrapper<'a> {
    registry: Arc<Mutex<Box<Registry<'a>>>>,
    module_factories: Vector<Arc<Mutex<Rc<dyn AnyFactoryHolder<'a>>>>>,
}


impl<'a> Bootstrapper<'a> {
    pub fn new() -> Arc<Self> {
        Arc::new(Bootstrapper{
            registry: Arc::new(Mutex::new(Box::new(Registry::new()))),
            module_factories: Vector::new(),
        })
    }
    
    pub fn register<Module>(self : Arc<Self>, bootstrappable: Box<dyn Factory<Module = Module> + Send + Sync>) -> Arc<Self>
        where Module: Send + Sync + 'static
    {
        let mut factories = self.module_factories.clone();
        let holder = FactoryHolder::new(bootstrappable);
        factories.push_back(Arc::new(Mutex::new(holder.into())));

        

        Arc::new(Self{
            registry: self.registry.clone(),
            module_factories: factories,
        })
    }
}

impl Bootstrapper<'static> {
    pub async fn run(self: Arc<Self>) -> Result<(), BootstrapError> {
        let set: Rc<RefCell<JoinSet<Result<(), BootstrapError>>>> = Rc::new(RefCell::new(JoinSet::new()));

        let registry_clone: Arc<Mutex<Box<Registry<'static>>>> = self.registry.clone();
        let unsafe_module_factories: &'static Vector<Arc<Mutex<Rc<dyn AnyFactoryHolder<'static>>>>> = unsafe {
            std::mem::transmute(&self.module_factories)
        };

        let local = tokio::task::LocalSet::new();
        local.run_until(async move {
            let future_task_handles: Vec<_> = unsafe_module_factories
                    .iter()
                    .map(|factory|{

                        let set_clone = set.clone();
                        let registry_local_clone = registry_clone.clone();

                        tokio::task::spawn_local(async move {
                            let lock_factory = factory.lock().await.clone();
                            let ptr_lock_factory = lock_factory.as_ref() as *const dyn AnyFactoryHolder<'static>;
                            let future_module = unsafe {
                                (*ptr_lock_factory).create_module(registry_local_clone)
                            };
                            let unsafe_future: BoxFuture<'static, Result<(), BootstrapError>> = unsafe {
                                std::mem::transmute(future_module)
                            };
                            let mut set_clone_ref = set_clone.as_ref().borrow_mut();

                            Some(set_clone_ref.spawn(unsafe_future))
                        })
                    })
                    .collect();

            let task_handles = future::join_all(future_task_handles).await;

            let mut set_ref = set.as_ref().borrow_mut();
            while let Some(res) = set_ref.join_next().await {
                if let Err(err) = res {
                    task_handles.iter().for_each(|handle|{
                        if let Ok(Some(h)) = handle {
                            h.abort();
                        }
                    });
                    return Err(BootstrapError::InternalError("Internal error".to_string()));
                } else if let Ok(Err(err)) = res {
                    task_handles.iter().for_each(|handle|{
                        if let Ok(Some(h)) = handle {
                            h.abort();
                        };
                    });
                    return Err(err);
                } 
            }
            Ok(())
        }).await
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

            async fn create(&self, _registry: Arc<Mutex<Box<Registry<'_>>>>) -> Provided<Self::Module> {
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


            async fn create(&self, registry: Arc<Mutex<Box<Registry<'_>>>>) -> Provided<Self::Module> {
                let lock_registry = registry.lock().await;
                let provide_result = lock_registry.provide::<First>().await;
                if let Some(provided) = provide_result {
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

        Bootstrapper::new()
            .register(Box::new(FirstFactory{}))
            .register(Box::new(SecondFactory{}))
            .run().await;

    }
}

