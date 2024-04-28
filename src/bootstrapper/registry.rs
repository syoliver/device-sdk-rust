use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::error::Error;
use futures::{future::BoxFuture, Future, FutureExt};
use tokio::{runtime::Runtime, time::Duration};
use clone_cell::cell::Cell;
use std::sync::{Arc, Mutex};
use std::ops::Deref;
use std::fmt;

#[derive(Debug)]
enum RegistryError {
    Internal,
}


impl Error for RegistryError {
    fn description(&self) -> &str {
        "Error in boostrap registry"
    }

    fn cause(&self) -> Option<&dyn Error> {
        None
    }
}

impl fmt::Display for RegistryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Internal error")
    }
}
/*
impl From<Box<dyn std::error::Error + std::marker::Send + Sync>> for RegistryError {
    fn from(err: Box<dyn std::error::Error + std::marker::Send + Sync>) -> Self {
        let mut bootstrap_err = BootstrapError::new();
        bootstrap_err.append(err);
        bootstrap_err
    }
}*/


pub trait Factory<T: Any> {
    fn create(&self) -> BoxFuture<Result<Box<T>, Box<dyn Error>>>;
}

trait AnyFactoryHolder {
    fn create(&self, rt: &Runtime) -> Result<Arc<dyn Any + Send + Sync>, Arc<dyn Error>>;
}

struct FactoryHolder<T: Any + Send> {
    factory: Box<dyn Factory<T>>,
    value: Cell<Option<Result<Arc<Mutex<T>>, Arc<dyn Error>>>>,
}

impl<T: Any + Send> AnyFactoryHolder for FactoryHolder<T> {
    fn create(&self, rt: &Runtime) -> Result<Arc<dyn Any + Send + Sync>, Arc<dyn Error>> {
        if let Some(hold_any_value) = self.value.get() {
            hold_any_value.map(|v| -> Arc<dyn Any + Send + Sync> {Arc::new(v)})
        } else {
            let future_value = self.factory.create();
            let value = rt.block_on(future_value);
            match value {
                Ok(value) => {
                    self.value.replace(Some(Ok(Arc::new(Mutex::new(*value)))));
                },
                Err(err) => {
                    self.value.replace(Some(Err(Arc::from(err))));
                }
            }
            self.value
                .get()
                .unwrap()
                .map(|v| -> Arc<dyn Any + Send + Sync> {Arc::new(v)})
        }
    }
}

pub struct Registry {
    factory_holders: Mutex<HashMap<TypeId, Box<dyn AnyFactoryHolder>>>,
    rt: tokio::runtime::Runtime,
}

impl Registry {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        Ok(Registry{
            factory_holders: HashMap::new().into(),
            rt: rt,
        })
    }

    pub fn register<T: Any + Send>(&mut self, factory: Box<dyn Factory<T>>) -> Result<(), Box<dyn Error>> {
        match self.factory_holders.lock() {
            Ok(mut lock_factory_holders) => {
                lock_factory_holders.insert(TypeId::of::<T>(), Box::new(FactoryHolder::<T> {
                    factory,
                    value: Cell::new(None),
                }));
                Ok(())
            }
            Err(_) => Err(Box::new(RegistryError::Internal))
        }
    }

    pub async fn provide<T: Any + Send>(&self) -> Option<Result<Arc<Mutex<T>>, Arc<dyn Error>>>
    {
        match self.factory_holders.lock() {
            Ok(lock_factory_holders) => {
                if let Some(factory_holder) = lock_factory_holders.get(&TypeId::of::<T>()) {
                    match factory_holder.create(&self.rt) {
                        Ok(any_value) => {
                            if let Ok(value) = any_value.downcast::<Mutex<T>>() {
                                return Some(Ok(value.clone()));
                            }
                        },
                        Err(_) => {
                            return Some(Err(Arc::new(RegistryError::Internal)));
                        }
                    }
                }
                None
            },
            Err(_) => {
                return Some(Err(Arc::new(RegistryError::Internal)));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_registry() -> Result<(), Box<dyn Error>> {
        #[derive(Debug)]
        struct MyType;

        impl Factory<MyType> for MyType {
            fn create(&self) -> BoxFuture<Result<Box<MyType>, Box<dyn Error>>> {
                // Implement your actual creation logic here
                Box::pin(async { Ok(Box::new(MyType)) })
            }
        }

        let mut registry = Registry::new()?;
        
        registry.register(Box::new(MyType))?;
        
        if let Some(my_type) = registry.provide::<MyType>().await {
            println!("Got an instance of MyType: {:?}", my_type);
        }
        Ok(())
    }
}
