use super::constants::{FADE_COMMAND_ACTION, IMMEDIATE_COMMAND_ACTION};
use super::Color;
use std::time::Duration;

#[derive(Debug, Copy, Clone)]
pub enum Message {
    Off,
    Fade(Color, Duration),
    Immediate(Color),
}

impl Message {
    pub fn buffer(&self) -> [u8; 8] {
        match self {
            Message::Off => Message::Immediate(Color::Three(0x00, 0x00, 0x00)).buffer(),
            Message::Fade(color, _duration) => {
                let (r, g, b) = color.rgb();
                [0x01, FADE_COMMAND_ACTION, r, g, b, 0x00, 0x00, 0x00]
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
        match input {
            "red" => Message::Immediate(Color::Red),
            "blue" => Message::Immediate(Color::Blue),
            "green" => Message::Immediate(Color::Green),
            _ => Message::Off,
        }
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
