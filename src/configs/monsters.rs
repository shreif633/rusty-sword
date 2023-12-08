use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Resource)]
pub struct MonstersConfig {
    pub config: HashMap<u16, MonsterConfig>
} 

#[derive(Debug, Deserialize)]
pub struct MonsterConfig {
    pub monster: Monster,
    pub drop: Vec<Drop>,
    pub spawn: Vec<Spawn>,
}

#[derive(Debug, Deserialize)]
pub struct Monster {
    pub index: u16,
    pub name: String,
    pub level: u8,
    pub range: u16,
    pub physical_attack: u16,
    pub defense: u16,
    pub agressive: bool,
    pub sight: u16,
    pub experience: u32,
    pub beheadable: bool,
}

#[derive(Debug, Deserialize)]
pub struct Drop {
    pub index: u16,
    pub prefix: u8,
    pub chance: f32,
    pub quest: u16,
}

#[derive(Debug, Deserialize)]
pub struct Spawn {
    pub bottom_x: u32, 
    pub bottom_y: u32, 
    pub top_x: u32, 
    pub top_y: u32,
    pub quantity: u32,  
}

pub fn read_config(monster_name: &str) -> MonsterConfig {
    let file_path = format!("configs/monsters/{}.toml", monster_name);
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    toml::from_str(&contents).unwrap()
}

pub fn load() -> MonstersConfig {
    let mut monsters_config = MonstersConfig { config: HashMap::new() };
    let monster_config = read_config("demon_vulgar");
    monsters_config.config.insert(monster_config.monster.index, monster_config);
    let monster_config = read_config("demon_scout");
    monsters_config.config.insert(monster_config.monster.index, monster_config);
    monsters_config
}