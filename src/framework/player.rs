use std::sync::{Arc, RwLock};
use tokio::sync::mpsc::Sender;
use async_trait::async_trait;
use crate::framework::packet::Packet;

#[derive(Debug)]
pub struct Player {
    pub id: u32,
    pub writer: Sender<Vec<u8>>,
    pub name: String,

    pub x: u32,
    pub y: u32,
    pub z: u32,

    pub face: u8,
    pub hair: u8,

    pub weapon_index: u16, 
    pub shield_index: u16, 
    pub helmet_index: u16, 
    pub chest_index: u16, 
    pub shorts_index: u16, 
    pub gloves_index: u16, 
    pub boots_index: u16, 
}

pub type PlayerLock = Arc<RwLock<Player>>;

#[async_trait]
pub trait PlayerManager {
    async fn send(&self, packet: &mut Packet);
}

#[async_trait]
impl PlayerManager for PlayerLock {

    async fn send(&self, packet: &mut Packet) {
        let writer = {
            let player = self.read().unwrap();
            player.writer.clone() 
        };
        if writer.send(packet.serialize()).await.is_err() {
            println!("receiver dropped");
        }
    }

}