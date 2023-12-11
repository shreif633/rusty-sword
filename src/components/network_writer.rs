use bevy::prelude::*;
use tokio::sync::mpsc::Sender;
use crate::framework::packet::Packet;

#[derive(Component)]
pub struct NetworkWriter {
    pub socket_writer: Sender<Vec<u8>>
}

impl NetworkWriter {
    pub fn write(&self, packet: &mut Packet) {
        let rt = tokio::runtime::Builder::new_current_thread().build().unwrap();
        let buffer = packet.serialize();
        rt.block_on(async move {
            self.socket_writer.send(buffer).await.unwrap();
        });
    }
}