use bevy::prelude::*;
use std::sync::{Mutex, Arc};
use crate::components::item::Item;
use crate::components::monster::Monster;
use crate::components::player::Player;
use crate::configs::{player_starter, items, monsters, npcs};
use crate::framework::entity_map::EntityMap;
use crate::plugins::appear_disappear::AppearDisappearPlugin;
use crate::plugins::level::LevelPlugin;
use crate::plugins::normal_hit::NormalHitPlugin;
use crate::plugins::behead::BeheadPlugin;
use crate::plugins::medicine::MedicinePlugin;
use crate::plugins::persist_item::PersistItemPlugin;
use crate::plugins::player_health::PlayerHealthPlugin;
use crate::plugins::select_server::ServerSelectPlugin;
use crate::framework::database::Database;
use crate::plugins::skills::SkillsPlugin;
use crate::plugins::inventory::InventoryPlugin;
use crate::plugins::persist_player::PersistPlayerPlugin;
use crate::plugins::chat::ChatPlugin;
use crate::plugins::emote::EmotePlugin;
use crate::plugins::player_movement::PlayerMovementPlugin;
use crate::plugins::select_character::SelectCharacterPlugin;
use crate::plugins::character_selection::CharacterSelectionPlugin;
use crate::plugins::monsters_lifecycle::MonstersLifecyclePlugin;
use crate::plugins::spawn_npcs::SpawnNpcsPlugin;
use crate::plugins::speed::SpeedPlugin;
use crate::plugins::tcp_server::{SocketMessage, SocketQueue, SocketPair, TcpServerPlugin};
use crate::plugins::visual_effects::VisualEffectPlugin;
use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}, sync::mpsc::{self}};
use crate::framework::packet_queue::PacketQueue;


async fn start_game_server(queue: Arc<Mutex<Vec<SocketPair>>>) {
    tokio::spawn(async move {
        let socket_queue = SocketQueue { queue };
        let database = Database::connect().await;
        let player_starter_config = player_starter::load();
        let items_config = items::load();
        let monsters_config = monsters::load();
        let npcs_config = npcs::load();
        let players_map = EntityMap::<Player>::new();
        let monsters_map = EntityMap::<Monster>::new();
        let items_map = EntityMap::<Item>::new();
        App::new()
            .add_plugins(MinimalPlugins)
            .add_plugins(ServerSelectPlugin)
            .add_plugins(TcpServerPlugin)
            .add_plugins(CharacterSelectionPlugin)
            .add_plugins(SelectCharacterPlugin)
            .add_plugins(PlayerMovementPlugin)
            .add_plugins(EmotePlugin)
            .add_plugins(ChatPlugin)
            .add_plugins(PersistPlayerPlugin)
            .add_plugins(PersistItemPlugin)
            .add_plugins(InventoryPlugin)
            .add_plugins(SkillsPlugin)
            .add_plugins(PlayerHealthPlugin)
            .add_plugins(MedicinePlugin)
            .add_plugins(VisualEffectPlugin)
            .add_plugins(MonstersLifecyclePlugin)
            .add_plugins(SpawnNpcsPlugin)
            .add_plugins(NormalHitPlugin)
            .add_plugins(AppearDisappearPlugin)
            .add_plugins(BeheadPlugin)
            .add_plugins(SpeedPlugin)
            .add_plugins(LevelPlugin)
            .insert_resource(socket_queue)
            .insert_resource(player_starter_config)
            .insert_resource(items_config)
            .insert_resource(monsters_config)
            .insert_resource(npcs_config)
            .insert_resource(players_map)
            .insert_resource(monsters_map)
            .insert_resource(items_map)
            .insert_resource(database)
            //.add_plugins(crate::plugins::framerate::Framerate)
            .run();
    });
}

async fn start_tcp_server(address: &str, socket_queue: Arc<Mutex<Vec<SocketPair>>>) {
    let listener = TcpListener::bind(address).await.unwrap();
    loop {
        let (stream, socket_addr) = listener.accept().await.unwrap();
        let (mut stream_reader, mut stream_writer) = stream.into_split();
        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(20);
        {
            let socket_queue = socket_queue.clone();
            socket_queue.lock().unwrap().push(SocketPair(socket_addr.to_string(), SocketMessage::Connected(tx)));
        }
        let socket_queue = socket_queue.clone();
        tokio::spawn(async move {
            let mut buffer = [0; 10024];
            let mut queue = PacketQueue { buffer: vec![] };
            loop {
                match stream_reader.read(&mut buffer).await {
                    Ok(0) => break,
                    Ok(n) => {
                        queue.push(&buffer[..n]);
                        while let Some(packet_buffer) = queue.pop() {
                            let client_packet = crate::requests::deserialize(&packet_buffer);
                            {
                                socket_queue.lock().unwrap().push(SocketPair(socket_addr.to_string(), SocketMessage::Packet(client_packet)));
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("error: {}", e);
                        break;
                    }
                }
            }
        });

        tokio::spawn(async move {
            while let Some(packet) = rx.recv().await {
                let _ = stream_writer.write_all(&packet).await;
            }
        });
    }
}

pub async fn start(address: &str) -> tokio::io::Result<()> {
    let queue = Arc::new(Mutex::new(Vec::<SocketPair>::new()));
    start_game_server(queue.clone()).await;
    start_tcp_server(address, queue.clone()).await;
    Ok(())
}