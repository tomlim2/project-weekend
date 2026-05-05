use bevy::prelude::*;

use crate::combat::Score;
use crate::emitter::EnemyBullet;
use crate::enemy::{Enemy, EnemySpawner};
use crate::player::{Player, PLAYER_HITBOX_RADIUS, PLAYER_START_Y};
use crate::weapon::PlayerBullet;
use crate::GameState;

const ENEMY_BULLET_RADIUS: f32 = 4.0;
const IFRAME_DURATION: f32 = 1.5;
pub const STARTING_LIVES: i32 = 3;

#[derive(Resource)]
pub struct Lives(pub i32);

impl Default for Lives {
    fn default() -> Self {
        Self(STARTING_LIVES)
    }
}

#[derive(Component, Default)]
pub struct IFrame(pub f32);

#[derive(Component)]
pub struct GameOverOverlay;

pub struct HpPlugin;

impl Plugin for HpPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Lives>()
            .add_systems(
                Update,
                (player_bullet_hit, tick_iframe, check_game_over)
                    .chain()
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                restart_input.run_if(in_state(GameState::GameOver)),
            )
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over_overlay)
            .add_systems(OnExit(GameState::GameOver), reset_game);
    }
}

fn player_bullet_hit(
    mut commands: Commands,
    mut lives: ResMut<Lives>,
    mut player_q: Query<(&Transform, &mut IFrame), With<Player>>,
    bullets: Query<(Entity, &Transform), With<EnemyBullet>>,
) {
    let Ok((p_t, mut iframe)) = player_q.single_mut() else {
        return;
    };
    if iframe.0 > 0.0 {
        return;
    }
    let r2 = (PLAYER_HITBOX_RADIUS + ENEMY_BULLET_RADIUS).powi(2);
    let p = p_t.translation.truncate();

    for (b_entity, b_t) in &bullets {
        let dx = b_t.translation.x - p.x;
        let dy = b_t.translation.y - p.y;
        if dx * dx + dy * dy <= r2 {
            lives.0 -= 1;
            iframe.0 = IFRAME_DURATION;
            commands.entity(b_entity).despawn();
            break;
        }
    }
}

fn tick_iframe(time: Res<Time>, mut q: Query<&mut IFrame, With<Player>>) {
    let dt = time.delta_secs();
    for mut f in &mut q {
        if f.0 > 0.0 {
            f.0 = (f.0 - dt).max(0.0);
        }
    }
}

fn check_game_over(lives: Res<Lives>, mut next: ResMut<NextState<GameState>>) {
    if lives.0 <= 0 {
        next.set(GameState::GameOver);
    }
}

fn restart_input(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::KeyR) {
        next.set(GameState::Playing);
    }
}

fn spawn_game_over_overlay(mut commands: Commands) {
    commands.spawn((
        GameOverOverlay,
        Text2d::new("GAME OVER\nPress R"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        TextColor(Color::srgb(1.0, 0.55, 0.6)),
        TextLayout::new_with_justify(Justify::Center),
        Transform::from_xyz(0.0, 0.0, 100.0),
    ));
}

fn reset_game(
    mut commands: Commands,
    mut lives: ResMut<Lives>,
    mut score: ResMut<Score>,
    mut spawner: ResMut<EnemySpawner>,
    overlays: Query<Entity, With<GameOverOverlay>>,
    enemies: Query<Entity, With<Enemy>>,
    enemy_bullets: Query<Entity, With<EnemyBullet>>,
    player_bullets: Query<Entity, With<PlayerBullet>>,
    mut player_q: Query<(&mut Transform, &mut IFrame), With<Player>>,
) {
    *lives = Lives::default();
    *score = Score::default();
    spawner.since_last = 0.0;

    for e in &overlays {
        commands.entity(e).despawn();
    }
    for e in &enemies {
        commands.entity(e).despawn();
    }
    for e in &enemy_bullets {
        commands.entity(e).despawn();
    }
    for e in &player_bullets {
        commands.entity(e).despawn();
    }
    if let Ok((mut t, mut iframe)) = player_q.single_mut() {
        t.translation.x = 0.0;
        t.translation.y = PLAYER_START_Y;
        iframe.0 = 0.0;
    }
}
