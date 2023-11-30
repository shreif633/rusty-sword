use bevy::prelude::*;
use sqlx::query;
use crate::{framework::database::Database, responses::player_skills::PlayerSkillsResponse};
use super::{tcp_server::SocketWriter, select_character::Player};

pub struct SkillsPlugin;

impl Plugin for SkillsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, load_skills);
    }
}

struct Skill {
    id: i32,
    index: u8,
    grade: u8,
}

#[derive(Component)]
struct Skills {
    skills: Vec<Skill>
}

fn query_player_skills(database: &Database, player_id: u32) -> Vec<Skill> {
    let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
    let rows = rt.block_on(async move {
        query!("SELECT * FROM skills WHERE player_id = ?", player_id).fetch_all(&database.connection).await.unwrap()
    });
    let mut skills: Vec<Skill> = vec![];
    for row in rows {
        let skill = Skill {
            id: row.id.try_into().unwrap(), 
            index: row.skill_index.try_into().unwrap(), 
            grade: row.grade.try_into().unwrap(), 
        };
        skills.push(skill);
    }
    skills
}

fn load_skills(mut commands: Commands, query: Query<(Entity, &Player, &SocketWriter), Without<Skills>>, database: Res<Database>) {
    for (entity, player, socket_writer) in &query {
        let player_skills = query_player_skills(&database, player.id);
        let skills: Vec<crate::responses::player_skills::Skill> = player_skills.iter().map(|i| crate::responses::player_skills::Skill { 
            index: i.index, 
            grade: i.grade, 
        }).collect();
        let player_skills = Skills { skills: player_skills };
        commands.entity(entity).insert(player_skills);
        let skills = PlayerSkillsResponse { skills };
        socket_writer.write(&mut (&skills).into());
    }
}