use std::time::Duration;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use rand::Rng;

use crate::boid::{Boid, BoidBundle};
use crate::SimState;
use crate::*;

pub struct WorldPlugin;

#[derive(Component)]
pub struct Food;
#[derive(Component)]
pub struct Poison;
#[derive(Component)]
pub struct Consumable(pub f32);

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            OnEnter(SimState::InitSim),
            (populate_boids, populate_consumables, start_simulation),
        )
        .add_systems(
            Update,
            (
                populate_consumables.run_if(on_timer(Duration::from_secs_f32(5.0))),
                populate_boids.run_if(on_timer(Duration::from_secs_f32(5.0))),
            )
                .run_if(in_state(SimState::Simulating)),
        );
    }
}

fn populate_boids(
    mut commands: Commands,
    handle: Res<GlobalTextureHandle>,
    boid_query: Query<With<Boid>>,
) {
    let num_boids = boid_query.iter().len();
    if num_boids > 0 {
        return;
    }

    for _ in 0..(NUM_BOIDS - num_boids) {
        commands.spawn(BoidBundle::new(handle.0.clone().unwrap()));
    }
}

fn populate_consumables(
    mut commands: Commands,
    handle: Res<GlobalTextureHandle>,
    food_query: Query<With<Food>>,
    poison_query: Query<With<Poison>>,
) {
    let mut rng = rand::thread_rng();
    let num_food = food_query.iter().len();
    let num_poison = poison_query.iter().len();

    if num_food < NUM_FOOD / 2 {
        for _ in 0..(NUM_FOOD - num_food) {
            let x = rng.gen_range(-WORLD_W..WORLD_W);
            let y = rng.gen_range(-WORLD_H..WORLD_H);
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: handle.0.clone().unwrap(),
                    sprite: TextureAtlasSprite {
                        index: 1,
                        color: Color::rgb_u8(COLOR_FOOD.0, COLOR_FOOD.1, COLOR_FOOD.2),
                        ..default()
                    },
                    transform: Transform::from_scale(Vec3::splat(CONSUMABLE_SPRITE_SCALE))
                        .with_translation(vec3(x, y, 2.0)),
                    ..default()
                },
                Consumable(FOOD_NUTRITION),
                Food,
            ));
        }
    }

    if num_poison < NUM_POISON / 2 {
        for _ in 0..(NUM_POISON - num_poison) {
            let x = rng.gen_range(-WORLD_W..WORLD_W);
            let y = rng.gen_range(-WORLD_H..WORLD_H);
            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: handle.0.clone().unwrap(),
                    sprite: TextureAtlasSprite {
                        index: 1,
                        color: Color::rgb_u8(COLOR_POISON.0, COLOR_POISON.1, COLOR_POISON.2),
                        ..default()
                    },
                    transform: Transform::from_scale(Vec3::splat(CONSUMABLE_SPRITE_SCALE))
                        .with_translation(vec3(x, y, 2.0)),
                    ..default()
                },
                Consumable(POISON_DAMAGE),
                Poison,
            ));
        }
    }
}

fn start_simulation(mut next_state: ResMut<NextState<SimState>>) {
    next_state.set(SimState::Simulating);
}
