use bevy::prelude::*;
use bevy::window::WindowResolution;

mod bomb;
mod combat;
mod emitter;
mod enemy;
mod hp;
mod player;
mod weapon;

use bomb::BombPlugin;
use combat::CombatPlugin;
use emitter::EmitterPlugin;
use enemy::EnemyPlugin;
use hp::HpPlugin;
use player::PlayerPlugin;
use weapon::WeaponPlugin;

pub const WINDOW_W: u32 = 540;
pub const WINDOW_H: u32 = 960;

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
        .add_plugins((
            PlayerPlugin,
            WeaponPlugin,
            EnemyPlugin,
            EmitterPlugin,
            CombatPlugin,
            HpPlugin,
            BombPlugin,
        ))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
