#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum DamageType {
    Miss = 0,
    Normal = 1,
    Critical = 3,
}

impl From<u8> for DamageType {
    fn from(value: u8) -> Self {
        match value {
            0 => DamageType::Miss,
            1 => DamageType::Normal,
            _ => DamageType::Critical,
        }
    }
}

impl From<DamageType> for u8 {
    fn from(value: DamageType) -> Self {
        match value {
            DamageType::Miss => 0,
            DamageType::Normal => 1,
            DamageType::Critical => 3,
        }
    }
}