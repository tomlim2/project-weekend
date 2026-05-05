use bevy::prelude::*;

use crate::emitter::EnemyBullet;
use crate::hp::IFrame;
use crate::player::Player;
use crate::GameState;

pub const STARTING_BOMBS: u32 = 3;
const BOMB_INVULN: f32 = 1.0;
const BOMB_RING_LIFETIME: f32 = 0.5;
const BOMB_RING_INITIAL_R: f32 = 18.0;
const BOMB_RING_FINAL_SCALE: f32 = 22.0;

#[derive(Resource)]
pub struct Bombs(pub u32);

impl Default for Bombs {
    fn default() -> Self {
        Self(STARTING_BOMBS)
    }
}

#[derive(Component)]
pub struct BombRing {
    pub age: f32,
}

pub struct BombPlugin;

impl Plugin for BombPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Bombs>()
            .add_systems(
                Update,
                (trigger_bomb, animate_ring).run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::GameOver), reset_bombs);
    }
}

fn trigger_bomb(
    mut commands: Commands,
    mut bombs: ResMut<Bombs>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut player_q: Query<(&Transform, &mut IFrame), With<Player>>,
    bullets: Query<Entity, With<EnemyBullet>>,
) {
    if !keys.just_pressed(KeyCode::KeyX) {
        return;
    }
    if bombs.0 == 0 {
        return;
    }
    let Ok((p_t, mut iframe)) = player_q.single_mut() else {
        return;
    };
    bombs.0 -= 1;
    iframe.0 = iframe.0.max(BOMB_INVULN);
    for e in &bullets {
        commands.entity(e).despawn();
    }
    let origin = p_t.translation.truncate();
    commands.spawn((
        BombRing { age: 0.0 },
        Mesh2d(meshes.add(Annulus::new(BOMB_RING_INITIAL_R, BOMB_RING_INITIAL_R + 6.0))),
        MeshMaterial2d(materials.add(Color::srgba(1.0, 0.95, 0.7, 0.9))),
        Transform::from_xyz(origin.x, origin.y, 50.0),
    ));
}

fn animate_ring(
    time: Res<Time>,
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut q: Query<(
        Entity,
        &mut BombRing,
        &mut Transform,
        &MeshMaterial2d<ColorMaterial>,
    )>,
) {
    let dt = time.delta_secs();
    for (e, mut ring, mut tf, mat) in &mut q {
        ring.age += dt;
        let t_n = (ring.age / BOMB_RING_LIFETIME).clamp(0.0, 1.0);
        let scale = 1.0 + (BOMB_RING_FINAL_SCALE - 1.0) * t_n;
        tf.scale = Vec3::splat(scale);
        if let Some(m) = materials.get_mut(&mat.0) {
            let alpha = (1.0 - t_n) * 0.9;
            m.color = m.color.with_alpha(alpha);
        }
        if ring.age >= BOMB_RING_LIFETIME {
            commands.entity(e).despawn();
        }
    }
}

fn reset_bombs(
    mut commands: Commands,
    mut bombs: ResMut<Bombs>,
    rings: Query<Entity, With<BombRing>>,
) {
    *bombs = Bombs::default();
    for e in &rings {
        commands.entity(e).despawn();
    }
}
