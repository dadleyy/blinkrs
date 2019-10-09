use super::constants::{FADE_COMMAND_ACTION, IMMEDIATE_COMMAND_ACTION};
use super::Color;
use std::time::Duration;

/// Represents a command processable by the specification outlined in the [blink1 docs](https://git.io/JenDr).
#[derive(Debug, Copy, Clone)]
pub enum Message {
    Off,
    Fade(Color, Duration),
    Immediate(Color),
}

impl Message {
    /// Returns the buffer that will be written to the blink(1) usb device based on the specification
    /// outlined in the [blink1 docs](https://git.io/JenDr).
    pub fn buffer(&self) -> [u8; 8] {
        match self {
            Message::Off => Message::Immediate(Color::Three(0x00, 0x00, 0x00)).buffer(),
            Message::Fade(color, duration) => {
                let (r, g, b) = color.rgb();
                // Divide by 10 and truncate into two parts
                let dms = duration.as_millis().checked_div(10).unwrap_or(0) as u16;
                let th = dms.checked_shr(8).unwrap_or(0) as u8;
                let tl = dms.checked_rem(0xff).unwrap_or(0) as u8;
                [0x01, FADE_COMMAND_ACTION, r, g, b, th, tl, 0x00]
            }
            Message::Immediate(color) => {
                let (r, g, b) = color.rgb();
                [0x01, IMMEDIATE_COMMAND_ACTION, r, g, b, 0x00, 0x00, 0x00]
            }
        }
    }
}

impl From<&str> for Message {
    fn from(input: &str) -> Self {
        Message::Immediate(Color::from(input))
    }
}

#[cfg(test)]
mod tests {
    use super::Message;

    #[test]
    fn test_red() {
        let red = Message::from("red");
        assert_eq!(red.buffer()[2..5], [0xff, 0x00, 0x00])
    }

    #[test]
    fn test_green() {
        let red = Message::from("green");
        assert_eq!(red.buffer()[2..5], [0x00, 0xff, 0x00])
    }

    #[test]
    fn test_blue() {
        let red = Message::from("blue");
        assert_eq!(red.buffer()[2..5], [0x00, 0x00, 0xff])
    }

    #[test]
    fn test_off() {
        let red = Message::from("off");
        assert_eq!(red.buffer()[2..5], [0x00, 0x00, 0x00])
    }
}
