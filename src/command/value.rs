use std::collections::HashMap;
use std::any::Any;

pub struct Value {
    resource_name: String,
    value_type: String,
    value: Box<dyn Any>,
    origin: i64,
    tags: HashMap<String, String>
}
