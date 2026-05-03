use bevy::prelude::*;

use crate::enemy::{Enemy, Hp};
use crate::weapon::PlayerBullet;
use crate::GameState;

const BULLET_HIT_RADIUS: f32 = 5.0;
const ENEMY_HIT_RADIUS: f32 = 10.0;
const SCORE_PER_KILL: u32 = 100;

#[derive(Resource, Default)]
pub struct Score {
    pub kills: u32,
    pub score: u32,
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>().add_systems(
            Update,
            bullet_enemy_collision.run_if(in_state(GameState::Playing)),
        );
    }
}

fn bullet_enemy_collision(
    mut commands: Commands,
    mut score: ResMut<Score>,
    bullets: Query<(Entity, &Transform), With<PlayerBullet>>,
    mut enemies: Query<(Entity, &Transform, &mut Hp), With<Enemy>>,
) {
    let r2 = (BULLET_HIT_RADIUS + ENEMY_HIT_RADIUS).powi(2);
    let mut killed = std::collections::HashSet::<Entity>::new();
    let mut consumed = std::collections::HashSet::<Entity>::new();

    for (b_entity, b_t) in &bullets {
        if consumed.contains(&b_entity) {
            continue;
        }
        for (e_entity, e_t, mut hp) in &mut enemies {
            if killed.contains(&e_entity) {
                continue;
            }
            let dx = b_t.translation.x - e_t.translation.x;
            let dy = b_t.translation.y - e_t.translation.y;
            if dx * dx + dy * dy <= r2 {
                hp.0 -= 1;
                consumed.insert(b_entity);
                if hp.0 <= 0 {
                    killed.insert(e_entity);
                    score.kills += 1;
                    score.score += SCORE_PER_KILL;
                }
                break;
            }
        }
    }

    for e in consumed {
        commands.entity(e).despawn();
    }
    // Enemies with hp <= 0 will be despawned by enemy.rs::despawn_dead_enemies
}
