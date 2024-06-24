use tokio::time::sleep;
use std::time::Duration;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rusb::{Context, DeviceHandle, Result, UsbContext};

fn detach_device<T: UsbContext>(handle: &mut DeviceHandle<T>) -> Result<()> {
    if let Ok(active) = handle.kernel_driver_active(0) {
        if active {
            handle.detach_kernel_driver(0)?;
            println!("Kernel driver detached.");
        }
    }

    Ok(())
}
#[tokio::main]
async fn main() {
    loop {
        match scan_usb_devices() {
            Ok(handlers) => {
                if let Some(mut random_handler) = handlers.choose(&mut thread_rng()) {
                    match detach_device(&mut random_handler) {
                        Ok(_) => println!("Device disconnected"),
                        Err(e) => eprintln!("Failed disconnect device")
                    }
                }
            },
            Err(e) => eprintln!("Error: {}", e),
        }

        sleep(Duration::from_secs(10 * 60)).await.await;
    }
}

fn scan_usb_devices() -> Result<Vec<DeviceHandle<Context>>> {
    let context = Context::new()?;
    let mut handles = Vec::new();
    for device in context.devices()?.iter() {
        match device.open() {
            Ok(handle) => {
                handles.push(handle)
            }
            Err(e) => eprintln!("Failed to open device: {}", e),
        }
    }
    Ok(handles)
}
