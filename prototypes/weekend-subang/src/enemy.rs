use bevy::prelude::*;
use rand::Rng;

use crate::emitter::BulletEmitter;
use crate::weapon::{Lifetime, Velocity};
use crate::GameState;

const SPAWN_INTERVAL: f32 = 0.7;
const ENEMY_RADIUS: f32 = 10.0;
const ENEMY_SPEED: f32 = 110.0;
const ENEMY_LIFETIME: f32 = 10.0;
const SPAWN_X_HALF_RANGE: f32 = 220.0; // ±220 from center

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Hp(pub i32);

#[derive(Resource, Default)]
pub struct EnemySpawner {
    pub since_last: f32,
}

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawner>().add_systems(
            Update,
            (spawn_enemies, move_enemies, despawn_dead_enemies)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn spawn_enemies(
    time: Res<Time>,
    mut spawner: ResMut<EnemySpawner>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawner.since_last += time.delta_secs();
    if spawner.since_last < SPAWN_INTERVAL {
        return;
    }
    spawner.since_last = 0.0;

    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-SPAWN_X_HALF_RANGE..SPAWN_X_HALF_RANGE);
    let y = crate::WINDOW_H as f32 * 0.5 + ENEMY_RADIUS + 4.0;

    let aimed = rng.gen_bool(0.5);
    let (color, emitter) = if aimed {
        (
            Color::srgb(1.0, 0.55, 0.45),
            BulletEmitter::aimed(3, 12.0, 220.0, 1.5),
        )
    } else {
        (
            Color::srgb(0.55, 0.7, 1.0),
            BulletEmitter::ring(12, 160.0, 7.0, 0.8),
        )
    };

    commands.spawn((
        Enemy,
        Hp(1),
        Velocity(Vec2::new(0.0, -ENEMY_SPEED)),
        Lifetime(0.0),
        emitter,
        Mesh2d(meshes.add(Circle::new(ENEMY_RADIUS))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(x, y, 0.0),
    ));
}

fn move_enemies(
    time: Res<Time>,
    mut q: Query<(&Velocity, &mut Transform, &mut Lifetime), With<Enemy>>,
) {
    let dt = time.delta_secs();
    for (v, mut t, mut life) in &mut q {
        t.translation.x += v.0.x * dt;
        t.translation.y += v.0.y * dt;
        life.0 += dt;
    }
}

fn despawn_dead_enemies(
    mut commands: Commands,
    q: Query<(Entity, &Hp, &Transform, &Lifetime), With<Enemy>>,
) {
    let bottom = -(crate::WINDOW_H as f32 * 0.5) - ENEMY_RADIUS - 4.0;
    for (e, hp, t, life) in &q {
        if hp.0 <= 0 || t.translation.y < bottom || life.0 > ENEMY_LIFETIME {
            commands.entity(e).despawn();
        }
    }
}
