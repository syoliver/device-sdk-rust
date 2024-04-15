#![allow(dead_code)]

use std::collections::HashMap;
use std::any::Any;

pub struct Request {
    resource_name: String,
    attributes: HashMap<String, Box<dyn Any>>,
    request_type: String
}
