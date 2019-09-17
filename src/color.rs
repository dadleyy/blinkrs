pub enum Color {
    Three(u8, u8, u8),
    Red,
    Green,
    Blue,
}

impl Color {
    pub fn rgb(&self) -> (u8, u8, u8) {
        match self {
            Color::Three(red, green, blue) => (*red, *green, *blue),
            Color::Red => (0xff, 0x00, 0x00),
            Color::Green => (0x00, 0xff, 0x00),
            Color::Blue => (0x00, 0x00, 0xff),
        }
    }
}
