use bevy::prelude::*;

use crate::GameState;

const PLAYER_SPEED: f32 = 220.0;
const PLAYER_FOCUS_MULT: f32 = 0.5;
const PLAYER_W: f32 = 14.0;
const PLAYER_H: f32 = 18.0;
const PLAY_FIELD_W: f32 = 540.0;
const PLAY_FIELD_H: f32 = 960.0;

#[derive(Component)]
pub struct Player {
    pub focused: bool,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player).add_systems(
            Update,
            (read_focus, move_player).run_if(in_state(GameState::Playing)),
        );
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Player { focused: false },
        Mesh2d(meshes.add(Rectangle::new(PLAYER_W, PLAYER_H))),
        MeshMaterial2d(materials.add(Color::srgb(0.85, 0.94, 1.0))),
        Transform::from_xyz(0.0, -PLAY_FIELD_H * 0.35, 0.0),
    ));
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
