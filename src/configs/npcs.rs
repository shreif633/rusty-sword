use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Resource)]
pub struct NpcsConfig {
    pub config: HashMap<u16, NpcConfig>
} 

#[derive(Debug, Deserialize)]
pub struct NpcConfig {
    pub name: String,
    pub index: u16,
    pub shape: u8,
    pub quest: u16,
    pub quest_flag: u8,
    pub html: u32,
    pub map: u8,
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub look_at_x: u32,
    pub look_at_y: u32
}

#[derive(Debug, Deserialize)]
pub struct OptionalConfig {
    pub name: String,
    pub index: u16,
    pub shape: u8,
    pub quest: Option<u16>,
    pub quest_flag: Option<u8>,
    pub html: Option<u32>,
    pub map: u8,
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub look_at_x: u32,
    pub look_at_y: u32
}

impl From<&OptionalConfig> for NpcConfig {
    fn from(val: &OptionalConfig) -> Self {
        NpcConfig {
            name: val.name.clone(),
            index: val.index,
            shape: val.shape,
            quest: val.quest.unwrap_or(0),
            quest_flag: val.quest_flag.unwrap_or(0),
            html: val.html.unwrap_or(0),
            map: val.map,
            x: val.x,
            y: val.y,
            z: val.z,
            look_at_x: val.look_at_x,
            look_at_y: val.look_at_y,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TempConfig {
    npc: Vec<OptionalConfig>
}

pub fn read_config(group_name: &str) -> TempConfig {
    let file_path = format!("configs/npcs/{}.toml", group_name);
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    toml::from_str(&contents).unwrap()
}

pub fn load() -> NpcsConfig {
    let mut npcs_config = NpcsConfig { config: HashMap::new() };
    let npcs = read_config("narootuh");
    for npc in npcs.npc {
        npcs_config.config.insert(npc.index, (&npc).into());
    }
    npcs_config
}