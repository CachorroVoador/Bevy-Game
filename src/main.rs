#![feature(exact_size_is_empty)]

use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_ineffable::plugin::IneffablePlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_xpbd_2d::plugins::PhysicsPlugins;

mod entities;
use entities::EntitiesPlugin;

mod collider_detection;
use collider_detection::ColliderDetectionPlugin;

mod game_state;
use game_state::GameStatePlugin;

mod spawn_enemy;
use interface::InterfacePlugin;
use spawn_enemy::SpawnAbnerPlugin;

mod interface;

#[derive(Resource)]
pub struct Points{
    points: i32
}

fn main() {
    println!("Bem vinda à Urubu Engine!");
    App::new()
        .add_plugins((DefaultPlugins, IneffablePlugin, InterfacePlugin, SpawnAbnerPlugin , GameStatePlugin, ColliderDetectionPlugin, PhysicsPlugins::default(), WorldInspectorPlugin::new().run_if(input_just_pressed(KeyCode::F1)), EntitiesPlugin))
        .insert_resource(Points {points: 0} )
        .run();
}