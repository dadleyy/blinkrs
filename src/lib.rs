use libusb::{Context, Device, DeviceHandle};
use std::fmt;
use std::time::Duration;

pub mod color;
mod constants;
pub mod error;
pub mod message;

use color::Color;
use constants::{COLOR_CONTROL, PRODUCT_ID, VENDOR_ID};
use error::BlinkError;
use message::Message;

fn is_blinker(device: &Device) -> bool {
    if let Ok(desc) = device.device_descriptor() {
        return desc.product_id() == PRODUCT_ID && desc.vendor_id() == VENDOR_ID;
    }

    false
}

fn send(handle: &DeviceHandle, message: &Message) -> Result<usize, BlinkError> {
    let buffer: &[u8] = &message.buffer();
    let time = Duration::new(0, 100);
    let (request_type, request, request_value) = COLOR_CONTROL;
    let size = handle.write_control(request_type, request, request_value, 0x00, buffer, time)?;
    Ok(size)
}

pub struct Blinkers {
    context: Context,
}

impl fmt::Debug for Blinkers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Blinkers {{ }}")
    }
}

impl Blinkers {
    pub fn new() -> Result<Self, BlinkError> {
        let context: Context = Context::new()?;
        if let Ok(d) = context.devices() {
            if d.iter().filter(|d| is_blinker(d)).count() == 0 {
                return Err(BlinkError::NotFound);
            }
        }
        Ok(Blinkers { context })
    }

    pub fn send(&self, cmd: Message) -> Result<usize, BlinkError> {
        let devices = self.context.devices()?;
        let blinkers = devices
            .iter()
            .filter(|d| is_blinker(d))
            .map(|d| d.open())
            .flatten()
            .map(|d| send(&d, &cmd))
            .flatten()
            .collect::<Vec<usize>>();
        Ok(blinkers.len())
    }

    pub fn device_count(&self) -> Result<usize, BlinkError> {
        let devices = self.context.devices()?;
        Ok(devices.iter().filter(|d| is_blinker(d)).count())
    }
}
