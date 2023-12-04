use bevy::prelude::*;
use crate::components::maximum_health_points::MaximumHealthPoints;
use crate::components::current_health_points::CurrentHealthPoints;
use crate::components::position::Position;
use crate::components::monster::Monster;
use crate::framework::packet::Packet;

pub const HEADER: u8 = 51;

#[derive(Debug)]
pub struct MonsterAppearResponse {
    pub monster_index: u16,
    pub monster_id: u32,
    pub monster_x: u32,
    pub monster_y: u32,
    pub monster_direction: u16,
    pub monster_current_hp: u32,
    pub monster_maximum_hp: u32,
    pub monster_g_state: i64,
    pub monster_m_state: i64,
    pub monster_guild_name: String,
    pub monster_race: u8,
    pub monster_guild_id: u32,
    pub monster_name: String,
    pub unknown1: i64,
    pub unknown2: i64,
    pub unknown3: u8,
    pub channel: u32,
}

impl MonsterAppearResponse {
    pub fn new(entity: Entity, monster: &Monster, position: &Position, current_health_points: &CurrentHealthPoints, maximum_health_points: &MaximumHealthPoints) -> Self {
        MonsterAppearResponse {
            monster_index: monster.index,
            monster_id: entity.index(),
            monster_x: position.x,
            monster_y: position.y,
            monster_direction: 0,
            monster_current_hp: current_health_points.current_health_points,
            monster_maximum_hp: maximum_health_points.maximum_health_points,
            monster_g_state: 0,
            monster_m_state: 0,
            monster_guild_name: "".to_string(),
            monster_race: 0,
            monster_guild_id: 0,
            monster_name: "".to_string(),
            unknown1: 0,
            unknown2: 0,
            unknown3: 0,
            channel: 0,
        }
    }
}

impl From<&mut Packet> for MonsterAppearResponse {
    fn from(packet: &mut Packet) -> Self {
        let monster_index = packet.get_u16();
        let monster_id = packet.get_u32();
        let monster_x = packet.get_u32();
        let monster_y = packet.get_u32();
        let monster_direction = packet.get_u16();
        let monster_current_hp = packet.get_u32();
        let monster_maximum_hp = packet.get_u32();
        let monster_g_state = packet.get_i64();
        let monster_m_state = packet.get_i64();
        let monster_guild_name = packet.get_string();
        let monster_race = packet.get_u8();
        let monster_guild_id = packet.get_u32();
        let monster_name = packet.get_string();
        let unknown1 = packet.get_i64();
        let unknown2 = packet.get_i64();
        let unknown3 = packet.get_u8();
        let channel = packet.get_u32();
        MonsterAppearResponse { 
            monster_index, monster_id, monster_x, monster_y, monster_direction, monster_current_hp, 
            monster_maximum_hp, monster_g_state, monster_m_state, monster_guild_name,
            monster_race, monster_guild_id, monster_name, unknown1, unknown2, unknown3, channel 
        }
    }
}

impl From<&MonsterAppearResponse> for Packet {
    fn from(val: &MonsterAppearResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u16(val.monster_index);
        packet.write_u32(val.monster_id);
        packet.write_u32(val.monster_x);
        packet.write_u32(val.monster_y);
        packet.write_u16(val.monster_direction);
        packet.write_u32(val.monster_current_hp);
        packet.write_u32(val.monster_maximum_hp);
        packet.write_i64(val.monster_g_state);
        packet.write_i64(val.monster_m_state);
        packet.write_string(&val.monster_guild_name);
        packet.write_u8(val.monster_race);
        packet.write_u32(val.monster_guild_id);
        packet.write_string(&val.monster_name);
        packet.write_i64(val.unknown1);
        packet.write_i64(val.unknown2);
        packet.write_u8(val.unknown3);
        packet.write_u32(val.channel);
        packet
    }
}