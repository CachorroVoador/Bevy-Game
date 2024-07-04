use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ineffable::{commands::IneffableCommands, config::simple_asset_loading::MergeMode, prelude::{ineff, InputAction}, register::InputActionRegistrar, resources::Ineffable};
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_xpbd_2d::{components::{CollisionLayers, LinearVelocity, RigidBody}, prelude::Collider};

use crate::{entities::CollisionLayer, game_state::GameState};


#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Urubu{
    #[inspector(min = 0.0)]
    pub speed: f32,
}

#[derive(InputAction)]
pub enum PlayerInput {
    #[ineffable(dual_axis)]
    Movement
}

//arrumar isso aqui depois
// pub mod gun;
// pub mod bullet;

pub struct UrubuPlugin;

impl Plugin for UrubuPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (setup, super::gun::spawn_gun).chain())
            .register_type::<Urubu>()
            .register_input_action::<PlayerInput>()
            .add_systems(FixedUpdate, (character_movement, limit_player_movement).run_if(has_player_alive))
            .add_systems(Update, stop_urubu.run_if(has_player_dead));
    }
}

pub fn has_player_alive(game_state: Res<GameState>) -> bool {
    return !game_state.game_over;
}

pub fn has_player_dead(game_state: Res<GameState>) -> bool {
    return game_state.game_over;
}

fn stop_urubu(mut query: Query<&mut LinearVelocity, With<Urubu>>){
    let mut velocity = query.single_mut();
    velocity.x = 0.0;
    velocity.y = 0.0;
}

fn character_movement(mut query: Query<(&mut LinearVelocity, &Urubu)>,
    input: Res<Ineffable>,
){
    for (mut linear_velocity, urubu) in &mut query{
        let mut velocity = Vec2::new(0.0,0.0);
        let input_value = input.direction_2d(ineff!(PlayerInput::Movement));
        velocity += input_value;
        velocity = velocity.normalize_or_zero() * urubu.speed;
        linear_velocity.x = velocity.x;
        linear_velocity.y = velocity.y;
    }
}

fn limit_player_movement(mut query: Query<&mut Transform, With<Urubu>>, window_query: Query<&Window, With<PrimaryWindow>>){
    let mut urubu_transform = query.single_mut();
    let window = window_query.get_single().unwrap();

    urubu_transform.translation.x = urubu_transform.translation.x.clamp(-window.width() / 2.0, window.width() / 2.0);
    urubu_transform.translation.y = urubu_transform.translation.y.clamp(-window.height() / 2.0, window.height() / 2.0);
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut ineffable: IneffableCommands){
    let mut urubu = commands.spawn(SpriteBundle{
        texture: asset_server.load("entities/urubu_sprite.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(50.0,100.0)),
            ..default()
        },
        ..default()
    });
    urubu.insert(RigidBody::Kinematic);
    urubu.insert(Name::new("Urubu"));
    urubu.insert(Urubu { speed: 500.0 });
    urubu.insert(Collider::circle(16.0));
    urubu.insert(CollisionLayers::new(
        [CollisionLayer::Player],
        [CollisionLayer::Enemy]
    ));
    println!("Urubu criado");

    ineffable.load_configs(vec![(MergeMode::Base, "input.ron")]);
}