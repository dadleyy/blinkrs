extern crate libusb;

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum BlinkError {
  NotFound,
  DeviceListError(libusb::Error),
}

impl From<libusb::Error> for BlinkError {
  fn from(error: libusb::Error) -> Self {
    BlinkError::DeviceListError(error)
  }
}

impl fmt::Display for BlinkError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{{ }}")
  }
}

impl Error for BlinkError {
  fn description(&self) -> &str {
    match self {
      BlinkError::NotFound => "not found",
      BlinkError::DeviceListError(_e) => "unable to find usb device",
    }
  }
}
