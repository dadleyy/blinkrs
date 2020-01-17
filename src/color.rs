/// Represents a color mode that can be applied to the [`blink(1)`] LED light.
///
/// [`blink(1)`]: https://blink1.thingm.com
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
  Three(u8, u8, u8),
  Red,
  Green,
  Blue,
}

impl Color {
  /// Returns a three element tuple of bytes representing a value of the red, green and blue
  /// components of a color in the0-255 range.
  pub fn rgb(self) -> (u8, u8, u8) {
    match self {
      Color::Three(red, green, blue) => (red, green, blue),
      Color::Red => (0xff, 0x00, 0x00),
      Color::Green => (0x00, 0xff, 0x00),
      Color::Blue => (0x00, 0x00, 0xff),
    }
  }
}

impl From<&str> for Color {
  fn from(input: &str) -> Self {
    match input {
      "red" => Color::Red,
      "green" => Color::Green,
      "blue" => Color::Blue,
      _ => Color::Three(0x00, 0x00, 0x00),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::Color::{Blue, Green, Red, Three};

  #[test]
  fn test_red() {
    let r = Red;
    assert_eq!(r, Red);
    assert_eq!(r.rgb(), r.rgb());
    assert_eq!(r.rgb(), (0xff, 0x00, 0x00));
  }

  #[test]
  fn test_green() {
    let g = Green;
    assert_eq!(g, Green);
    assert_eq!(g.rgb(), g.rgb());
    assert_eq!(g.rgb(), (0x00, 0xff, 0x00));
  }

  #[test]
  fn test_blue() {
    let b = Blue;
    assert_eq!(b, Blue);
    assert_eq!(b.rgb(), b.rgb());
    assert_eq!(b.rgb(), (0x00, 0x00, 0xff));
  }

  #[test]
  fn test_three() {
    let c = Three(0x11, 0xaa, 0xee);
    assert_eq!(c, Three(0x11, 0xaa, 0xee));
    assert_eq!(c.rgb(), c.rgb());
    assert_eq!(c.rgb(), (0x11, 0xaa, 0xee));
  }
}
