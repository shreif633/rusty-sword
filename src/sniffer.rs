use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use crate::framework::packet_queue::PacketQueue;
use crate::responses::ServerPacket;
use crate::requests::ClientPacket;

pub async fn start(client: &str, server: &str, hide_known_packets: bool, show_server_packets: bool, show_client_packets: bool) -> tokio::io::Result<()> {
    let listener = TcpListener::bind(client).await?;

    loop {
        let (client, _) = listener.accept().await?;
        let server = TcpStream::connect(server).await?;
        let (mut client_reader, mut client_writer) = client.into_split();
        let (mut server_reader, mut server_writer) = server.into_split();

        tokio::spawn(async move {
            let mut buffer = [0; 10024];
            let mut queue = PacketQueue { buffer: vec![] };
            loop {
                match client_reader.read(&mut buffer).await {
                    Ok(0) => break,
                    Ok(n) => {
                        queue.push(&buffer[..n]);
                        while let Some(packet_buffer) = queue.pop() {
                            let _ = server_writer.write_all(&packet_buffer).await;
                            let client_packet = crate::requests::deserialize(&packet_buffer);
                            if show_client_packets {
                                if hide_known_packets {
                                    if let ClientPacket::Unknown(client_packet) = client_packet {
                                        println!("[request] {:?}", &client_packet);
                                    }
                                } else {
                                    println!("[request] {:?}", &client_packet);
                                }
                            }
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

        tokio::spawn(async move {
            let mut buffer = [0; 10024];
            let mut queue = PacketQueue { buffer: vec![] };
            loop {
                match server_reader.read(&mut buffer).await {
                    Ok(0) => break,
                    Ok(n) => {
                        queue.push(&buffer[..n]);
                        while let Some(packet_buffer) = queue.pop() {
                            let _ = client_writer.write_all(&packet_buffer).await;
                            let server_packet = crate::responses::deserialize(&packet_buffer);
                            if show_server_packets {
                                if hide_known_packets {
                                    if let ServerPacket::Unknown(server_packet) = server_packet {
                                        println!("[response] {:?}", &server_packet);
                                    }
                                } else {
                                    println!("[response] {:?}", &server_packet);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("server error: {}", e);
                        break;
                    }
                }
            }
            println!("ended server reader");
        });
    }
}