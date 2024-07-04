use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiPlugin;

use crate::{entities::urubu::has_player_dead, Points};


#[derive(Component)]
struct PointsText;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin{
    fn build(&self, app: &mut App) {
        app
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup_points_text)
        .add_systems(Update, (update_points_text, show_game_over_text.run_if(has_player_dead)));
    }
}

fn setup_points_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut text = commands.spawn(TextBundle::from_sections([
            TextSection::new(
                "Your Points: ", TextStyle {
                    font: asset_server.load("fonts/arial.ttf"),
                    font_size: 50.0,
                    ..default()
                }),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/arial.ttf"),
                font_size: 50.0,
                ..default()
            }),
        ]));
    text.insert(PointsText);
}

fn show_game_over_text(mut commands: Commands, asset_server: Res<AssetServer>){
    commands.spawn((TextBundle::from_section("Game Over!", TextStyle {
        font: asset_server.load("fonts/arial.ttf"),
        font_size: 100.0,
        ..default()
    }).with_text_justify(JustifyText::Center)).with_style(Style{
        position_type: PositionType::Absolute,
        left: Val::Percent(0.10),
        right: Val::Percent(0.10),
        top: Val::Percent(0.10),
        bottom: Val::Percent(0.10),
        ..Default::default()
    }));
}

fn update_points_text(mut text_query: Query<&mut Text, With<PointsText>>, points: ResMut<Points>){
    for mut text in &mut text_query {
        text.sections[1].value = format!("{0}", points.points);
    }
}