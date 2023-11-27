use async_trait::async_trait;
use crate::framework::player::{PlayerManager, PlayerClass};
use crate::framework::world::{WorldLock, WorldManager};
use crate::framework::packet::HandlePacket;
use crate::models::user::User;
use crate::packets::client::select_character::SelectCharacter;
use crate::packets::server::guild_members::{GuildMembers, Member, Position};
use crate::packets::server::inventory::{Inventory, Item};
use crate::packets::server::player_appear::PlayerAppear;
use crate::packets::server::player_extra_agility::PlayerExtraAgility;
use crate::packets::server::player_extra_health::PlayerExtraHealth;
use crate::packets::server::player_extra_intelligence::PlayerExtraIntelligence;
use crate::packets::server::player_extra_strength::PlayerExtraStrength;
use crate::packets::server::player_extra_wisdom::PlayerExtraWisdom;
use crate::packets::server::player_information::PlayerInformation;
use crate::packets::server::player_position::PlayerPosition;
use crate::packets::server::player_skills::{PlayerSkills, Skill};
use sqlx::Row;

#[async_trait]
impl HandlePacket for SelectCharacter {
    async fn handle(&self, current_user: &mut User) {
        let player = current_user.select_player(self.character_id);
        // let player = world.create_player(user_id);
        
        // {
        //     let db = {
        //         world.get_database().read().unwrap().connection.clone()
        //     };

        //     let id = {
        //         let current_user = world.get_user_by_id(user_id).unwrap();
        //         let current_user = current_user.read().unwrap();
        //         current_user.id
        //     };
        //     let row = sqlx::query("SELECT * FROM players WHERE id = ? AND user_id = ? AND deleted_at IS NULL")
        //         .bind(self.character_id)
        //         .bind(id)
        //         .fetch_one(&db).await;

        //     if row.is_err() {
        //         return;
        //     }

        //     let row = row.unwrap();

        //     let mut player = player.write().unwrap();
        //     player.name = row.try_get("name").unwrap();

        //     player.x = row.try_get("x").unwrap();
        //     player.y = row.try_get("y").unwrap();
        //     player.z = row.try_get("z").unwrap();

        //     player.face = row.try_get("face").unwrap();
        //     player.hair = row.try_get("hair").unwrap();

        //     player.class = match row.try_get("class").unwrap() {
        //         0 => PlayerClass::Knight,
        //         1 => PlayerClass::Mage,
        //         _ => PlayerClass::Archer,
        //     };

        //     player.weapon_index = row.try_get("weapon_index").unwrap();
        //     player.shield_index = row.try_get("shield_index").unwrap();
        //     player.helmet_index = row.try_get("helmet_index").unwrap();
        //     player.chest_index = row.try_get("chest_index").unwrap();
        //     player.shorts_index = row.try_get("shorts_index").unwrap();
        //     player.gloves_index = row.try_get("gloves_index").unwrap();
        //     player.boots_index = row.try_get("boots_index").unwrap();

        //     player.current_health_points = row.try_get("current_health_points").unwrap();
        //     player.maximum_health_points = row.try_get("maximum_health_points").unwrap();
        //     player.current_magic_points = row.try_get("current_magic_points").unwrap();
        //     player.maximum_magic_points = row.try_get("maximum_magic_points").unwrap();

        //     player.experience = row.try_get("experience").unwrap();
        //     player.rage = row.try_get("rage").unwrap();

        //     player.base_strength = row.try_get("base_strength").unwrap();
        //     player.base_health = row.try_get("base_health").unwrap();
        //     player.base_intelligence = row.try_get("base_intelligence").unwrap();
        //     player.base_wisdom = row.try_get("base_wisdom").unwrap();
        //     player.base_agility = row.try_get("base_agility").unwrap();
        // }

        let position = { 
            let player = player.read().unwrap();
            PlayerPosition { unknown: vec![47, 1], x: player.x, y: player.y }
        };
        player.send(&mut (&position).into()).await;

        let mut player_appear = {
            let player = player.read().unwrap();
            PlayerAppear { 
                player_id: player.id, 
                name: player.name.clone(), 
                class: player.class, 
                is_current_player: true,
                x: player.x, 
                y: player.y, 
                z: player.z, 
                unknown1: vec![1, 0, 0, 0, 0, 136, 0, 0, 0, 0], 
                weapon_index: player.weapon_index, 
                shield_index: player.shield_index, 
                helmet_index: player.helmet_index, 
                chest_index: player.chest_index, 
                shorts_index: player.shorts_index, 
                gloves_index: player.gloves_index, 
                boots_index: player.boots_index, 
                unknown2: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
                face: player.face, 
                hair: player.hair, 
                unknown3: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 2, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] 
            }
        };
        player.send(&mut (&player_appear).into()).await;

        for other_player in world.get_other_players_in_sight(user_id) {
            player_appear.is_current_player = false;
            other_player.send(&mut (&player_appear).into()).await;

            let other_player_appear = {
                let other_player = other_player.read().unwrap();
                PlayerAppear { 
                    player_id: other_player.id, 
                    name: other_player.name.clone(), 
                    class: other_player.class, 
                    is_current_player: false,
                    x: other_player.x, 
                    y: other_player.y, 
                    z: other_player.z, 
                    unknown1: vec![1, 0, 0, 0, 0, 136, 0, 0, 0, 0], 
                    weapon_index: other_player.weapon_index, 
                    shield_index: other_player.shield_index, 
                    helmet_index: other_player.helmet_index, 
                    chest_index: other_player.chest_index, 
                    shorts_index: other_player.shorts_index, 
                    gloves_index: other_player.gloves_index, 
                    boots_index: other_player.boots_index, 
                    unknown2: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], 
                    face: other_player.face, 
                    hair: other_player.hair, 
                    unknown3: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 36, 2, 0, 0, 96, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] 
                }
            };
            player.send(&mut (&other_player_appear).into()).await;
        }

        let player_skills = PlayerSkills {
            skills: vec![
                Skill { index: 0, grade: 1 }, Skill { index: 1, grade: 1 }, Skill { index: 11, grade: 2 }, Skill { index: 4, grade: 10 }, Skill { index: 5, grade: 1 }, Skill { index: 17, grade: 1 }, Skill { index: 18, grade: 1 }, Skill { index: 19, grade: 1 }, Skill { index: 20, grade: 1 }, Skill { index: 21, grade: 1 }, Skill { index: 3, grade: 20 }, Skill { index: 23, grade: 10 }, Skill { index: 15, grade: 3 }, Skill { index: 13, grade: 5 }, Skill { index: 24, grade: 5 }, Skill { index: 30, grade: 3 }, Skill { index: 82, grade: 1 }, Skill { index: 32, grade: 2 }, Skill { index: 44, grade: 1 }, Skill { index: 31, grade: 2 }, Skill { index: 38, grade: 2 }, Skill { index: 9, grade: 5 }, Skill { index: 27, grade: 1 }, Skill { index: 37, grade: 1 }, Skill { index: 8, grade: 3 }, Skill { index: 10, grade: 10 }, Skill { index: 16, grade: 1 }, Skill { index: 29, grade: 1 }, Skill { index: 22, grade: 5 }, Skill { index: 26, grade: 3 }
            ]
        };
        player.send(&mut (&player_skills).into()).await;

        let inventory = Inventory { 
            items: vec![
                Item { index: 262, id: -2147482791, prefix: 85, info: 2097161, quantity: 1, maximum_endurance: 14, current_endurance: 14, unknown1: 0, physical_attack_talisman: 0, magical_attack_talisman: 0, unknown2: vec![0], talisman_of_accuracy: 0, unknown3: vec![0], talisman_of_defence: 0, unknown4: vec![57], upgrade_level: 0, upgrade_rate: 0, seconds_remaining: 0, unknown5: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] },
                Item { index: 781, id: -2147482787, prefix: 83, info: 1081345, quantity: 1, maximum_endurance: 30, current_endurance: 28, unknown1: 0, physical_attack_talisman: 7, magical_attack_talisman: 10, unknown2: vec![0], talisman_of_accuracy: 11, unknown3: vec![0], talisman_of_defence: 39, unknown4: vec![57], upgrade_level: 10, upgrade_rate: 1, seconds_remaining: 0, unknown5: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] }
            ]
        };
        player.send(&mut (&inventory).into()).await;

        let player_information = {
            let player = player.read().unwrap();
            PlayerInformation { 
                specialization: 7, 
                unknown1: vec![0, 0], 
                contribution: 9, 
                base_strength: player.base_strength, 
                base_health: player.base_health, 
                base_intelligence: player.base_intelligence, 
                base_wisdom: player.base_wisdom, 
                base_agility: player.base_agility, 
                current_health_points: player.current_health_points, 
                maximum_health_points: player.maximum_health_points, 
                current_magic_points: player.current_magic_points, 
                maximum_magic_points: player.maximum_magic_points, 
                on_target_point: 80, 
                evasion: 83, 
                defense: 100, 
                absorption: 9, 
                experience: player.experience,
                unknown2: vec![0, 0, 0], 
                minimum_physical_attack: 160, 
                maximum_physical_attack: 212, 
                minimum_magical_attack: 229, 
                maximum_magical_attack: 371, 
                status_points: 20, 
                skill_points: 44, 
                fire_resistence: 11, 
                ice_resistence: 21, 
                lighning_resistence: 31, 
                curse_resistence: 17, 
                non_elemental_resistence: 25, 
                rage: player.rage 
            }
        };
        player.send(&mut (&player_information).into()).await;

        let player_extra_health = PlayerExtraHealth { 
            extra_health: 11, 
            current_health_points: 4077, 
            maximum_health_points: 6077, 
            non_elemental_resistence: 25 
        };
        player.send(&mut (&player_extra_health).into()).await;

        let player_extra_strength = PlayerExtraStrength { 
            extra_strength: 10, 
            on_target_point: 80, 
            minimum_physical_attack: 160, 
            maximum_physical_attack: 212 
        };
        player.send(&mut (&player_extra_strength).into()).await;

        let player_extra_intelligence = PlayerExtraIntelligence { 
            extra_intelligence: 12, 
            minimum_magical_attack: 229, 
            maximum_magical_attack: 371, 
            fire_resistence: 11, 
            ice_resistence: 21, 
            lighning_resistence: 31 
        };
        player.send(&mut (&player_extra_intelligence).into()).await;

        let player_extra_wisdom = PlayerExtraWisdom { 
            extra_wisdom: 13, 
            current_magic_points: 1053, 
            maximum_magic_points: 1261, 
            minimum_magical_attack: 229, 
            maximum_magical_attack: 371, 
            curse_resistence: 17 
        };
        player.send(&mut (&player_extra_wisdom).into()).await;

        let player_extra_agility = PlayerExtraAgility {
            extra_agility: 14, 
            on_target_point: 80, 
            evasion: 83, 
            unknown_evasion_copy: 83, 
            minimum_physical_attack: 160, 
            maximum_physical_attack: 212
        };
        player.send(&mut (&player_extra_agility).into()).await;

        let guild_members = GuildMembers { 
            unknown: vec![36, 2, 0, 0], 
            guild_name: "KalSaga".to_string(), 
            leader_position_name: "Leader".to_string(), 
            subleader_position_name: "SubLeader".to_string(), 
            manager_position_name: "Manager".to_string(), 
            chief_position_name: "Chief".to_string(), 
            regular_member_position_name: "Member".to_string(), 
            temporary_member_position_name: "TempMember".to_string(), 
            members: vec![
                Member { name: "Mortaro".to_string(), position: Position::Leader, level: 0 }, 
                Member { name: "Hermit".to_string(), position: Position::RegularMember, level: 60 }, 
                Member { name: "CJB".to_string(), position: Position::TemporaryMember, level: 0 }, 
                Member { name: "Comma".to_string(), position: Position::SubLeader, level: 0 }
            ] 
        };
        player.send(&mut (&guild_members).into()).await;
    }
}