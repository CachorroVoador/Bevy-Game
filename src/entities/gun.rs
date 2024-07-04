use bevy::input::common_conditions::input_pressed;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_xpbd_2d::components::{CollisionLayers, RigidBody};
use bevy_xpbd_2d::prelude::Collider;

use super::bullet::Bullet;
use super::urubu::{has_player_alive, Urubu};
use crate::entities::CollisionLayer;

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Gun;

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct GunPoint;


#[derive(Resource, Default)]
struct WorldCoords(Vec2);

pub struct GunPlugin;

impl Plugin for GunPlugin{
    fn build(&self, app: &mut App) {
        app
        .init_resource::<WorldCoords>()
        .add_systems(Update, (shoot.run_if(input_pressed(MouseButton::Left)), look_to_mouse).run_if(has_player_alive));
    }
}


pub fn spawn_gun(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
){
    println!("Arma spawnada");

    let sprite = SpriteBundle{
        texture: asset_server.load("entities/gun.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(50.0, 50.0)),
            flip_y: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 1.0),
        ..Default::default()};
    let gun = commands.spawn((sprite,
        Name::new("Gun"),
        Gun )).id();
    commands.spawn((TransformBundle {
        local: Transform::from_xyz(39.0, 0.0, 0.0),
        ..default()
    }, GunPoint)).set_parent(gun);

}

fn look_to_mouse(
    mut query: Query<(&mut Transform, &Gun)>,
    mut mycoords: ResMut<WorldCoords>,
    mut player_query: Query<&mut Transform, (With<Urubu>, Without<Gun>)>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&bevy::render::camera::Camera, &GlobalTransform)>
){
    let (camera, camera_transform) = q_camera.single();

    let window = q_window.single();

    if let Some(world_position) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
        {
            mycoords.0 = world_position;
        }
    for (mut gun_transform, _) in &mut query{
        let pos = gun_transform.translation.truncate();
        let player = player_query.single_mut();

        let direction = mycoords.0 - pos;
        let angle = direction.y.atan2(direction.x);

        gun_transform.rotation = Quat::from_rotation_z(angle);
        
        gun_transform.translation = player.translation;
        gun_transform.translation.z = 1.0;
    }
}

fn shoot(
    commands: Commands,
    asset_server: Res<AssetServer>,
    mut point_query: Query<&mut GlobalTransform, With<GunPoint>>,
    mut gun_query: Query<&mut Transform, With<Gun>>
){
    let mut global_transform = point_query.single_mut();
    let mut transform = gun_query.single_mut();
         

    create_bullet(commands,
        asset_server,
        global_transform.as_mut(),
         transform.as_mut());

}

fn create_bullet(mut commands: Commands, asset_server: Res<AssetServer>, global_transform: &mut GlobalTransform, transform: &mut Transform){
    let sprite =SpriteBundle{
        texture: asset_server.load("entities/gun.png"),
        sprite: Sprite {
            custom_size: Some(Vec2::new(30.0, 30.0)),
            ..default()
        },
        // transform: Transform::from_xyz(transform.translation().x, transform.translation().y, 1.0),
        transform: Transform {
            translation: Vec3::new(global_transform.translation().x, global_transform.translation().y, 1.0),
            rotation: transform.rotation,
            ..default()
        },
        ..Default::default()};

    commands.spawn((sprite,
        Bullet{ speed: 30.0},
        Name::new("Bullet"),
        RigidBody::Kinematic,
        Collider::circle(4.0),
        CollisionLayers::new([CollisionLayer::Bullet], [CollisionLayer::Enemy])
    ));

}
