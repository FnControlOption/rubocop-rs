#[repr(u8)]
pub enum Color {
    Black = 30,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack = 90,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl Color {
    pub fn fg_code(self) -> u8 {
        self as u8
    }

    pub fn bg_code(self) -> u8 {
        self as u8 + 10
    }
}
