use std::f32::consts::PI;
use std::time::Duration;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::Instant;
use rand::Rng;

use crate::utils::*;
use crate::world::{Consumable, Food, Poison};
use crate::*;

pub struct BoidPlugin;

#[derive(Component)]
pub struct Boid;
#[derive(Component)]
struct Velocity(Vec2);
#[derive(Component)]
struct Acceleration(Vec2);
#[derive(Component)]
struct Health(f32);
#[derive(Component)]
struct ReplicateTimer(Timer);
#[derive(Component)]
pub struct BirthTimeStamp(pub Instant);
#[derive(Component, Clone, Copy)]
pub struct Dna {
    pub steering_force: f32,
    pub speed: f32,
    pub food_pull: f32,
    pub poison_pull: f32,
    pub food_perception_radius: f32,
    pub poison_perception_radius: f32,
}

#[derive(Bundle)]
pub struct BoidBundle {
    sprite_sheet_bundle: SpriteSheetBundle,
    boid: Boid,
    velocity: Velocity,
    acceleration: Acceleration,
    dna: Dna,
    health: Health,
    replicate_timer: ReplicateTimer,
    birth_ts: BirthTimeStamp,
}

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                update_boid_color,
                update_boid_transform,
                update_boid_direction,
                handle_boid_collision,
                boids_replicate,
                despawn_boids,
                camera_follow_boid,
                boid_health_tick.run_if(on_timer(Duration::from_secs_f32(0.5))),
            )
                .run_if(in_state(SimState::Simulating)),
        );
    }
}

fn boids_replicate(
    time: Res<Time>,
    mut commands: Commands,
    handle: Res<GlobalTextureHandle>,
    mut boid_query: Query<(&mut ReplicateTimer, &Transform, &Dna, &Health), With<Boid>>,
) {
    let mut rng = rand::thread_rng();
    if boid_query.iter().len() >= NUM_BOIDS {
        return;
    }

    for (mut timer, transform, dna, health) in boid_query.iter_mut() {
        if !timer.0.tick(time.delta()).just_finished() {
            continue;
        }
        if rng.gen_range(0.0..1.0) <= 0.7 && health.0 > BOID_MAX_HEALTH / 2.0 {
            continue;
        }
        let (x, y) = (transform.translation.x, transform.translation.y);
        let mut child_dna = dna.clone();
        child_dna.mutate();

        commands.spawn(BoidBundle::child(
            (x, y),
            &child_dna,
            handle.0.clone().unwrap(),
        ));
    }
}

fn camera_follow_boid(
    boid_query: Query<&Transform, With<Boid>>,
    settings: Res<Settings>,
    mut cam_query: Query<&mut Transform, (With<Camera>, Without<Boid>)>,
) {
    if cam_query.is_empty() || boid_query.is_empty() {
        return;
    }

    let mut cam_transform = cam_query.single_mut();
    if settings.enable_camera_follow {
        if let Some(transform) = boid_query.iter().next() {
            let mut new_pos = transform.translation.clone();
            new_pos.y -= 50.0;
            cam_transform.translation = cam_transform.translation.lerp(new_pos, 0.05);
        }
    } else {
        cam_transform.translation = cam_transform.translation.lerp(Vec3::ZERO, 0.05);
    }
}

fn update_boid_transform(
    mut boids_query: Query<(&mut Transform, &mut Velocity, &mut Acceleration, &Dna), With<Boid>>,
) {
    if boids_query.is_empty() {
        return;
    }

    for (mut transform, mut velocity, mut acceleration, dna) in boids_query.iter_mut() {
        let old_pos = transform.translation;
        if acceleration.0.is_nan() {
            acceleration.0 = Vec2::ZERO;
            continue;
        }

        velocity.0 = (velocity.0 + acceleration.0).normalize();
        acceleration.0 = Vec2::ZERO;

        let new_translation =
            transform.translation + vec3(velocity.0.x, velocity.0.y, 0.0) * dna.speed;
        if !new_translation.is_nan() {
            transform.translation = new_translation;
        }

        transform.rotation =
            Quat::from_rotation_z(calc_rotation_angle(old_pos, transform.translation) + PI / 2.0);
    }
}

fn handle_boid_collision(
    mut commands: Commands,
    mut boids_query: Query<(&mut Health, &Transform), With<Boid>>,
    consumable_query: Query<(&Transform, &Consumable, Entity), With<Consumable>>,
) {
    let mut consumables = Vec::new();
    for (t, c, e) in consumable_query.iter() {
        consumables.push((t.translation, c.0, e));
    }

    for (mut health, boid_transform) in boids_query.iter_mut() {
        let mut food_idx = None;
        for (index, (t, c, e)) in consumables.iter().enumerate() {
            if boid_transform.translation.distance_squared(*t)
                < BOID_COLLISION_RADIUS * BOID_COLLISION_RADIUS
            {
                health.0 = (health.0 + c).min(BOID_MAX_HEALTH);
                commands.entity(*e).despawn();
                food_idx = Some(index);
                break;
            }
        }
        if let Some(index) = food_idx {
            consumables.remove(index);
        }
    }
}

fn despawn_boids(mut commands: Commands, boids_query: Query<(&Health, Entity), With<Boid>>) {
    for (health, e) in boids_query.iter() {
        if health.0 <= 0.0 {
            commands.entity(e).despawn();
        }
    }
}

fn boid_health_tick(mut boids_query: Query<(&mut Health, &Dna), With<Boid>>) {
    for (mut health, dna) in boids_query.iter_mut() {
        health.0 -= BOID_TICK_DAMAGE * dna.speed.max(1.0) * 0.5;
    }
}

fn update_boid_direction(
    mut boids_query: Query<(&Transform, &Velocity, &mut Acceleration, &Dna), With<Boid>>,
    food_query: Query<&Transform, With<Food>>,
    poison_query: Query<&Transform, With<Poison>>,
) {
    if boids_query.is_empty() {
        return;
    }

    for (transform, velocity, mut acceleration, dna) in boids_query.iter_mut() {
        // Boundary collisions
        let (x, y) = (transform.translation.x, transform.translation.y);
        let (min_x, min_y, max_x, max_y) = get_world_bounds();
        if x <= min_x || x >= max_x || y <= min_y || y >= max_y {
            // Don't randomize direction here, boid will spin around
            acceleration.0 +=
                get_steering_force(Vec2::ZERO, transform.translation.truncate(), velocity.0)
                    * 0.001;
            continue;
        }

        let mut closest_food_dist = f32::INFINITY;
        let mut closest_food_pos = None;
        let mut closest_poison_dist = f32::INFINITY;
        let mut closest_poison_pos = None;
        for f in food_query.iter() {
            let dist_to_food = transform.translation.distance_squared(f.translation);
            if dist_to_food <= closest_food_dist
                && dist_to_food <= dna.food_perception_radius * dna.food_perception_radius
            {
                closest_food_dist = dist_to_food;
                closest_food_pos = Some(f.translation.truncate());
            }
        }
        for f in poison_query.iter() {
            let dist_to_poison = transform.translation.distance_squared(f.translation);
            if dist_to_poison <= closest_poison_dist
                && dist_to_poison <= dna.poison_perception_radius * dna.poison_perception_radius
            {
                closest_poison_dist = dist_to_poison;
                closest_poison_pos = Some(f.translation.truncate());
            }
        }

        if let Some(food) = closest_food_pos {
            acceleration.0 +=
                get_steering_force(food, transform.translation.truncate(), velocity.0)
                    * dna.steering_force.abs()
                    * dna.food_pull;
        }
        if let Some(poison) = closest_poison_pos {
            acceleration.0 +=
                get_steering_force(poison, transform.translation.truncate(), velocity.0)
                    * dna.steering_force.abs()
                    * dna.poison_pull;
        }
    }
}

fn update_boid_color(mut boids_query: Query<(&mut TextureAtlasSprite, &Health), With<Boid>>) {
    for (mut sprite, health) in boids_query.iter_mut() {
        sprite.color =
            Color::from(Vec4::from(Color::CYAN).lerp(Vec4::from(Color::WHITE), health.0 / 100.0));
    }
}

impl BoidBundle {
    pub fn new(handle: Handle<TextureAtlas>) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-WORLD_W..WORLD_W);
        let y = rng.gen_range(-WORLD_H..WORLD_H);

        Self::child((x, y), &Dna::rand(), handle)
    }

    pub fn child(pos: (f32, f32), dna: &Dna, handle: Handle<TextureAtlas>) -> Self {
        let (x, y) = pos;
        Self {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: handle,
                sprite: TextureAtlasSprite::new(0),
                transform: Transform::from_scale(Vec3::splat(BOID_SPRITE_SCALE))
                    .with_translation(vec3(x, y, 2.0)),
                ..default()
            },
            boid: Boid,
            velocity: Velocity(get_rand_unit_vec2()),
            acceleration: Acceleration(Vec2::ZERO),
            dna: dna.clone(),
            health: Health(BOID_MAX_HEALTH),
            replicate_timer: ReplicateTimer(Timer::new(
                Duration::from_secs_f32(BOID_REPLICATE_INTERVAL),
                TimerMode::Repeating,
            )),
            birth_ts: BirthTimeStamp(Instant::now()),
        }
    }
}

impl Dna {
    fn rand() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            steering_force: rng.gen_range(0.0001..0.005),
            speed: rng.gen_range(0.5..1.5),
            food_pull: rng.gen_range(-1.5..1.5),
            poison_pull: rng.gen_range(-1.5..1.5),
            food_perception_radius: rng.gen_range(10.0..100.0),
            poison_perception_radius: rng.gen_range(10.0..100.0),
        }
    }

    fn get_rand_mutation_delta() -> f32 {
        let mut rng = rand::thread_rng();
        if rng.gen_range(0.0..1.0) > 0.9 {
            if rng.gen_range(0.0..1.0) > 0.5 {
                0.1
            } else {
                -0.1
            }
        } else {
            0.0
        }
    }

    fn mutate(&mut self) {
        self.steering_force += Self::get_rand_mutation_delta() * 0.01;
        self.speed += Self::get_rand_mutation_delta();
        self.food_pull += Self::get_rand_mutation_delta();
        self.poison_pull += Self::get_rand_mutation_delta();
        self.food_perception_radius += Self::get_rand_mutation_delta() * 50.0;
        self.poison_perception_radius += Self::get_rand_mutation_delta() * 50.0;
    }
}
