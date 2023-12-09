#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum LevelColor {
    Gray,
    Blue,
    Green,
    Yellow,
    Orange,
    Red,
    Purple
}

impl LevelColor {
    pub fn experience_rate(&self) -> u16 {
        match self {
            LevelColor::Gray => 0,
            LevelColor::Blue => 50,
            LevelColor::Green => 75,
            LevelColor::Yellow => 100,
            LevelColor::Orange => 125,
            LevelColor::Red => 150,
            LevelColor::Purple => 200
        }
    }
}

impl From<u8> for LevelColor {
    fn from(value: u8) -> Self {
        match value {
            1 => LevelColor::Blue,
            2 => LevelColor::Green,
            3 => LevelColor::Yellow,
            4 => LevelColor::Orange,
            5 => LevelColor::Red,
            6 => LevelColor::Purple,
            _ => LevelColor::Gray,
        }
    }
}

impl From<LevelColor> for u8 {
    fn from(value: LevelColor) -> Self {
        match value {
            LevelColor::Gray => 0,
            LevelColor::Blue => 1,
            LevelColor::Green => 2,
            LevelColor::Yellow => 3,
            LevelColor::Orange => 4,
            LevelColor::Red => 5,
            LevelColor::Purple => 6,
        }
    }
}