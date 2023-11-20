use async_trait::async_trait;
use crate::framework::user::UserManager;
use crate::framework::world::{WorldLock, WorldManager};
use crate::framework::packet::HandlePacket;
use crate::packets::client::select_character::SelectCharacter;
use crate::packets::server::inventory::{Inventory, Item};
use crate::packets::server::player_appear::{PlayerAppear, PlayerClass};
use crate::packets::server::player_extra_agility::{self, PlayerExtraAgility};
use crate::packets::server::player_extra_health::{self, PlayerExtraHealth};
use crate::packets::server::player_extra_intelligence::{self, PlayerExtraIntelligence};
use crate::packets::server::player_extra_strength::{self, PlayerExtraStrength};
use crate::packets::server::player_extra_wisdom::{self, PlayerExtraWisdom};
use crate::packets::server::player_information::PlayerInformation;
use crate::packets::server::player_position::PlayerPosition;
use crate::packets::server::player_skills::{PlayerSkills, Skill};

#[async_trait]
impl HandlePacket for SelectCharacter {
    async fn handle(&self, world: &mut WorldLock, user_id: u32) {
        let current_user_lock = world.get_user_lock_by_id(user_id).unwrap();

        let position = PlayerPosition { unknown: vec![47, 1], x: 267701, y: 242655 };
        current_user_lock.send(&mut (&position).into()).await;

        let player_appear = PlayerAppear { 
            player_id: 573, 
            name: "Hermit".into(), 
            class: PlayerClass::Mage, 
            x: 267701, 
            y: 242655, 
            z: 19630, 
            unknown1: vec![1, 0, 0, 0, 0, 136, 0, 0, 0, 0], 
            weapon_index: 0, 
            shield_index: 0, 
            helmet_index: 262, 
            chest_index: 0, 
            shorts_index: 0, 
            gloves_index: 0, 
            boots_index: 0, 
            unknown2: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
            face: 6, 
            hair: 6, 
            unknown3: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 2, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] 
        };
        current_user_lock.send(&mut (&player_appear).into()).await;

        let player_skills = PlayerSkills {
            skills: vec![
                Skill { index: 0, grade: 1 }, Skill { index: 1, grade: 1 }, Skill { index: 11, grade: 2 }, Skill { index: 4, grade: 10 }, Skill { index: 5, grade: 1 }, Skill { index: 17, grade: 1 }, Skill { index: 18, grade: 1 }, Skill { index: 19, grade: 1 }, Skill { index: 20, grade: 1 }, Skill { index: 21, grade: 1 }, Skill { index: 3, grade: 20 }, Skill { index: 23, grade: 10 }, Skill { index: 15, grade: 3 }, Skill { index: 13, grade: 5 }, Skill { index: 24, grade: 5 }, Skill { index: 30, grade: 3 }, Skill { index: 82, grade: 1 }, Skill { index: 32, grade: 2 }, Skill { index: 44, grade: 1 }, Skill { index: 31, grade: 2 }, Skill { index: 38, grade: 2 }, Skill { index: 9, grade: 5 }, Skill { index: 27, grade: 1 }, Skill { index: 37, grade: 1 }, Skill { index: 8, grade: 3 }, Skill { index: 10, grade: 10 }, Skill { index: 16, grade: 1 }, Skill { index: 29, grade: 1 }, Skill { index: 22, grade: 5 }, Skill { index: 26, grade: 3 }
            ]
        };
        current_user_lock.send(&mut (&player_skills).into()).await;

        let inventory = Inventory { 
            items: vec![
                Item { index: 262, id: -2147482791, prefix: 85, info: 2097161, quantity: 1, maximum_endurance: 14, current_endurance: 14, unknown1: 0, physical_attack_talisman: 0, magical_attack_talisman: 0, unknown2: vec![0], talisman_of_accuracy: 0, unknown3: vec![0], talisman_of_defence: 0, unknown4: vec![57], upgrade_level: 0, upgrade_rate: 0, seconds_remaining: 0, unknown5: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
            ]
        };
        current_user_lock.send(&mut (&inventory).into()).await;

        let player_information = PlayerInformation { 
            specialization: 7, 
            unknown1: vec![0, 0], 
            contribution: 9, 
            base_strength: 19, 
            base_health: 118, 
            base_intelligence: 111, 
            base_wisdom: 53, 
            base_agility: 24, 
            current_health_points: 4077, 
            maximum_health_points: 6077, 
            current_magic_points: 1053, 
            maximum_magic_points: 1261, 
            on_target_point: 80, 
            evasion: 83, 
            defense: 100, 
            absorption: 9, 
            experience: 15000,
            unknown2: vec![0, 0, 0], 
            minimum_physical_attack: 160, 
            maximum_physical_attack: 212, 
            minimum_magical_attack: 229, 
            maximum_magical_attack: 371, 
            status_points: 2, 
            skill_points: 44, 
            fire_resistence: 11, 
            ice_resistence: 21, 
            lighning_resistence: 31, 
            curse_resistence: 17, 
            non_elemental_resistence: 25, 
            rage: 617142 
        };
        current_user_lock.send(&mut (&player_information).into()).await;

        let player_extra_health = PlayerExtraHealth { 
            extra_health: 11, 
            current_health_points: 4077, 
            maximum_health_points: 6077, 
            non_elemental_resistence: 25 
        };
        current_user_lock.send(&mut (&player_extra_health).into()).await;

        let player_extra_strength = PlayerExtraStrength { 
            extra_strength: 10, 
            on_target_point: 80, 
            minimum_physical_attack: 160, 
            maximum_physical_attack: 212 
        };
        current_user_lock.send(&mut (&player_extra_strength).into()).await;

        let player_extra_intelligence = PlayerExtraIntelligence { 
            extra_intelligence: 12, 
            minimum_magical_attack: 229, 
            maximum_magical_attack: 371, 
            fire_resistence: 11, 
            ice_resistence: 21, 
            lighning_resistence: 31 
        };
        current_user_lock.send(&mut (&player_extra_intelligence).into()).await;

        let player_extra_wisdom = PlayerExtraWisdom { 
            extra_wisdom: 13, 
            current_magic_points: 1053, 
            maximum_magic_points: 1261, 
            minimum_magical_attack: 229, 
            maximum_magical_attack: 371, 
            curse_resistence: 17 
        };
        current_user_lock.send(&mut (&player_extra_wisdom).into()).await;

        let player_extra_agility = PlayerExtraAgility {
            extra_agility: 14, 
            on_target_point: 80, 
            evasion: 83, 
            unknown_evasion_copy: 83, 
            minimum_physical_attack: 160, 
            maximum_physical_attack: 212
        };
        current_user_lock.send(&mut (&player_extra_agility).into()).await;
    }
}