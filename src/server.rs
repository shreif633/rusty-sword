use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}, sync::mpsc};
use crate::framework::packet_queue::PacketQueue;
use crate::framework::world::{WorldLock, WorldManager};

pub async fn start(address: &str) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(address).await?;
    let world = WorldLock::default();

    loop {
        let (client, _) = listener.accept().await?;
        let (mut client_reader, mut client_writer) = client.into_split();
        let (writer, mut reader) = mpsc::channel::<Vec<u8>>(30);
        let mut world = world.clone();
        
        let _ = tokio::spawn(async move {
            let user_id = world.insert_user(writer);
            let mut buffer = [0; 10024];
            let mut queue = PacketQueue { buffer: vec![] };
            loop {
                match client_reader.read(&mut buffer).await {
                    Ok(0) => break,
                    Ok(n) => {
                        queue.push(&buffer[..n]);
                        while let Some(packet_buffer) = queue.pop() {
                            let client_packet = crate::packets::client::deserialize(&packet_buffer);
                            client_packet.handle(&mut world, user_id).await;
                        }
                    }
                    Err(e) => {
                        eprintln!("client error: {}", e);
                        break;
                    }
                }
            }
            world.remove_user(user_id);
            println!("ended client reader");
        });

        let _ = tokio::spawn(async move {
            while let Some(packet) = reader.recv().await {
                let _ = client_writer.write_all(&packet).await;
            }
            println!("signals ended");
        });
    }
}