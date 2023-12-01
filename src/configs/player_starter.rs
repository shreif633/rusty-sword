use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use crate::enums::player_class::PlayerClass;

#[derive(Debug, Resource)]
pub struct PlayerStarterConfig {
    pub config: HashMap<PlayerClass, Config>
} 

#[derive(Debug, Deserialize)]
pub struct Config {
    pub welcome: Welcome,
    pub base_points: BasePoints,
    pub experience: Experience,
    pub position: Position,
    pub item: Vec<Item>
}

#[derive(Debug, Deserialize)]
pub struct Welcome {
    pub message: String
}

#[derive(Debug, Deserialize)]
pub struct Experience {
    pub experience: u32,
}

#[derive(Debug, Deserialize)]
pub struct BasePoints {
    pub base_strength: u8, 
    pub base_health: u8, 
    pub base_intelligence: u8, 
    pub base_wisdom: u8,
    pub base_agility: u8,  
}

#[derive(Debug, Deserialize)]
pub struct Position {
    pub map: u8,
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub index: u16,
    pub quantity: u32,
    pub prefix: u8,
    pub bound: bool
}

pub fn read_config(class_name: &str) -> Config {
    let file_path = format!("configs/player_starter/{}.toml", class_name);
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    toml::from_str(&contents).unwrap()
}

pub fn load() -> PlayerStarterConfig {
    let mut player_starter_config = PlayerStarterConfig { config: HashMap::new() };
    player_starter_config.config.insert(PlayerClass::Knight, read_config("knight"));
    player_starter_config.config.insert(PlayerClass::Mage, read_config("mage"));
    player_starter_config.config.insert(PlayerClass::Archer, read_config("archer"));
    player_starter_config
}