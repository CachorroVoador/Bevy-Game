use bevy::prelude::*;


#[derive(Resource, Default)]
pub struct GameState{
    pub game_over: bool
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin{
    fn build(&self, app: &mut App) {
        app
        .init_resource::<GameState>()
        .add_systems(Startup, load_state);
    }
}

fn load_state(mut game_state: ResMut<GameState>){
    game_state.game_over = false;
}