#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    Three(u8, u8, u8),
    Red,
    Green,
    Blue,
}

impl Color {
    pub fn rgb(self) -> (u8, u8, u8) {
        match self {
            Color::Three(red, green, blue) => (red, green, blue),
            Color::Red => (0xff, 0x00, 0x00),
            Color::Green => (0x00, 0xff, 0x00),
            Color::Blue => (0x00, 0x00, 0xff),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Color::{Three,Red,Blue,Green};

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
