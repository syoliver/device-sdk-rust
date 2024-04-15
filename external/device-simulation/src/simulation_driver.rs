#![allow(unused_variables)]

use device_sdk::protocol::Driver;
use device_sdk::command::{Request as CommandRequest, Value as CommandValue};
use device_sdk::service::Service;
use device_sdk::protocol::Properties;
use device_sdk::device::{Device, AdminState};

use std::collections::HashMap;
use std::vec::Vec;
use std::error::Error;

pub struct SimulationDriver {

}

impl Driver for SimulationDriver {
	// Initialize performs protocol-specific initialization for the device service.
	// The given *AsyncValues channel can be used to push asynchronous events and
	// readings to Core Data. The given []DiscoveredDevice channel is used to send
	// discovered devices that will be filtered and added to Core Metadata asynchronously.
	fn initialize(&self, service: &Service) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

	// HandleReadCommands passes a slice of CommandRequest struct each representing
	// a ResourceOperation for a specific device resource.
	fn handle_read_commands(&self, device_name: &str, protocols: &HashMap<String, Properties>, reqs: Vec<CommandRequest>) -> Result<Vec<CommandValue>, Box<dyn Error>> {
        Ok(vec![])
    }

	// HandleWriteCommands passes a slice of CommandRequest struct each representing
	// a ResourceOperation for a specific device resource.
	// Since the commands are actuation commands, params provide parameters for the individual
	// command.
	fn handle_write_commands(&self, device_name: &str, protocols: &HashMap<String, Properties>, reqs: Vec<CommandRequest>, params: Vec<CommandValue>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

	// Stop instructs the protocol-specific DS code to shutdown gracefully, or
	// if the force parameter is 'true', immediately. The driver is responsible
	// for closing any in-use channels, including the channel used to send async
	// readings (if supported).
	fn stop(&self, force: bool) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

	// Start runs Device Service startup tasks after the SDK has been completely initialized.
	// This allows Device Service to safely use DeviceServiceSDK interface features in this function call.
	fn start(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

	// AddDevice is a callback function that is invoked
	// when a new Device associated with this Device Service is added
	fn add_device(&self, device_name: &str, protocols: &HashMap<String, Properties>, admin_state: AdminState) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

	// UpdateDevice is a callback function that is invoked
	// when a Device associated with this Device Service is updated
	fn update_device(&self, device_name: &str, protocols: &HashMap<String, Properties>, admin_state: AdminState) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

	// RemoveDevice is a callback function that is invoked
	// when a Device associated with this Device Service is removed
	fn remove_device(&self, device_name: &str, protocols: &HashMap<String, Properties>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

	// Discover triggers protocol specific device discovery, asynchronously
	// writes the results to the channel which is passed to the implementation
	// via ProtocolDriver.Initialize(). The results may be added to the device service
	// based on a set of acceptance criteria (i.e. Provision Watchers).
	fn discover(&self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

	// ValidateDevice triggers device's protocol properties validation, returns error
	// if validation failed and the incoming device will not be added into EdgeX.
	fn validate_device(&self, device: &Device) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}
