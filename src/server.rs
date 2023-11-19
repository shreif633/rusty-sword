use tokio::{net::TcpListener, io::{AsyncReadExt, AsyncWriteExt}, sync::mpsc};
use crate::framework::packet_queue::PacketQueue;
use crate::framework::world::{WorldLock, WorldManager};

pub async fn start(address: &str) -> tokio::io::Result<()> {
    println!("SERVER");
    let mut listener = TcpListener::bind(address).await?;
    let world = WorldLock::default();

    loop {
        println!("LOOP START");
        let (client, _) = listener.accept().await?;
        let (mut client_reader, mut client_writer) = client.into_split();
        let (writer, mut reader) = mpsc::channel::<Vec<u8>>(30);
        println!("LOOP CONN");
        let mut world = world.clone();
        println!("LOOP NOT LOCK");
        let user_id = world.insert_user(writer);
        // let user_id = 1;
        println!("LOOP ID {}", user_id);
        println!("WORLD {:?}", world);
        
        let _ = tokio::spawn(async move {
            let mut buffer = [0; 10024];
            let mut queue = PacketQueue { buffer: vec![] };
            loop {
                println!("LOOP");
                match client_reader.read(&mut buffer).await {
                    Ok(0) => {
                        println!("Client disconnected");
                        break;
                    },
                    Ok(n) => {
                        println!("READING");
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
            println!("ended client reader");
        });

        let _ = tokio::spawn(async move {
            while let Some(packet) = reader.recv().await {
                let _ = client_writer.write_all(&packet).await;
                println!("got = {:?}", packet);
            }
        });
    }
}