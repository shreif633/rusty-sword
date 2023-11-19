use async_trait::async_trait;
use crate::framework::packet::HandlePacket;
use crate::framework::user::UserManager;
use crate::framework::world::{WorldLock, WorldManager};
use crate::packets::server::server_selected::ServerSelected;
use crate::packets::server::check_hash::CheckHash;
use crate::packets::server::analyze::Analyze;
use crate::packets::client::server_select::ServerSelect;

#[async_trait]
impl HandlePacket for ServerSelect {
    async fn handle(&self, world: &mut WorldLock, user_id: u32) {
        let current_user_lock = world.get_user_lock_by_id(user_id).unwrap();

        let analyze = Analyze { unknown: vec![211, 0, 0, 0, 28, 207, 86, 101, 0, 128, 3, 0] };
        current_user_lock.send(&mut (&analyze).into()).await;

        let check_hash = CheckHash { hash: 1325039837 };
        current_user_lock.send(&mut (&check_hash).into()).await;

        let server_selected = ServerSelected { unknown: vec![242, 108, 141, 16, 54, 212, 76, 126, 68, 30, 207, 86, 101, 97, 30, 0, 0, 118, 0, 0, 0, 0, 0, 0, 0, 2, 18, 2] };
        current_user_lock.send(&mut (&server_selected).into()).await;
    }
}