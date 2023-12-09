use bevy::prelude::*;
use sqlx::query;
use crate::components::player::Player;
use crate::components::id::Id;
use crate::responses::player_skills::PlayerSkillsResponse;
use crate::framework::database::Database;
use super::tcp_server::SocketWriter;

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

fn query_player_skills(database: &Database, player_id: i32) -> Vec<Skill> {
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
    skills.push(Skill { id: 1, index: 1, grade: 1 });
    // skills.push(Skill { id: 3, index: 3, grade: 1 });
    skills
}

fn load_skills(mut commands: Commands, query: Query<(Entity, &Id, &SocketWriter), (With<Player>, Without<Skills>)>, database: Res<Database>) {
    for (entity, id, socket_writer) in &query {
        let player_skills = query_player_skills(&database, id.id);
        let skills: Vec<crate::responses::player_skills::Skill> = player_skills.iter().map(|i| crate::responses::player_skills::Skill { 
            index: i.index, 
            grade: i.grade, 
        }).collect();
        let player_skills = Skills { skills: player_skills };
        commands.entity(entity).insert(player_skills);
        let skills = PlayerSkillsResponse { skills };
        println!("SKILLS {:?}", skills);
        socket_writer.write(&mut (&skills).into());
    }
}