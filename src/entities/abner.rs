use bevy::prelude::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_xpbd_2d::{components::{CollisionLayers, LinearVelocity, RigidBody}, prelude::Collider};
use rand::Rng;

use crate::entities::CollisionLayer;

use super::urubu::Urubu;


#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Abner{
    #[inspector(min = 0.0)]
    speed: f32,
}

pub struct AbnerPlugin;

impl Plugin for AbnerPlugin{
    fn build(&self, app: &mut App) {
        app
        .register_type::<Abner>()
        .add_systems(Update, abner_movement);
    }
}

pub fn spawn_abner(mut commands: Commands, asset_server: Res<AssetServer>, window: &Window){
    println!("Abner Spawnado!");
    let mut rng = rand::thread_rng();
    let mut abner = commands.spawn(SpriteBundle {
        texture: asset_server.load("entities/abner_sprite.png"),
        transform: Transform::from_xyz(rng.gen_range(-window.width()..=window.width()) , rng.gen_range(-window.height()..=window.height()), 0.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
        ..default()
    });
    abner.insert(Name::new("Abner"));
    abner.insert(Abner { speed: 300.0});
    abner.insert(RigidBody::Kinematic);
    abner.insert(Collider::circle(64.0));
    abner.insert(CollisionLayers::new(
        [CollisionLayer::Enemy],
        [CollisionLayer::Player, CollisionLayer::Bullet]
    ));
}


fn abner_movement(mut abner_character: Query<(&mut LinearVelocity, &mut Transform, &Abner), Without<Urubu>>,
 mut urubu_query: Query<(&Urubu, &mut Transform), Without<Abner>>){
     for (mut linear_velocity, transform, abner) in &mut abner_character{
            let (_, urubu_transform) = urubu_query.single_mut();

            let direction = urubu_transform.translation - transform.translation;
            let mut velocity = Vec2::new(direction.x,direction.y);

            velocity = velocity.normalize_or_zero() * abner.speed;

            linear_velocity.x = velocity.x;
            linear_velocity.y = velocity.y;

    }
}
