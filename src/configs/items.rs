use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Resource)]
pub struct ItemsConfig {
    pub config: HashMap<u16, Config>
} 

#[derive(Debug, PartialEq)]
pub enum ItemCategory {
    Medicine,
    Weapon,
    Regular
}

#[derive(Debug)]
pub struct Config {
    pub name: String,
    pub index: u16,
    pub image: String,
    pub description: String,
    pub consumable: bool,
    pub stackable: bool,
    pub cooldown_in_seconds: f32,
    pub category: ItemCategory,
    pub range: u16,
    pub endurance: u16,
    pub attack_speed: u16,
    pub minimum_physical_attack: u16,
    pub maximum_physical_attack: u16,
    pub on_target_point: u16,
    pub health_recovered: u32
}

#[derive(Debug, Deserialize)]
pub struct OptionalConfig {
    pub name: String,
    pub index: u16,
    pub image: String,
    pub description: Option<String>,
    pub consumable: Option<bool>,
    pub stackable: Option<bool>,
    pub cooldown_in_seconds: Option<f32>,
    pub category: Option<String>,
    pub range: Option<u16>,
    pub endurance: Option<u16>,
    pub attack_speed: Option<u16>,
    pub minimum_physical_attack: Option<u16>,
    pub maximum_physical_attack: Option<u16>,
    pub on_target_point: Option<u16>,
    pub health_recovered: Option<u32>,
}

impl From<&OptionalConfig> for Config {
    fn from(val: &OptionalConfig) -> Self {
        let category = if val.category.is_none() {
            ItemCategory::Regular
        } else {
            match val.category.clone().unwrap().as_str() {
                "medicine" => ItemCategory::Medicine,
                "weapon" => ItemCategory::Weapon,
                _ => ItemCategory::Regular
            }
        };
        Config {
            name: val.name.clone(),
            index: val.index,
            image: val.image.clone(),
            description: val.description.clone().unwrap_or("".to_string()),
            consumable: val.consumable.unwrap_or(false),
            stackable: val.stackable.unwrap_or(false),
            cooldown_in_seconds: val.cooldown_in_seconds.unwrap_or(0.0),
            category,
            range: val.range.unwrap_or(0),
            endurance: val.endurance.unwrap_or(0),
            attack_speed: val.attack_speed.unwrap_or(0),
            minimum_physical_attack: val.minimum_physical_attack.unwrap_or(0),
            maximum_physical_attack: val.maximum_physical_attack.unwrap_or(0),
            on_target_point: val.on_target_point.unwrap_or(0),
            health_recovered: val.health_recovered.unwrap_or(0)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TempConfig {
    item: Vec<OptionalConfig>
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
        items_config.config.insert(item.index, (&item).into());
    }
    items_config
}