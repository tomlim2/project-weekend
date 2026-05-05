use bevy::prelude::*;

use crate::player::Player;
use crate::sfx::{PlaySfx, SfxKind};
use crate::GameState;

const FIRE_COOLDOWN: f32 = 0.1; // 10 shots/sec
const BULLET_SPEED: f32 = 720.0;
const BULLET_LIFETIME: f32 = 2.0;
const BULLET_W: f32 = 3.0;
const BULLET_H: f32 = 10.0;
const SPREAD_NORMAL_DEG: f32 = 7.0; // half-spread for the fan
const FOCUS_PARALLEL_OFFSET: f32 = 4.0; // px sideways from player center, focus mode

#[derive(Component)]
pub struct AutoFire {
    pub since_last: f32,
}

impl Default for AutoFire {
    fn default() -> Self {
        Self {
            since_last: FIRE_COOLDOWN,
        }
    }
}

#[derive(Component)]
pub struct PlayerBullet;

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Lifetime(pub f32);

pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (auto_fire, move_bullets, age_bullets, despawn_offscreen)
                .run_if(in_state(GameState::Playing)),
        );
    }
}

fn auto_fire(
    time: Res<Time>,
    mut commands: Commands,
    mut sfx: MessageWriter<PlaySfx>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut q: Query<(&Player, &Transform, &mut AutoFire)>,
) {
    let Ok((player, transform, mut fire)) = q.single_mut() else {
        return;
    };

    fire.since_last += time.delta_secs();
    if fire.since_last < FIRE_COOLDOWN {
        return;
    }
    fire.since_last = 0.0;
    sfx.write(PlaySfx(SfxKind::Shot));

    let origin = transform.translation.truncate() + Vec2::new(0.0, 12.0);
    let mesh = meshes.add(Rectangle::new(BULLET_W, BULLET_H));
    let mat = materials.add(Color::srgb(0.95, 0.97, 0.6));

    if player.focused {
        // 2 parallel narrow shots
        for sign in [-1.0_f32, 1.0] {
            spawn_bullet(
                &mut commands,
                origin + Vec2::new(sign * FOCUS_PARALLEL_OFFSET, 0.0),
                Vec2::new(0.0, BULLET_SPEED),
                mesh.clone(),
                mat.clone(),
            );
        }
    } else {
        // 3-shot fan, half-spread SPREAD_NORMAL_DEG
        let spread = SPREAD_NORMAL_DEG.to_radians();
        for angle in [-spread, 0.0, spread] {
            let v = Vec2::new(angle.sin(), angle.cos()) * BULLET_SPEED;
            spawn_bullet(&mut commands, origin, v, mesh.clone(), mat.clone());
        }
    }
}

fn spawn_bullet(
    commands: &mut Commands,
    origin: Vec2,
    velocity: Vec2,
    mesh: Handle<Mesh>,
    material: Handle<ColorMaterial>,
) {
    commands.spawn((
        PlayerBullet,
        Velocity(velocity),
        Lifetime(0.0),
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(origin.x, origin.y, 0.0),
    ));
}

fn move_bullets(time: Res<Time>, mut q: Query<(&Velocity, &mut Transform), With<PlayerBullet>>) {
    let dt = time.delta_secs();
    for (v, mut t) in &mut q {
        t.translation.x += v.0.x * dt;
        t.translation.y += v.0.y * dt;
    }
}

fn age_bullets(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut Lifetime), With<PlayerBullet>>,
) {
    let dt = time.delta_secs();
    for (e, mut life) in &mut q {
        life.0 += dt;
        if life.0 > BULLET_LIFETIME {
            commands.entity(e).despawn();
        }
    }
}

fn despawn_offscreen(
    mut commands: Commands,
    q: Query<(Entity, &Transform), With<PlayerBullet>>,
) {
    let half_w = crate::WINDOW_W as f32 * 0.5 + 32.0;
    let half_h = crate::WINDOW_H as f32 * 0.5 + 32.0;
    for (e, t) in &q {
        if t.translation.x.abs() > half_w || t.translation.y.abs() > half_h {
            commands.entity(e).despawn();
        }
    }
}
