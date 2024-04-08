use std::collections::HashMap;
use std::vec::Vec;
use std::any::Any;
use crate::protocol::Properties as ProtocolProperties;
use crate::device::{AdminState, OperatingState};

pub struct Device {
	id: String,
	name: String,
	description: String,
	admin_state:     AdminState,
	operating_state: OperatingState,
	protocols:      HashMap<String, ProtocolProperties>,
	labels:        Vec<String>,
	location:      Box<dyn Any>,
	service_name:    String,
	profile_name:    String,
	// AutoEvents     []AutoEvent
	tags:          HashMap<String, Box<dyn Any>>,
	properties:    HashMap<String, Box<dyn Any>>,
}