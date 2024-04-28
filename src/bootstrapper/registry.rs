use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::error::Error;
use futures::{FutureExt, future::{BoxFuture, Shared}};
use tokio::{runtime::{Handle, Runtime}, time::{sleep, Duration}};
use tokio;
use clone_cell::cell::Cell;
use std::ops::Deref;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::pin::Pin;
use std::future::Future;

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

type AnyFuture<'a> = Shared<BoxFuture<'a, Result<Arc<dyn Any + Send + Sync>, Box<dyn Error>>>>;

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

    pub fn register<T: Any + Send + Sync>(&mut self, module: BoxFuture<'a, Result<Arc<dyn Any + Send + Sync>, Box<dyn Error>>>) -> Result<(), Box<dyn Error>> {
        self.modules.insert(TypeId::of::<T>(), module.shared());
        Ok(())
    }

    pub async fn provide<T: 'static>(&self) -> Option<Result<Arc<Mutex<T>>, Box<dyn Error>>> {
        let type_id = TypeId::of::<T>();
        if let Some(future_any_module) = self.modules.get(&type_id) {
            let any_module = future_any_module.clone().await;
            if let Some(module) = any_module.downcast_ref::<Arc<Mutex<T>>>() {
                Some(Ok(module.clone()))
            } else {
                Some(Err(Box::new(RegistryError::Internal)))
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    enum ProviderError {
        Test,
    }
    
    impl Error for ProviderError {
        fn description(&self) -> &str {
            "Error in test provider"
        }
    
        fn cause(&self) -> Option<&dyn Error> {
            None
        }
    }
    
    impl fmt::Display for ProviderError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Internal error")
        }
    }

    
    #[tokio::test]
    async fn test_registry() {
        let mut registry = Arc::new(Registry::new());

        async fn first_closure() -> Result<Arc<dyn Any + Send + Sync>, Box<dyn Error>> {
            sleep(Duration::from_millis(1000)).await;
            Ok(Arc::new(42))
        }

        async fn second_closure(registry: Arc<Registry<'_>>) -> Result<Arc<dyn Any + Send + Sync>, Box<dyn Error>> {
            if let Some(Ok(value)) = registry.provide::<i32>().await {
                Ok(Arc::new(value.lock().unwrap().to_string()))
            } else {
                Err(Box::new(ProviderError::Test))
            }
        }

        let first_future = first_closure().boxed();
        let second_future = second_closure(registry.clone()).boxed();

        if let Err(err) = registry.register::<i32>(first_future) {
            assert!(false, "Error during i32 register");
        }
        
        if let Err(err) = registry.register::<String>(second_future) {
            assert!(false, "Error during String register");
        }
        
        if let Some(provided) = registry.provide::<String>().await {
            if let Ok(value) = provided {
                assert_eq!(value, "42".to_string().into());
            } else {
                assert!(false, "Error during String cast");
            }
        } else {
            assert!(false, "Error retrieving String value");
        }
    }
}
