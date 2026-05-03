use bevy::prelude::*;
use bevy::window::WindowResolution;

mod player;

use player::PlayerPlugin;

const WINDOW_W: u32 = 540;
const WINDOW_H: u32 = 960;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
    GameOver,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Weekend Subang".into(),
                resolution: WindowResolution::new(WINDOW_W, WINDOW_H),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
