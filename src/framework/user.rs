use std::sync::{Arc, RwLock};
use tokio::sync::mpsc::Sender;
use async_trait::async_trait;
use crate::framework::packet::Packet;

#[derive(Debug)]
pub struct User {
    pub id: u32,
    pub writer: Sender<Vec<u8>>
}

pub type UserLock = Arc<RwLock<User>>;

#[async_trait]
pub trait UserManager {
    async fn send(&self, packet: &mut Packet);
}

#[async_trait]
impl UserManager for UserLock {

    async fn send(&self, packet: &mut Packet) {
        let writer = {
            let user = self.read().unwrap();
            user.writer.clone() 
        };
        if writer.send(packet.serialize()).await.is_err() {
            println!("receiver dropped");
        }
    }

}