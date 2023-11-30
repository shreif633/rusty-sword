use crate::framework::packet::Packet;

pub const HEADER: u8 = 255;
pub const SUB_HEADER: u32 = 248;

#[derive(Debug)]
pub enum Color {
    General = 16777215,
	Info = 15262534,
	Shutdown = 1012976,
	Orange = 4227327,
	Blue = 16744448,
	LightBlue = 16776960,
	Yellow = 8454143,
	Red = 255,
	Party = 16594,
	Guild = 15073034,
	Alliance = 12615808,
	Green = 65280,
	DarkGreen = 43520,
	Failed = 54010,
	ClassMate = 32768,
	Pink = 16751615,
}

#[derive(Debug)]
pub struct SystemMessage {
    pub message: String,
    pub color: Color,
}

impl From<&mut Packet> for SystemMessage {
    fn from(packet: &mut Packet) -> Self {
        let message = packet.get_string();
        let color = packet.get_u32();
        let color = match color {
            15262534 => Color::Info,
            1012976 => Color::Shutdown,
            4227327 => Color::Orange,
            16744448 => Color::Blue,
            16776960 => Color::LightBlue,
            8454143 => Color::Yellow,
            255 => Color::Red,
            16594 => Color::Party,
            15073034 => Color::Guild,
            12615808 => Color::Alliance,
            65280 => Color::Green,
            43520 => Color::DarkGreen,
            54010 => Color::Failed,
            32768 => Color::ClassMate,
            16751615 => Color::Pink,
            _ => Color::General,
        };
        SystemMessage { message, color }
    }
}

impl From<&SystemMessage> for Packet {
    fn from(val: &SystemMessage) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u32(SUB_HEADER);
        packet.write_string(&val.message);
        match val.color {
            Color::Info => packet.write_u32(15262534),
            Color::Shutdown => packet.write_u32(1012976),
            Color::Orange => packet.write_u32(4227327),
            Color::Blue => packet.write_u32(16744448),
            Color::LightBlue => packet.write_u32(16776960),
            Color::Yellow => packet.write_u32(8454143),
            Color::Red => packet.write_u32(255),
            Color::Party => packet.write_u32(16594),
            Color::Guild => packet.write_u32(15073034),
            Color::Alliance => packet.write_u32(12615808),
            Color::Green => packet.write_u32(65280),
            Color::DarkGreen => packet.write_u32(43520),
            Color::Failed => packet.write_u32(54010),
            Color::ClassMate => packet.write_u32(32768),
            Color::Pink => packet.write_u32(16751615),
            Color::General => packet.write_u32(16777215),
        };
        packet
    }
}