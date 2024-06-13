use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::error::Error;
use futures::{FutureExt, future::{BoxFuture, Shared}};
use tokio::{runtime::{Handle, Runtime}, time::{sleep, Duration}, sync::Mutex};
use tokio;
use clone_cell::cell::Cell;
use std::ops::Deref;
use std::fmt;
use std::sync::Arc;
use std::pin::Pin;
use std::future::Future;

#[derive(Debug, Clone)]
pub enum RegistryError {
    CastFailure,
    Error(String),
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
        match self {
            RegistryError::CastFailure => {
                write!(f, "Cast failure")
            },
            RegistryError::Error(err) => {
                write!(f, "Failure {}", err)
            }
        }
    }
}

pub type Provided<Module> = Result<Arc<Mutex<Module>>, RegistryError>;
pub type AnyProvided = Result<Arc<dyn Any + Send + Sync>, RegistryError>;
type AnyFuture<'a> = Shared<BoxFuture<'a, AnyProvided>>;
pub struct Registry<'a> {
    modules: HashMap<TypeId, AnyFuture<'a>>,
}

/*
struct LazyFuture {
    upstream: Box<Future<Output = Box<Any>>>,
    value: Option<Box<Any>>,
    mutex: Mutex<()>,
}

impl Future for LazyFuture {
    type Output;

    fn poll( self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let lock = self.mutex.lock();
        if let Some(value) = self.value {
            Poll::Ready(value)
        } else {
            self.upstream.poll(cx)
        }
    }
}
*/
impl<'a> Registry<'a> {
    pub fn new() -> Self {
        Registry{
            modules: HashMap::new(),
        }
    }

    pub fn register<T: Any + Send + Sync>(&mut self, module: BoxFuture<'a, AnyProvided>) -> Result<(), Box<dyn Error>> {
        self.modules.insert(TypeId::of::<T>(), module.shared());
        Ok(())
    }

    fn provide_any<T: 'static>(&self) -> Option<AnyFuture<'a>> {
        let type_id = TypeId::of::<T>();
        if let Some(future_any_module) = self.modules.get(&type_id) {
            Some(future_any_module.clone())
        } else {
            None
        }
    }

    pub async fn provide<T: Send + 'static>(&self) -> Option<Provided<T>> {
        if let Some(any_result_module) = self.provide_any::<T>() {
            match any_result_module.await {
                Ok(any_module) =>  {
                    if let Ok(module) = any_module.downcast::<Mutex<T>>() {
                        Some(Ok(module))
                    } else {
                        Some(Err(RegistryError::CastFailure))
                    }
                },
                Err(err) => {
                    Some(Err(err))
                }
            }
        } else {
            None
        }
    }
}

unsafe impl Send for Registry<'_> {}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[tokio::test]
    async fn test_registry() {
        let registry = Arc::new(Mutex::new(Registry::new()));
        let mut future_any_value: Option<AnyFuture<'static>> = None;
        {
            let mut registry_lock = registry.lock().await;
            async fn first_closure() -> Result<Arc<dyn Any + Send + Sync>, RegistryError> {
                sleep(Duration::from_millis(1000)).await;
                Ok(Arc::new(Mutex::new(42)))
            }

            async fn second_closure(registry: Arc<Mutex<Registry<'_>>>) -> Result<Arc<dyn Any + Send + Sync>, RegistryError> {
                let future_any_value: Shared<Pin<Box<dyn Future<Output = Result<Arc<dyn Any + Sync + Send>, RegistryError>> + Send>>> = registry.lock().await.provide_any::<i32>().unwrap();
                if let Ok(any_value) = future_any_value.await {
                    if let Ok(value) = any_value.downcast::<Mutex<i32>>() {
                        Ok(Arc::new(Mutex::new(value.lock().await.to_string())))
                    } else {
                        Err(RegistryError::CastFailure)
                    }
                    
                } else {
                    Err(RegistryError::Error("Got an error".to_owned()))
                }
            }

            let first_future = first_closure().boxed();
            let second_future = second_closure(registry.clone()).boxed();

            if let Err(err) = registry_lock.register::<i32>(first_future) {
                assert!(false, "Error during i32 register");
            }

            if let Err(err) = registry_lock.register::<String>(second_future) {
                assert!(false, "Error during String register");
            }

            future_any_value = registry_lock.provide_any::<String>();
        }
        
        if let Ok(any_value) = future_any_value.unwrap().await {
            if let Ok(value) = any_value.downcast::<Mutex<String>>() {
                assert_eq!(*value.lock().await.deref(), "42".to_string());
            } else {
                assert!(false, "Error during String evaluation");
            }
        } else {
            assert!(false, "Type failure in String module");
        }
    }
}
