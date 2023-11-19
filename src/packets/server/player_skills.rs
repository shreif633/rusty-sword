use crate::framework::packet::Packet;

pub const HEADER: u8 = 16;

#[derive(Debug)]
pub struct Skill {
    pub index: u8,
    pub grade: u8,
}

#[derive(Debug)]
pub struct PlayerSkills {
    pub skills: Vec<Skill>,
}

impl From<&mut Packet> for PlayerSkills {
    fn from(packet: &mut Packet) -> Self {
        let skill_count = packet.get_u8();
        let mut skills = Vec::<Skill>::with_capacity(skill_count as usize);
        for _ in 0..skill_count {
            let index = packet.get_u8();
            let grade = packet.get_u8();
            let skill = Skill { index, grade };
            skills.push(skill);
        }
        PlayerSkills { skills }
    }
}

impl From<&PlayerSkills> for Packet {
    fn from(val: &PlayerSkills) -> Self {
        let mut packet = Packet::from(HEADER);
        packet.write_u8(val.skills.len().try_into().unwrap());
        for skill in &val.skills {
            packet.write_u8(skill.index);
            packet.write_u8(skill.grade);
        }
        packet
    }
}