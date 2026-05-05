use bevy::prelude::*;

use crate::bomb::{Bombs, STARTING_BOMBS};
use crate::combat::Score;
use crate::hp::{Lives, STARTING_LIVES};
use crate::persist::BestScore;
use crate::GameState;

#[derive(Resource, Default)]
pub struct RunTime(pub f32);

#[derive(Component)]
pub struct HudScore;

#[derive(Component)]
pub struct HudHi;

#[derive(Component)]
pub struct HudLives;

#[derive(Component)]
pub struct HudBombs;

#[derive(Component)]
pub struct HudTime;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RunTime>()
            .add_systems(Startup, spawn_hud)
            .add_systems(
                Update,
                (tick_runtime, refresh_hud).run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, refresh_hud.run_if(in_state(GameState::GameOver)))
            .add_systems(OnExit(GameState::GameOver), reset_runtime);
    }
}

fn spawn_hud(mut commands: Commands) {
    let font = TextFont {
        font_size: 14.0,
        ..default()
    };
    let color = TextColor(Color::srgb(0.9, 0.95, 1.0));

    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            top: Val::Px(8.0),
            left: Val::Px(10.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(2.0),
            ..default()
        },))
        .with_children(|p| {
            p.spawn((HudScore, Text::new("SCORE 0"), font.clone(), color));
            p.spawn((HudHi, Text::new("HI    0"), font.clone(), color));
            p.spawn((HudTime, Text::new("TIME  0.0"), font.clone(), color));
        });

    commands
        .spawn((Node {
            position_type: PositionType::Absolute,
            top: Val::Px(8.0),
            right: Val::Px(10.0),
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(2.0),
            align_items: AlignItems::FlexEnd,
            ..default()
        },))
        .with_children(|p| {
            let hearts_color = TextColor(Color::srgb(1.0, 0.45, 0.55));
            let bombs_color = TextColor(Color::srgb(1.0, 0.85, 0.45));
            p.spawn((HudLives, Text::new(hearts_string(STARTING_LIVES, STARTING_LIVES as i32)), font.clone(), hearts_color));
            p.spawn((HudBombs, Text::new(stars_string(STARTING_BOMBS, STARTING_BOMBS)), font, bombs_color));
        });
}

fn tick_runtime(time: Res<Time>, mut rt: ResMut<RunTime>) {
    rt.0 += time.delta_secs();
}

#[allow(clippy::too_many_arguments)]
fn refresh_hud(
    score: Res<Score>,
    best: Res<BestScore>,
    lives: Res<Lives>,
    bombs: Res<Bombs>,
    rt: Res<RunTime>,
    mut q_score: Query<&mut Text, (With<HudScore>, Without<HudHi>, Without<HudLives>, Without<HudBombs>, Without<HudTime>)>,
    mut q_hi: Query<&mut Text, (With<HudHi>, Without<HudScore>, Without<HudLives>, Without<HudBombs>, Without<HudTime>)>,
    mut q_lives: Query<&mut Text, (With<HudLives>, Without<HudScore>, Without<HudHi>, Without<HudBombs>, Without<HudTime>)>,
    mut q_bombs: Query<&mut Text, (With<HudBombs>, Without<HudScore>, Without<HudHi>, Without<HudLives>, Without<HudTime>)>,
    mut q_time: Query<&mut Text, (With<HudTime>, Without<HudScore>, Without<HudHi>, Without<HudLives>, Without<HudBombs>)>,
) {
    if let Ok(mut t) = q_score.single_mut() {
        t.0 = format!("SCORE {}", score.score);
    }
    if let Ok(mut t) = q_hi.single_mut() {
        let shown = score.score.max(best.0);
        t.0 = format!("HI    {}", shown);
    }
    if let Ok(mut t) = q_lives.single_mut() {
        t.0 = hearts_string(STARTING_LIVES, lives.0);
    }
    if let Ok(mut t) = q_bombs.single_mut() {
        t.0 = stars_string(STARTING_BOMBS, bombs.0);
    }
    if let Ok(mut t) = q_time.single_mut() {
        t.0 = format!("TIME  {:.1}", rt.0);
    }
}

fn reset_runtime(mut rt: ResMut<RunTime>) {
    rt.0 = 0.0;
}

fn hearts_string(max: i32, current: i32) -> String {
    let cur = current.max(0);
    let filled = cur.min(max);
    let empty = (max - filled).max(0);
    let mut s = String::with_capacity((max as usize) * 4 + 6);
    s.push_str("LIFE ");
    for _ in 0..filled {
        s.push('\u{2665}'); // ♥ filled heart
    }
    for _ in 0..empty {
        s.push('\u{2661}'); // ♡ empty heart
    }
    s
}

fn stars_string(max: u32, current: u32) -> String {
    let filled = current.min(max);
    let empty = max.saturating_sub(filled);
    let mut s = String::with_capacity((max as usize) * 4 + 6);
    s.push_str("BOMB[X] ");
    for _ in 0..filled {
        s.push('\u{2605}'); // ★ filled star
    }
    for _ in 0..empty {
        s.push('\u{2606}'); // ☆ empty star
    }
    s
}
