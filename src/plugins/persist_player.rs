use bevy::prelude::*;
use crate::components::id::Id;
use crate::repositories::player::{PlayerUpdatePositionChangeset, update_all_player_position_by_id};
use crate::components::position::Position;
use crate::components::player::Player;
use crate::framework::database::Database;

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
    players_query: Query<(&Id, &Position), With<Player>>, 
    time: Res<Time>,
    database: Res<Database>
) {
    let mut timer = timer.get_single_mut().unwrap();
    timer.timer.tick(time.delta());
    if timer.timer.just_finished() {
        let mut changesets = Vec::<PlayerUpdatePositionChangeset>::new();
        for (id, position) in &players_query {
            let changeset = PlayerUpdatePositionChangeset { id: id.id, x: position.x, y: position.y, z: position.z };
            changesets.push(changeset);
        }
        update_all_player_position_by_id(&database, &changesets);
    }
} 