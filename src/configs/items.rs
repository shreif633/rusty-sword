use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Resource)]
pub struct ItemsConfig {
    pub config: HashMap<u16, Config>
} 

#[derive(Debug, Deserialize)]
pub struct Config {
    pub name: String,
    pub index: u16,
    pub image: String,
    pub description: String,
    pub consumable: bool,
    pub stackable: bool,
    pub cooldown_in_seconds: f32,
    pub medicine: Option<u32>
}

#[derive(Debug, Deserialize)]
pub struct TempConfig {
    item: Vec<Config>
}

pub fn read_config(group_name: &str) -> TempConfig {
    let file_path = format!("configs/items/{}.toml", group_name);
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    toml::from_str(&contents).unwrap()
}

pub fn load() -> ItemsConfig {
    let mut items_config = ItemsConfig { config: HashMap::new() };
    let items = read_config("medicine");
    for item in items.item {
        items_config.config.insert(item.index, item);
    }
    items_config
}