use bevy::prelude::*;

use crate::hp::IFrame;
use crate::weapon::AutoFire;
use crate::GameState;

const PLAYER_SPEED: f32 = 220.0;
const PLAYER_FOCUS_MULT: f32 = 0.5;
const PLAYER_W: f32 = 14.0;
const PLAYER_H: f32 = 18.0;
const PLAY_FIELD_W: f32 = 540.0;
const PLAY_FIELD_H: f32 = 960.0;
pub const PLAYER_HITBOX_RADIUS: f32 = 2.5;
pub const PLAYER_START_Y: f32 = -PLAY_FIELD_H * 0.35;

#[derive(Component)]
pub struct Player {
    pub focused: bool,
}

#[derive(Component)]
pub struct HitboxIndicator;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (read_focus, move_player, toggle_hitbox_visibility, blink_player_iframe)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(
                Update,
                toggle_hitbox_visibility.run_if(in_state(GameState::GameOver)),
            );
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let body_material = materials.add(Color::srgb(0.85, 0.94, 1.0));
    let hitbox_material = materials.add(Color::srgba(1.0, 0.35, 0.45, 1.0));
    commands
        .spawn((
            Player { focused: false },
            AutoFire::default(),
            IFrame::default(),
            Mesh2d(meshes.add(Rectangle::new(PLAYER_W, PLAYER_H))),
            MeshMaterial2d(body_material),
            Transform::from_xyz(0.0, PLAYER_START_Y, 0.0),
        ))
        .with_children(|parent| {
            parent.spawn((
                HitboxIndicator,
                Mesh2d(meshes.add(Circle::new(PLAYER_HITBOX_RADIUS))),
                MeshMaterial2d(hitbox_material),
                Transform::from_xyz(0.0, 0.0, 1.0),
                Visibility::Hidden,
            ));
        });
}

fn read_focus(keys: Res<ButtonInput<KeyCode>>, mut q: Query<&mut Player>) {
    if let Ok(mut p) = q.single_mut() {
        p.focused = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
    }
}

fn move_player(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut q: Query<(&Player, &mut Transform)>,
) {
    let Ok((p, mut transform)) = q.single_mut() else {
        return;
    };

    let mut dir = Vec2::ZERO;
    if keys.pressed(KeyCode::ArrowUp) {
        dir.y += 1.0;
    }
    if keys.pressed(KeyCode::ArrowDown) {
        dir.y -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowLeft) {
        dir.x -= 1.0;
    }
    if keys.pressed(KeyCode::ArrowRight) {
        dir.x += 1.0;
    }

    if dir.length_squared() > 0.0 {
        let speed = if p.focused {
            PLAYER_SPEED * PLAYER_FOCUS_MULT
        } else {
            PLAYER_SPEED
        };
        let delta = dir.normalize() * speed * time.delta_secs();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;

        let half_w = PLAY_FIELD_W * 0.5 - PLAYER_W * 0.5;
        let half_h = PLAY_FIELD_H * 0.5 - PLAYER_H * 0.5;
        transform.translation.x = transform.translation.x.clamp(-half_w, half_w);
        transform.translation.y = transform.translation.y.clamp(-half_h, half_h);
    }
}

fn toggle_hitbox_visibility(
    player_q: Query<&Player>,
    mut hitbox_q: Query<&mut Visibility, With<HitboxIndicator>>,
) {
    let focused = player_q.single().map(|p| p.focused).unwrap_or(false);
    for mut v in &mut hitbox_q {
        *v = if focused {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    }
}

fn blink_player_iframe(
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q: Query<(&IFrame, &MeshMaterial2d<ColorMaterial>), With<Player>>,
) {
    let Ok((iframe, mat)) = q.single() else {
        return;
    };
    let Some(color_mat) = materials.get_mut(&mat.0) else {
        return;
    };
    let alpha = if iframe.0 > 0.0 {
        let phase = (time.elapsed_secs() * 18.0).sin();
        0.45 + 0.35 * (phase * 0.5 + 0.5)
    } else {
        1.0
    };
    color_mat.color = color_mat.color.with_alpha(alpha);
}
