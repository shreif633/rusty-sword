#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum TargetType {
    Player = 0,
    Monster = 1,
}

impl From<u8> for TargetType {
    fn from(value: u8) -> Self {
        match value {
            0 => TargetType::Player,
            _ => TargetType::Monster,
        }
    }
}

impl From<TargetType> for u8 {
    fn from(value: TargetType) -> Self {
        match value {
            TargetType::Player => 0,
            TargetType::Monster => 1
        }
    }
}