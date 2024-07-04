use bevy::prelude::*;

use bevy_inspector_egui::InspectorOptions;
use bevy_inspector_egui::prelude::ReflectInspectorOptions;
use bevy_xpbd_2d::components::LinearVelocity;


#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Bullet{
    pub speed: f32
}

pub struct BulletPlugin;

impl Plugin for BulletPlugin{
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, move_bullet);
    }
}


fn move_bullet(mut bullet_query: Query<(&mut LinearVelocity, &mut Transform), With<Bullet>>){
    for (mut velocity, transform) in bullet_query.iter_mut(){
        let direction = transform.right() * 500.0;
        velocity.x = direction.x;
        velocity.y = direction.y;
    }

}