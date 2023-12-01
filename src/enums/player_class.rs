#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum PlayerClass {
    Knight = 0,
    Mage = 1,
    Archer = 2,
}

impl From<u8> for PlayerClass {
    fn from(value: u8) -> Self {
        match value {
            0 => PlayerClass::Knight,
            1 => PlayerClass::Mage,
            _ => PlayerClass::Archer,
        }
    }
}

impl From<PlayerClass> for u8 {
    fn from(value: PlayerClass) -> Self {
        match value {
            PlayerClass::Knight => 0,
            PlayerClass::Mage => 1,
            PlayerClass::Archer => 2,
        }
    }
}