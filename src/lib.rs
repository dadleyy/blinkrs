//! This crate provides a lightweight wrapper around the [libusb](https://github.com/dcuddeback/libusb-rs) crate
//! specifically targeting the API of a [blink(1)] usb device.
//!
//! ## Example
//!
//! ```rust
//! use std::boxed::Box;
//! use std::error::Error;
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
//!
//! [blink(1)]: https://blink1.thingm.com

use libusb::{request_type, Context, Device, DeviceHandle, Direction, Recipient, RequestType};
use std::fmt;
use std::time::Duration;

pub use color::Color;
use constants::{HID_FEATURE, HID_SET_REPORT, PRODUCT_ID, VENDOR_ID};
pub use error::BlinkError;
pub use message::Message;

mod color;
mod constants;
mod error;
mod message;

fn is_blinker(device: &Device) -> bool {
  if let Ok(desc) = device.device_descriptor() {
    return desc.num_configurations() > 0 && desc.product_id() == PRODUCT_ID && desc.vendor_id() == VENDOR_ID;
  }

  false
}

fn send(device: &Device, message: &Message) -> Result<usize, BlinkError> {
  let config = device.active_config_descriptor()?;
  let mut handle: DeviceHandle = device.open()?;
  let interface_num = config.interfaces().nth(0).ok_or(BlinkError::NotFound)?.number();

  if let Ok(active) = handle.kernel_driver_active(interface_num) {
    if active {
      handle.detach_kernel_driver(interface_num)?;
    }
  }

  let buffer = message.buffer();
  let time = Duration::new(0, 100);
  let r_type = request_type(Direction::Out, RequestType::Class, Recipient::Interface);
  let request_value: u16 = HID_FEATURE | (buffer[0] as u16);
  let out = handle.write_control(r_type, HID_SET_REPORT, request_value, 0x00, &buffer, time);
  out.map_err(|e| BlinkError::from(e))
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
  fn from_context(ctx: Context) -> Self {
    Blinkers { context: ctx }
  }

  pub fn new() -> Result<Self, BlinkError> {
    let context: Context = Context::new()?;
    Ok(Blinkers::from_context(context))
  }

  pub fn send(&self, cmd: Message) -> Result<usize, BlinkError> {
    let devices = self.context.devices()?;
    devices
      .iter()
      .filter(is_blinker)
      .map(|d| send(&d, &cmd))
      .collect::<Result<Vec<usize>, BlinkError>>()
      .map(|d| d.iter().sum())
  }

  pub fn device_count(&self) -> Result<usize, BlinkError> {
    let devices = self.context.devices()?;
    Ok(devices.iter().filter(is_blinker).count())
  }
}
