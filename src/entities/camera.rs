use bevy::prelude::*;

#[derive(Component)]
pub struct Camera;

pub struct CameraPlugin;

fn setup_camera(mut commands: Commands){
    println!("Camera criada");
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0,0.0,0.0),
            camera: bevy::render::camera::Camera{
                clear_color: ClearColorConfig::Custom(Color::rgb(0.005, 0.513, 0.683)),
                ..default()
            },
            ..default()
        },
        Camera
    ));
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, setup_camera);
    }
}