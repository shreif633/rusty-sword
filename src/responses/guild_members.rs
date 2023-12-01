use crate::framework::packet::Packet;

pub const HEADER: u8 = 94;
pub const SUB_HEADER: u8 = 6;

#[derive(Debug)]
pub enum Position {
    Leader = 1,
    SubLeader = 2,
    Manager = 3,
    Chief = 4,
    RegularMember = 5,
    TemporaryMember = 6,
}

#[derive(Debug)]
pub struct Member {
    pub name: String,
    pub position: Position,
    pub level: u8,
}

#[derive(Debug)]
pub struct GuildMembersResponse {
    pub unknown: Vec<u8>,
    pub guild_name: String,
    pub leader_position_name: String,
    pub subleader_position_name: String,
    pub manager_position_name: String,
    pub chief_position_name: String,
    pub regular_member_position_name: String,
    pub temporary_member_position_name: String,
    pub members: Vec<Member>,
}

impl From<&mut Packet> for GuildMembersResponse {
    fn from(packet: &mut Packet) -> Self {
        let unknown = packet.get_buffer(4);
        let guild_name = packet.get_string();
        let leader_position_name = packet.get_string();
        let subleader_position_name = packet.get_string();
        let manager_position_name = packet.get_string();
        let chief_position_name = packet.get_string();
        let regular_member_position_name = packet.get_string();
        let temporary_member_position_name = packet.get_string();
        let members_count = packet.get_u8();
        let mut members = Vec::<Member>::with_capacity(members_count as usize);
        for _ in 0..members_count {
            let name = packet.get_string();
            let position = packet.get_u8();
            let position = match position {
                1 => Position::Leader,
                2 => Position::SubLeader,
                3 => Position::Manager,
                4 => Position::Chief,
                5 => Position::RegularMember,
                _ => Position::TemporaryMember
            };
            let level = packet.get_u8();
            let member = Member { name, position, level };
            members.push(member);
        }
        GuildMembersResponse { 
            unknown, guild_name, leader_position_name, subleader_position_name, manager_position_name, 
            chief_position_name, regular_member_position_name, temporary_member_position_name, members  
        }
    }
}

impl From<&GuildMembersResponse> for Packet {
    fn from(val: &GuildMembersResponse) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u8(SUB_HEADER);
        packet.write_buffer(&val.unknown);
        packet.write_string(&val.guild_name);
        packet.write_string(&val.leader_position_name);
        packet.write_string(&val.subleader_position_name);
        packet.write_string(&val.manager_position_name);
        packet.write_string(&val.chief_position_name);
        packet.write_string(&val.regular_member_position_name);
        packet.write_string(&val.temporary_member_position_name);
        packet.write_u8(val.members.len().try_into().unwrap());
        for member in &val.members {
            packet.write_string(&member.name);
            match member.position {
                Position::Leader => packet.write_u8(1),
                Position::SubLeader => packet.write_u8(2),
                Position::Manager => packet.write_u8(3),
                Position::Chief => packet.write_u8(4),
                Position::RegularMember => packet.write_u8(5),
                Position::TemporaryMember => packet.write_u8(6),
            };
            packet.write_u8(member.level);
        }
        packet
    }
}