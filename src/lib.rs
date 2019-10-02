//! This crate provides a lightweight wrapped around the [libusb](https://github.com/dcuddeback/libusb-rs) crate
//! specifically targeting the API of a [blink(1)](https://blink1.thingm.com) usb device.
//!
//! ## Example
//!
//! ```
//! extern crate libusb;
//!
//! use std::boxed::Box;
//! use std::error::Error;
//! use std::io::stdin;
//!
//! use blinkrs::{Blinkers, Message};
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     let blinkers: Blinkers = match Blinkers::new() {
//!         Ok(b) => b,
//!         Err(_e) => {
//!             println!("unable to find device");
//!             return Ok(())
//!         },
//!     };
//!     blinkers.send(Message::from("red"))?;
//!     blinkers.send(Message::from("off"))?;
//!     Ok(())
//! }
//! ```

use libusb::{Context, Device, DeviceHandle};
use std::fmt;
use std::time::Duration;

pub use color::Color;
use constants::{COLOR_CONTROL, PRODUCT_ID, VENDOR_ID};
pub use error::BlinkError;
pub use message::Message;

mod color;
mod constants;
mod error;
mod message;

fn is_blinker(device: &Device) -> bool {
    if let Ok(desc) = device.device_descriptor() {
        return desc.product_id() == PRODUCT_ID && desc.vendor_id() == VENDOR_ID;
    }

    false
}

fn send(handle: &DeviceHandle, message: &Message) -> Result<usize, BlinkError> {
    let buffer = message.buffer();
    let time = Duration::new(0, 100);
    let (request_type, request, request_value) = COLOR_CONTROL;
    let size = handle.write_control(request_type, request, request_value, 0x00, &buffer, time)?;
    Ok(size)
}

/// Wraps the [`libusb::Context`](https://docs.rs/libusb/0.3.0/libusb/struct.Context.html) type.
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
