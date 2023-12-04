use crate::{framework::packet::Packet, components::{base_points::BasePoints, current_health_points::CurrentHealthPoints, player::Player, maximum_health_points::{self, MaximumHealthPoints}, current_magic_points::{self, CurrentMagicPoints}, maximum_magic_points::{self, MaximumMagicPoints}, final_points::FinalPoints, experience::{self, Experience}, physical_attack::PhysicalAttack, magical_attack::{self, MagicalAttack}, rage::Rage}};

pub const HEADER: u8 = 66;

#[derive(Debug)]
pub struct PlayerInformationResponse {
    pub specialization: u8, 
    pub unknown1: Vec<u8>,
    pub contribution: u16, 
    pub base_strength: u16, 
    pub base_health: u16, 
    pub base_intelligence: u16, 
    pub base_wisdom: u16, 
    pub base_agility: u16, 
    pub current_health_points: u32,
    pub maximum_health_points: u32, 
    pub current_magic_points: u16, 
    pub maximum_magic_points: u16, 
    pub on_target_point: u16, 
    pub evasion: u16,
    pub defense: u16, 
    pub absorption: u16,
    pub experience: u32,
    pub unknown2: Vec<u8>,
    pub minimum_physical_attack: u16,
    pub maximum_physical_attack: u16,
    pub minimum_magical_attack: u16,
    pub maximum_magical_attack: u16,
    pub status_points: u16, 
    pub skill_points: u16, 
    pub fire_resistence: u8, 
    pub ice_resistence: u8, 
    pub lighning_resistence: u8,
    pub curse_resistence: u8, 
    pub non_elemental_resistence: u8,
    pub rage: u32,
}

impl PlayerInformationResponse {
    pub fn new(
        player: &Player, 
        base_points: &BasePoints, 
        current_health_points: &CurrentHealthPoints, 
        maximum_health_points: &MaximumHealthPoints, 
        current_magic_points: &CurrentMagicPoints, 
        maximum_magic_points: &MaximumMagicPoints, 
        final_points: &FinalPoints,
        experience: &Experience,
        physical_attack: &PhysicalAttack,
        magical_attack: &MagicalAttack,
        rage: &Rage
    ) -> Self {
        PlayerInformationResponse { 
            specialization: player.specialty, 
            unknown1: vec![0, 0], 
            contribution: 9, 
            base_strength: base_points.base_strength, 
            base_health: base_points.base_health, 
            base_intelligence: base_points.base_intelligence, 
            base_wisdom: base_points.base_wisdom, 
            base_agility: base_points.base_agility, 
            current_health_points: current_health_points.current_health_points, 
            maximum_health_points: maximum_health_points.maximum_health_points, 
            current_magic_points: current_magic_points.current_magic_points, 
            maximum_magic_points: maximum_magic_points.maximum_magic_points, 
            on_target_point: final_points.on_target_point, 
            evasion: final_points.evasion, 
            defense: final_points.defense, 
            absorption: final_points.absorption, 
            experience: experience.experience,
            unknown2: vec![0, 0, 0], 
            minimum_physical_attack: physical_attack.minimum_physical_attack, 
            maximum_physical_attack: physical_attack.maximum_physical_attack, 
            minimum_magical_attack: magical_attack.minimum_magical_attack, 
            maximum_magical_attack: magical_attack.maximum_magical_attack, 
            status_points: 20, 
            skill_points: 44, 
            fire_resistence: final_points.fire_resistence.try_into().unwrap(), 
            ice_resistence: final_points.ice_resistence.try_into().unwrap(), 
            lighning_resistence: final_points.lighning_resistence.try_into().unwrap(), 
            curse_resistence: final_points.curse_resistence.try_into().unwrap(), 
            non_elemental_resistence: final_points.non_elemental_resistence.try_into().unwrap(), 
            rage: rage.rage 
        }
    }
}

impl From<&mut Packet> for PlayerInformationResponse {
    fn from(packet: &mut Packet) -> Self {
        let specialization = packet.get_u8();
        let unknown1 = packet.get_buffer(2);
        let contribution = packet.get_u16(); 
        let base_strength = packet.get_u16();  
        let base_health = packet.get_u16();  
        let base_intelligence = packet.get_u16();  
        let base_wisdom = packet.get_u16(); 
        let base_agility = packet.get_u16();  
        let current_health_points = packet.get_u32(); 
        let maximum_health_points = packet.get_u32();  
        let current_magic_points = packet.get_u16();  
        let maximum_magic_points = packet.get_u16();  
        let on_target_point = packet.get_u16();  
        let evasion = packet.get_u16(); 
        let defense = packet.get_u16(); 
        let absorption = packet.get_u16(); 
        let experience = packet.get_u32();
        let unknown2 = packet.get_buffer(3);
        let minimum_physical_attack = packet.get_u16(); 
        let maximum_physical_attack = packet.get_u16(); 
        let minimum_magical_attack = packet.get_u16(); 
        let maximum_magical_attack = packet.get_u16(); 
        let status_points = packet.get_u16(); 
        let skill_points = packet.get_u16();  
        let fire_resistence = packet.get_u8();  
        let ice_resistence = packet.get_u8();  
        let lighning_resistence = packet.get_u8(); 
        let curse_resistence = packet.get_u8();  
        let non_elemental_resistence = packet.get_u8(); 
        let rage = packet.get_u32();
        PlayerInformationResponse { 
            unknown1, contribution, base_strength, base_health, base_intelligence, base_wisdom, base_agility, 
            current_health_points, maximum_health_points, current_magic_points, maximum_magic_points, on_target_point, 
            evasion, defense, experience, unknown2, status_points, skill_points, fire_resistence, 
            ice_resistence, lighning_resistence, curse_resistence, non_elemental_resistence,
            absorption,
            minimum_physical_attack,
            maximum_physical_attack,
            minimum_magical_attack,
            maximum_magical_attack,
            specialization,
            rage,  
        }
    }
}

impl From<&PlayerInformationResponse> for Packet {
    fn from(val: &PlayerInformationResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u8(val.specialization);
        packet.write_buffer(&val.unknown1);
        packet.write_u16(val.contribution); 
        packet.write_u16(val.base_strength);
        packet.write_u16(val.base_health);
        packet.write_u16(val.base_intelligence); 
        packet.write_u16(val.base_wisdom); 
        packet.write_u16(val.base_agility);
        packet.write_u32(val.current_health_points);
        packet.write_u32(val.maximum_health_points); 
        packet.write_u16(val.current_magic_points); 
        packet.write_u16(val.maximum_magic_points);
        packet.write_u16(val.on_target_point);
        packet.write_u16(val.evasion);
        packet.write_u16(val.defense);
        packet.write_u16(val.absorption);
        packet.write_u32(val.experience);
        packet.write_buffer(&val.unknown2);
        packet.write_u16(val.minimum_physical_attack);
        packet.write_u16(val.maximum_physical_attack);
        packet.write_u16(val.minimum_magical_attack);
        packet.write_u16(val.maximum_magical_attack);
        packet.write_u16(val.status_points);
        packet.write_u16(val.skill_points);
        packet.write_u8(val.fire_resistence);
        packet.write_u8(val.ice_resistence);
        packet.write_u8(val.lighning_resistence);
        packet.write_u8(val.curse_resistence); 
        packet.write_u8(val.non_elemental_resistence);
        packet.write_u32(val.rage);
        packet
    }
}