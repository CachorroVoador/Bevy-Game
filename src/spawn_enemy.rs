use std::time::Duration;
use bevy::{prelude::*, window::PrimaryWindow};

#[derive(Resource)]
struct SpawnAbner{
    timer_spawn: Timer
}

pub struct SpawnAbnerPlugin;

impl Plugin for SpawnAbnerPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup_timer)
        .add_systems(Update, spawn_abner.run_if(super::entities::urubu::has_player_alive));
    }
}

fn spawn_abner(
    commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnAbner>,
    asset_server: Res<AssetServer>,
    q_window: Query<&Window, With<PrimaryWindow>>
){
    timer.timer_spawn.tick(time.delta());

    if !timer.timer_spawn.finished() { return };
    super::entities::abner::spawn_abner(commands, asset_server, q_window.single());
}

fn setup_timer(mut commands: Commands){
    commands.insert_resource(
        SpawnAbner {
            timer_spawn: Timer::new(Duration::from_millis(500), TimerMode::Repeating),  
        }
    )
}