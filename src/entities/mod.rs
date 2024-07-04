use bevy::prelude::*;
use abner::AbnerPlugin;
use bevy_xpbd_2d::prelude::PhysicsLayer;
use bullet::BulletPlugin;
use camera::CameraPlugin;
use gun::GunPlugin;
use urubu::UrubuPlugin;

mod camera;
pub mod urubu;
mod gun;
pub mod bullet;
pub mod abner;

#[derive(PhysicsLayer)]
pub enum CollisionLayer {
    Player,
    Bullet,
    Enemy
}
pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin{
    fn build(&self, app: &mut App) {
        app.add_plugins((CameraPlugin, GunPlugin, BulletPlugin, UrubuPlugin, AbnerPlugin));
    }
}