use bevy::prelude::*;

use crate::enemy::Enemy;
use crate::player::Player;
use crate::GameState;

const BULLET_RADIUS: f32 = 4.0;
const BULLET_LIFETIME: f32 = 6.0;

#[derive(Clone, Copy)]
pub enum Pattern {
    Aimed {
        count: u32,
        spread_deg: f32,
        speed: f32,
    },
    Ring {
        count: u32,
        speed: f32,
        drift_deg: f32,
    },
}

#[derive(Component)]
pub struct BulletEmitter {
    pub pattern: Pattern,
    pub rate: f32,
    pub since_last: f32,
    pub phase_deg: f32,
}

impl BulletEmitter {
    pub fn aimed(count: u32, spread_deg: f32, speed: f32, rate: f32) -> Self {
        Self {
            pattern: Pattern::Aimed {
                count,
                spread_deg,
                speed,
            },
            rate,
            since_last: 0.0,
            phase_deg: 0.0,
        }
    }

    pub fn ring(count: u32, speed: f32, drift_deg: f32, rate: f32) -> Self {
        Self {
            pattern: Pattern::Ring {
                count,
                speed,
                drift_deg,
            },
            rate,
            since_last: 0.0,
            phase_deg: 0.0,
        }
    }
}

#[derive(Component)]
pub struct EnemyBullet;

#[derive(Component)]
pub struct EBVelocity(pub Vec2);

#[derive(Component)]
pub struct EBLifetime(pub f32);

#[derive(Resource)]
pub struct EnemyBulletAssets {
    pub mesh: Handle<Mesh>,
    pub mat_aimed: Handle<ColorMaterial>,
    pub mat_ring: Handle<ColorMaterial>,
}

pub struct EmitterPlugin;

impl Plugin for EmitterPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_assets).add_systems(
            Update,
            (tick_emitters, move_enemy_bullets, age_and_cull)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn init_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(EnemyBulletAssets {
        mesh: meshes.add(Circle::new(BULLET_RADIUS)),
        mat_aimed: materials.add(Color::srgb(1.0, 0.55, 0.25)),
        mat_ring: materials.add(Color::srgb(0.4, 0.9, 1.0)),
    });
}

fn tick_emitters(
    time: Res<Time>,
    mut commands: Commands,
    assets: Res<EnemyBulletAssets>,
    player_q: Query<&Transform, (With<Player>, Without<Enemy>)>,
    mut emitters: Query<(&Transform, &mut BulletEmitter), With<Enemy>>,
) {
    let dt = time.delta_secs();
    let player_pos = player_q.single().ok().map(|t| t.translation.truncate());

    for (t, mut em) in &mut emitters {
        em.since_last += dt;
        let interval = 1.0 / em.rate.max(0.0001);
        if em.since_last < interval {
            continue;
        }
        em.since_last = 0.0;
        let origin = t.translation.truncate();

        match em.pattern {
            Pattern::Aimed {
                count,
                spread_deg,
                speed,
            } => {
                let Some(target) = player_pos else {
                    continue;
                };
                let dir = (target - origin).normalize_or_zero();
                if dir == Vec2::ZERO {
                    continue;
                }
                let base = dir.y.atan2(dir.x);
                let spread = spread_deg.to_radians();
                for i in 0..count {
                    let t_n = if count == 1 {
                        0.0
                    } else {
                        i as f32 / (count - 1) as f32 * 2.0 - 1.0
                    };
                    let a = base + t_n * spread;
                    let v = Vec2::new(a.cos(), a.sin()) * speed;
                    spawn_bullet(&mut commands, &assets.mesh, &assets.mat_aimed, origin, v);
                }
            }
            Pattern::Ring {
                count,
                speed,
                drift_deg,
            } => {
                em.phase_deg = (em.phase_deg + drift_deg).rem_euclid(360.0);
                let phase = em.phase_deg.to_radians();
                let step = std::f32::consts::TAU / count.max(1) as f32;
                for i in 0..count {
                    let a = phase + step * i as f32;
                    let v = Vec2::new(a.cos(), a.sin()) * speed;
                    spawn_bullet(&mut commands, &assets.mesh, &assets.mat_ring, origin, v);
                }
            }
        }
    }
}

fn spawn_bullet(
    commands: &mut Commands,
    mesh: &Handle<Mesh>,
    mat: &Handle<ColorMaterial>,
    origin: Vec2,
    velocity: Vec2,
) {
    commands.spawn((
        EnemyBullet,
        EBVelocity(velocity),
        EBLifetime(0.0),
        Mesh2d(mesh.clone()),
        MeshMaterial2d(mat.clone()),
        Transform::from_xyz(origin.x, origin.y, 0.0),
    ));
}

fn move_enemy_bullets(
    time: Res<Time>,
    mut q: Query<(&EBVelocity, &mut Transform), With<EnemyBullet>>,
) {
    let dt = time.delta_secs();
    for (v, mut t) in &mut q {
        t.translation.x += v.0.x * dt;
        t.translation.y += v.0.y * dt;
    }
}

fn age_and_cull(
    mut commands: Commands,
    time: Res<Time>,
    mut q: Query<(Entity, &Transform, &mut EBLifetime), With<EnemyBullet>>,
) {
    let dt = time.delta_secs();
    let half_w = crate::WINDOW_W as f32 * 0.5 + 16.0;
    let half_h = crate::WINDOW_H as f32 * 0.5 + 16.0;
    for (e, t, mut life) in &mut q {
        life.0 += dt;
        if life.0 > BULLET_LIFETIME
            || t.translation.x.abs() > half_w
            || t.translation.y.abs() > half_h
        {
            commands.entity(e).despawn();
        }
    }
}
