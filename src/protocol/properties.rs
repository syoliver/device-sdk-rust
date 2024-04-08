use std::collections::HashMap;
use std::any::Any;

pub type Properties = HashMap<String, Box<dyn Any>>;

