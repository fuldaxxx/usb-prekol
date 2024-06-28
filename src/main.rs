use std::thread;
use std::time::Duration;
use rand::seq::SliceRandom;
use rand::{Rng, thread_rng};
use rusb::{Context, DeviceHandle, Result, UsbContext};

fn detach_device<T: UsbContext>(handle: &DeviceHandle<T>) -> Result<()> {
    if let Ok(active) = handle.kernel_driver_active(0) {
        if active {
            handle.detach_kernel_driver(0)?;
            println!("Kernel driver detached.");
        }
    }

    Ok(())
}

fn main() {
    loop {
        match scan_usb_devices() {
            Ok(handlers) => {
                if let Some(random_handler) = handlers.choose(&mut thread_rng()) {
                    match detach_device(&random_handler) {
                        Ok(_) => println!("Device disconnected"),
                        Err(e) => eprintln!("Failed disconnect device: {}", e)
                    }
                }
            },
            Err(e) => eprintln!("Error: {}", e),
        }

        thread::sleep(Duration::from_secs(thread_rng().gen_range(10..=600)));
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
