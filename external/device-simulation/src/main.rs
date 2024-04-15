use device_sdk;
use log;
use tokio;

mod simulation_driver;

#[tokio::main]
async fn main() {
    log::info!("Startup Device Simulation");
    match device_sdk::bootstrap("simulation", "1.0.0", Box::new(simulation_driver::SimulationDriver{})) {
        Err(err) => log::error!("Device Service {}", err),
        Ok(notify_close) => {
            device_sdk::shutdown_signal().await;
            notify_close.notify_waiters();
        }
    }
    log::info!("Teardown Device Simulation");
}
