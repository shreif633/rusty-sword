use crate::framework::packet::Packet;

pub const HEADER: u8 = 9;

#[derive(Debug)]
pub enum ItemQuantityAction {
    Unknown = 0,
    Buy = 7,
    Consume = 9,
    Drop = 11,
    Pick = 12,
}

#[derive(Debug)]
pub struct UpdateItemQuantityResponse {
    pub item_id: u32,
    pub quantity: u32,
    pub action: ItemQuantityAction
}

impl From<&mut Packet> for UpdateItemQuantityResponse {
    fn from(packet: &mut Packet) -> Self {
        let item_id = packet.get_u32();
        let quantity = packet.get_u32();
        let action = packet.get_u8();
        let action = match action {
            7 => ItemQuantityAction::Buy,
            9 => ItemQuantityAction::Consume,
            11 => ItemQuantityAction::Drop,
            12 => ItemQuantityAction::Pick,
            _ => {
                println!("UnknownItemQuantityAction {:?}", action);
                ItemQuantityAction::Unknown
            },
        };
        UpdateItemQuantityResponse { item_id, quantity, action }
    }
}


impl From<&UpdateItemQuantityResponse> for Packet {
    fn from(val: &UpdateItemQuantityResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u32(val.item_id);
        packet.write_u32(val.quantity);
        let action = match val.action {
            ItemQuantityAction::Buy => 7,
            ItemQuantityAction::Consume => 9,
            ItemQuantityAction::Drop => 11,
            ItemQuantityAction::Pick => 12,
            ItemQuantityAction::Unknown => 0
        };
        packet.write_u8(action);
        packet
    }
}