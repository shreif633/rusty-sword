use crate::framework::packet::Packet;

pub async fn sandbox() -> tokio::io::Result<()> {
    let bytes = [18, 0, 94, 0, 5, 115, 117, 99, 107, 32, 109, 121, 32, 100, 105, 99, 107, 0];
    let mut packet = Packet::new(&bytes);
    
    let header = packet.get_header();
    println!("header {:?}", header);
    let sub_header = packet.get_u8();
    println!("sub_header {:?}", sub_header);
    let unknown = packet.get_buffer(1);
    println!("unknown {:?}", unknown);
    let notice = packet.get_string();
    println!("notice {:?}", notice);

    Ok(())
}