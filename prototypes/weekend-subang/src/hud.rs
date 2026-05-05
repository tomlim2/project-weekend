use bevy::prelude::*;

use crate::bomb::Bombs;
use crate::combat::Score;
use crate::hp::Lives;
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
            p.spawn((HudLives, Text::new("LIVES 3"), font.clone(), color));
            p.spawn((HudBombs, Text::new("BOMBS 3"), font, color));
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
        t.0 = format!("LIVES {}", lives.0.max(0));
    }
    if let Ok(mut t) = q_bombs.single_mut() {
        t.0 = format!("BOMBS {}", bombs.0);
    }
    if let Ok(mut t) = q_time.single_mut() {
        t.0 = format!("TIME  {:.1}", rt.0);
    }
}

fn reset_runtime(mut rt: ResMut<RunTime>) {
    rt.0 = 0.0;
}
