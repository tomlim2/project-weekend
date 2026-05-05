use bevy::prelude::*;
use std::fs;
use std::path::PathBuf;

use crate::combat::Score;
use crate::GameState;

const SAVE_FILE: &str = "weekend-subang-save.json";

#[derive(Resource, Default)]
pub struct BestScore(pub u32);

pub struct PersistPlugin;

impl Plugin for PersistPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BestScore>()
            .add_systems(Startup, load_best)
            .add_systems(OnEnter(GameState::GameOver), save_best);
    }
}

fn save_path() -> PathBuf {
    PathBuf::from(SAVE_FILE)
}

fn load_best(mut best: ResMut<BestScore>) {
    let Ok(s) = fs::read_to_string(save_path()) else {
        return;
    };
    if let Some(value) = s
        .split(':')
        .nth(1)
        .and_then(|t| t.trim_matches(|c: char| !c.is_ascii_digit()).parse::<u32>().ok())
    {
        best.0 = value;
    }
}

fn save_best(score: Res<Score>, mut best: ResMut<BestScore>) {
    if score.score <= best.0 {
        return;
    }
    best.0 = score.score;
    let body = format!("{{\"best\":{}}}\n", best.0);
    let _ = fs::write(save_path(), body);
}
