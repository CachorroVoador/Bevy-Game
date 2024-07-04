use bevy::prelude::*;
use bevy_xpbd_2d::prelude::CollidingEntities;

use crate::{entities::{bullet::Bullet, urubu::Urubu}, game_state::GameState, Points};

pub struct ColliderDetectionPlugin;

impl Plugin for ColliderDetectionPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collider_detection);
    }
}

fn collider_detection(
    query: Query<(Entity, &CollidingEntities, Option<&Urubu>, Option<&Bullet>)>,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut points: ResMut<Points>
) {
    for (entity, colliding_entities, urubu, bullet) in &query {
        for colliding in &colliding_entities.0{
            if let Some(_urubu) = urubu{
                game_state.game_over = true;
                break;
            }
            if let Some(_bullet) = bullet {
                commands.entity(*colliding).despawn();
                commands.entity(entity).despawn();
                points.points += 1;
                break;
            }
        }
    }
}

