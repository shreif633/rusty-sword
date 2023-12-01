use crate::framework::packet::Packet;

pub const HEADER: u8 = 254;

#[derive(Debug)]
pub struct AnalyzeResponse {
    pub unknown: Vec<u8>
}

impl AnalyzeResponse {
    pub fn new() -> Self {
        AnalyzeResponse { unknown: vec![211, 0, 0, 0, 28, 207, 86, 101, 0, 128, 3, 0] }
    }
}

impl From<&mut Packet> for AnalyzeResponse {
    fn from(packet: &mut Packet) -> Self {
        AnalyzeResponse { unknown: packet.buffer.clone() }
    }
}

impl From<&AnalyzeResponse> for Packet {
    fn from(val: &AnalyzeResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_buffer(&val.unknown);
        packet
    }
}