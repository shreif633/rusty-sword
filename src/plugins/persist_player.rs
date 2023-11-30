use bevy::prelude::*;
use crate::framework::database::Database;
use super::{player_movement::Position, select_character::Player};
use sqlx::query;

pub struct PersistPlayerPlugin;

impl Plugin for PersistPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_persist_player_timer);
        app.add_systems(Update, persist_player);
    }
}

#[derive(Component)]
struct PersistPlayerTimer {
    timer: Timer,
}

fn create_persist_player_timer(mut commands: Commands) {
    commands.spawn(PersistPlayerTimer { timer: Timer::from_seconds(30.0, TimerMode::Repeating) });
}

fn persist_player(
    mut timer: Query<&mut PersistPlayerTimer>, 
    players_query: Query<(&Player, &Position)>, 
    time: Res<Time>,
    database: Res<Database>
) {
    let mut timer = timer.get_single_mut().unwrap();
    timer.timer.tick(time.delta());
    if timer.timer.just_finished() {
        let mut update = "".to_string();
        for (player, position) in &players_query {
            update.push_str(&format!("UPDATE players SET x = {}, y = {}, z = {} WHERE id = {};", position.x, position.y, position.z, player.id));
        }
        let rt = tokio::runtime::Builder::new_current_thread().enable_time().build().unwrap();
        rt.block_on(async move {
            query(&update).execute(&database.connection).await.unwrap()
        });
    }
} 