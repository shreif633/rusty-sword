use crate::components::id::Id;
use crate::components::maximum_health_points::MaximumHealthPoints;
use crate::components::current_health_points::CurrentHealthPoints;
use crate::components::position::Position;
use crate::components::monster::Monster;
use crate::framework::packet::Packet;

pub const HEADER: u8 = 51;

#[derive(Debug)]
pub struct MonsterAppearResponse {
    pub index: u16,
    pub id: i32,
    pub x: u32,
    pub y: u32,
    pub direction: u16,
    pub current_hp: u32,
    pub maximum_hp: u32,
    pub general_state: i64,
    pub magical_state: i64,
    pub guild_name: String,
    pub race: u8,
    pub guild_id: i32,
    pub name: String,
    pub unknown1: i64,
    pub unknown2: i64,
    pub unknown3: u8,
    pub channel: u32,
}

impl MonsterAppearResponse {
    pub fn new(id: &Id, monster: &Monster, position: &Position, current_health_points: &CurrentHealthPoints, maximum_health_points: &MaximumHealthPoints) -> Self {
        MonsterAppearResponse {
            index: monster.index,
            id: id.id,
            x: position.x,
            y: position.y,
            direction: 0,
            current_hp: current_health_points.current_health_points,
            maximum_hp: maximum_health_points.maximum_health_points,
            general_state: 0,
            magical_state: 0,
            guild_name: "".to_string(),
            race: 0,
            guild_id: 0,
            name: "".to_string(),
            unknown1: 0,
            unknown2: 0,
            unknown3: 0,
            channel: 0,
        }
    }
}

impl From<&mut Packet> for MonsterAppearResponse {
    fn from(packet: &mut Packet) -> Self {
        let index = packet.get_u16();
        let id = packet.get_i32();
        let x = packet.get_u32();
        let y = packet.get_u32();
        let direction = packet.get_u16();
        let current_hp = packet.get_u32();
        let maximum_hp = packet.get_u32();
        let g_state = packet.get_i64();
        let m_state = packet.get_i64();
        let guild_name = packet.get_string();
        let race = packet.get_u8();
        let guild_id = packet.get_i32();
        let name = packet.get_string();
        let unknown1 = packet.get_i64();
        let unknown2 = packet.get_i64();
        let unknown3 = packet.get_u8();
        let channel = packet.get_u32();
        MonsterAppearResponse { 
            index, id, x, y, direction, current_hp, 
            maximum_hp, general_state: g_state, magical_state: m_state, guild_name,
            race, guild_id, name, unknown1, unknown2, unknown3, channel 
        }
    }
}

impl From<&MonsterAppearResponse> for Packet {
    fn from(val: &MonsterAppearResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u16(val.index);
        packet.write_i32(val.id);
        packet.write_u32(val.x);
        packet.write_u32(val.y);
        packet.write_u16(val.direction);
        packet.write_u32(val.current_hp);
        packet.write_u32(val.maximum_hp);
        packet.write_i64(val.general_state);
        packet.write_i64(val.magical_state);
        packet.write_string(&val.guild_name);
        packet.write_u8(val.race);
        packet.write_i32(val.guild_id);
        packet.write_string(&val.name);
        packet.write_i64(val.unknown1);
        packet.write_i64(val.unknown2);
        packet.write_u8(val.unknown3);
        packet.write_u32(val.channel);
        packet
    }
}