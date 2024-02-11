use std::f32::consts::PI;
use std::time::Duration;

use bevy::math::vec3;
use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use bevy::utils::Instant;
use rand::Rng;

use crate::elements::{Consumable, Food, FoodBundle, Poison, PoisonBundle};
use crate::utils::*;
use crate::*;

pub struct BoidPlugin;

#[derive(Component)]
pub struct Boid;
#[derive(Component)]
pub struct Predator;
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

#[derive(Event)]
struct BoidDeathFoodSpawnEvent(Vec2);

#[derive(Component, Clone, Copy)]
pub struct Dna {
    pub steering_force: f32,
    pub speed: f32,
    pub food_pull: f32,
    pub poison_pull: f32,
    pub predator_pull: f32,
    pub food_perception_radius: f32,
    pub poison_perception_radius: f32,
    pub predator_perception_radius: f32,
}

#[derive(Component, Clone, Copy)]
pub struct PredatorDna {
    pub prey_perception: f32,
    pub prey_pull: f32,
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

#[derive(Bundle)]
pub struct PredatorBundle {
    boid_bundle: BoidBundle,
    predator: Predator,
    predator_dna: PredatorDna,
}

impl Plugin for BoidPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                update_boid_transform,
                boundary_boids_direction_update,
                (update_boid_color, update_predator_color),
                (update_boid_direction, update_predator_direction),
                (handle_boid_collision, handle_predator_collision),
                (boids_replicate, predators_replicate),
                (camera_follow_boid, camera_follow_predator),
                despawn_boids,
                boid_health_tick.run_if(on_timer(Duration::from_secs_f32(0.5))),
                handle_boid_despawn_events,
                boid_separation,
            )
                .run_if(in_state(SimState::Simulating)),
        )
        .add_event::<BoidDeathFoodSpawnEvent>();
    }
}

fn boids_replicate(
    time: Res<Time>,
    mut commands: Commands,
    handle: Res<GlobalTextureHandle>,
    mut boid_query: Query<
        (&mut ReplicateTimer, &Transform, &Dna, &Health),
        (With<Boid>, Without<Predator>),
    >,
) {
    if boid_query.iter().len() >= NUM_BOIDS {
        return;
    }

    let mut rng = rand::thread_rng();
    let replication_threshold = 1.0 - BOID_REPLICATE_PROBABILITY;
    for (mut timer, transform, dna, health) in boid_query.iter_mut() {
        if rng.gen_range(0.0..1.0) <= replication_threshold && health.0 > BOID_MAX_HEALTH / 2.0 {
            continue;
        }
        // No tick until health valid
        if !timer.0.tick(time.delta()).just_finished() {
            continue;
        }
        let (x, y) = (transform.translation.x, transform.translation.y);
        let mut child_dna = dna.clone();
        child_dna.mutate();

        commands.spawn(BoidBundle::child(
            (x, y),
            &child_dna,
            handle.0.clone().unwrap(),
            false,
        ));
    }
}

fn predators_replicate(
    time: Res<Time>,
    mut commands: Commands,
    handle: Res<GlobalTextureHandle>,
    mut predators_query: Query<
        (&mut ReplicateTimer, &Transform, &Dna, &PredatorDna, &Health),
        With<Predator>,
    >,
) {
    if predators_query.iter().len() >= NUM_PREDATORS {
        return;
    }

    let mut rng = rand::thread_rng();
    let replication_threshold = 1.0 - BOID_REPLICATE_PROBABILITY;
    for (mut timer, transform, dna, predator_dna, health) in predators_query.iter_mut() {
        if rng.gen_range(0.0..1.0) <= replication_threshold && health.0 > BOID_MAX_HEALTH / 3.0 {
            continue;
        }
        // No tick until health valid
        if !timer.0.tick(time.delta()).just_finished() {
            continue;
        }
        let (x, y) = (transform.translation.x, transform.translation.y);
        let mut child_dna = dna.clone();
        let mut child_predator_dna = predator_dna.clone();
        child_dna.mutate();
        child_predator_dna.mutate();

        commands.spawn(PredatorBundle::child(
            (x, y),
            &child_dna,
            &predator_dna,
            handle.0.clone().unwrap(),
        ));
    }
}

fn camera_follow_predator(
    predator_query: Query<&Transform, With<Predator>>,
    settings: Res<Settings>,
    mut cam_query: Query<&mut Transform, (With<Camera>, Without<Predator>)>,
) {
    if cam_query.is_empty() {
        return;
    }
    if predator_query.is_empty() {
        return;
    }

    let mut cam_transform = cam_query.single_mut();
    if settings.camera_follow_predator {
        if let Some(transform) = predator_query.iter().next() {
            let mut new_pos = transform.translation.clone();
            new_pos.y -= 50.0;
            cam_transform.translation = cam_transform.translation.lerp(new_pos, 0.05);
        }
    }
}

fn camera_follow_boid(
    boid_query: Query<&Transform, (With<Boid>, Without<Predator>)>,
    settings: Res<Settings>,
    mut cam_query: Query<&mut Transform, (With<Camera>, Without<Boid>)>,
) {
    if cam_query.is_empty() {
        return;
    }
    if boid_query.is_empty() {
        return;
    }

    let mut cam_transform = cam_query.single_mut();
    if settings.camera_follow_boid {
        if let Some(transform) = boid_query.iter().next() {
            let mut new_pos = transform.translation.clone();
            new_pos.y -= 50.0;
            cam_transform.translation = cam_transform.translation.lerp(new_pos, 0.05);
        }
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

fn handle_boid_despawn_events(
    mut commands: Commands,
    handle: Res<GlobalTextureHandle>,
    mut events: EventReader<BoidDeathFoodSpawnEvent>,
) {
    let mut rng = rand::thread_rng();
    for e in events.read() {
        for _ in 0..rng.gen_range(2..5) {
            let (x, y) = (e.0.x, e.0.y);
            let (x, y) = (
                x + rng.gen_range(-30.0..30.0),
                y + rng.gen_range(-30.0..30.0),
            );

            let (x, y) = limit_to_world((x, y));
            if rng.gen_range(0.0..1.0) > 0.85 {
                commands.spawn(PoisonBundle::new((x, y), handle.0.clone().unwrap()));
            } else {
                commands.spawn(FoodBundle::new((x, y), handle.0.clone().unwrap()));
            }
        }
    }
}

fn handle_boid_collision(
    mut commands: Commands,
    mut boids_query: Query<(&mut Health, &Transform), (With<Boid>, Without<Predator>)>,
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

fn handle_predator_collision(
    mut commands: Commands,
    mut writer: EventWriter<BoidDeathFoodSpawnEvent>,
    mut predator_query: Query<(&mut Health, &Transform), With<Predator>>,
    boids_query: Query<(&Transform, Entity), (With<Boid>, Without<Predator>)>,
) {
    let mut boids = Vec::new();
    for (t, e) in boids_query.iter() {
        boids.push((t.translation, e));
    }

    for (mut health, predator_transform) in predator_query.iter_mut() {
        let mut boid_idx = None;
        for (index, (t, e)) in boids.iter().enumerate() {
            if predator_transform.translation.distance_squared(*t)
                < PREDATOR_COLLISION_RADIUS * PREDATOR_COLLISION_RADIUS
            {
                health.0 = (health.0 + BOID_NUTRITION).min(BOID_MAX_HEALTH);
                writer.send(BoidDeathFoodSpawnEvent(t.truncate()));
                boid_idx = Some(index);
                commands.entity(*e).despawn();
                break;
            }
        }

        if let Some(index) = boid_idx {
            boids.remove(index);
        }
    }
}

fn despawn_boids(
    mut commands: Commands,
    mut writer: EventWriter<BoidDeathFoodSpawnEvent>,
    boids_query: Query<(&Health, &Transform, Entity), With<Boid>>,
) {
    for (health, transform, e) in boids_query.iter() {
        if health.0 <= 0.0 {
            commands.entity(e).despawn();
            writer.send(BoidDeathFoodSpawnEvent(transform.translation.truncate()));
        }
    }
}

fn boid_health_tick(mut boids_query: Query<(&mut Health, &Dna), With<Boid>>) {
    for (mut health, dna) in boids_query.iter_mut() {
        health.0 -= BOID_TICK_DAMAGE * dna.speed.max(1.0) * 0.5;
    }
}

fn boundary_boids_direction_update(
    mut boids_query: Query<(&Transform, &Velocity, &mut Acceleration), With<Boid>>,
) {
    if boids_query.is_empty() {
        return;
    }

    for (transform, velocity, mut acceleration) in boids_query.iter_mut() {
        // Boundary collisions
        let (x, y) = (transform.translation.x, transform.translation.y);
        let (min_x, min_y, max_x, max_y) = get_world_bounds();
        if x <= min_x || x >= max_x || y <= min_y || y >= max_y {
            // Don't randomize direction here, boid will spin around
            acceleration.0 +=
                get_steering_force(Vec2::ZERO, transform.translation.truncate(), velocity.0)
                    * 0.001;
        }
    }
}

fn update_predator_direction(
    mut predator_query: Query<
        (&Transform, &Velocity, &mut Acceleration, &Dna, &PredatorDna),
        With<Predator>,
    >,
    boids_query: Query<&Transform, (With<Boid>, Without<Predator>)>,
) {
    if predator_query.is_empty() || boids_query.is_empty() {
        return;
    }

    for (predator_transform, velocity, mut acceleration, dna, predator_dna) in
        predator_query.iter_mut()
    {
        let mut closest_boid_dist = f32::INFINITY;
        let mut closest_boid_pos = None;
        for boid_transform in boids_query.iter() {
            let dist = predator_transform
                .translation
                .distance_squared(boid_transform.translation);
            if dist <= closest_boid_dist
                && dist <= predator_dna.prey_perception * predator_dna.prey_perception
            {
                closest_boid_dist = dist;
                closest_boid_pos = Some(boid_transform.translation.truncate());
            }
        }

        if let Some(boid_pos) = closest_boid_pos {
            acceleration.0 += get_steering_force(
                boid_pos,
                predator_transform.translation.truncate(),
                velocity.0,
            ) * dna.steering_force.abs()
                * predator_dna.prey_pull;
        }
    }
}

fn boid_separation(mut boids_query: Query<(&Transform, &Velocity, &mut Acceleration), With<Boid>>) {
    if boids_query.is_empty() {
        return;
    }

    let all_positions: Vec<Vec2> = boids_query
        .into_iter()
        .map(|(t, _, _)| t.translation.truncate())
        .collect();

    for (transform, velocity, mut acceleration) in boids_query.iter_mut() {
        let boid_pos = transform.translation.truncate();
        for neighbour_pos in all_positions.iter() {
            let dist = neighbour_pos.distance_squared(boid_pos);
            if dist != 0.0 && dist <= 20.0 {
                acceleration.0 += get_steering_force(*neighbour_pos, boid_pos, velocity.0) * -0.01;
            }
        }
    }
}

fn update_boid_direction(
    mut boids_query: Query<
        (&Transform, &Velocity, &mut Acceleration, &Dna),
        (With<Boid>, Without<Predator>),
    >,
    predator_query: Query<&Transform, With<Predator>>,
    food_query: Query<&Transform, With<Food>>,
    poison_query: Query<&Transform, With<Poison>>,
) {
    if boids_query.is_empty() {
        return;
    }

    for (transform, velocity, mut acceleration, dna) in boids_query.iter_mut() {
        let closest_food = get_closest_item(
            transform.translation,
            dna.food_perception_radius,
            food_query.iter(),
        );
        let closest_poison = get_closest_item(
            transform.translation,
            dna.poison_perception_radius,
            poison_query.iter(),
        );
        let closest_predator = get_closest_item(
            transform.translation,
            dna.predator_perception_radius,
            predator_query.iter(),
        );

        if let Some(food) = closest_food {
            acceleration.0 +=
                get_steering_force(food, transform.translation.truncate(), velocity.0)
                    * dna.steering_force.abs()
                    * dna.food_pull;
        }
        if let Some(poison) = closest_poison {
            acceleration.0 +=
                get_steering_force(poison, transform.translation.truncate(), velocity.0)
                    * dna.steering_force.abs()
                    * dna.poison_pull;
        }
        if let Some(predator) = closest_predator {
            acceleration.0 +=
                get_steering_force(predator, transform.translation.truncate(), velocity.0)
                    * dna.steering_force.abs()
                    * dna.predator_pull;
        }
    }
}

fn get_closest_item<'a, I>(origin: Vec3, range: f32, items: I) -> Option<Vec2>
where
    I: IntoIterator<Item = &'a Transform>,
{
    let mut closest_item_dist = f32::INFINITY;
    let mut closest_item_pos = None;

    for t in items {
        let dist = t.translation.distance_squared(origin);
        if dist <= closest_item_dist && dist <= range * range {
            closest_item_dist = dist;
            closest_item_pos = Some(t.translation.truncate());
        }
    }

    closest_item_pos
}

fn update_boid_color(
    mut boids_query: Query<(&mut TextureAtlasSprite, &Health), (With<Boid>, Without<Predator>)>,
) {
    for (mut sprite, health) in boids_query.iter_mut() {
        sprite.color = Color::from(
            Vec4::from(get_color(COLOR_BOID_LOW_HEALTH))
                .lerp(Vec4::from(get_color(COLOR_BOID)), health.0 / 100.0),
        );
    }
}

fn update_predator_color(
    mut predator_query: Query<(&mut TextureAtlasSprite, &Health), With<Predator>>,
) {
    for (mut sprite, health) in predator_query.iter_mut() {
        sprite.color = Color::from(
            Vec4::from(get_color(COLOR_PREDATOR_LOW_HEALTH))
                .lerp(Vec4::from(get_color(COLOR_PREDATOR)), health.0 / 100.0),
        );
    }
}

impl BoidBundle {
    pub fn new(handle: Handle<TextureAtlas>, is_predator: bool) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(-WORLD_W..WORLD_W);
        let y = rng.gen_range(-WORLD_H..WORLD_H);

        Self::child((x, y), &Dna::rand(), handle, is_predator)
    }

    fn child(pos: (f32, f32), dna: &Dna, handle: Handle<TextureAtlas>, is_predator: bool) -> Self {
        let (x, y) = pos;
        let sprite_index = if is_predator { 2 } else { 0 };
        Self {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: handle,
                sprite: TextureAtlasSprite::new(sprite_index),
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

impl PredatorBundle {
    pub fn new(handle: Handle<TextureAtlas>) -> Self {
        Self {
            boid_bundle: BoidBundle::new(handle, true),
            predator: Predator,
            predator_dna: PredatorDna::rand(),
        }
    }

    fn child(
        pos: (f32, f32),
        dna: &Dna,
        predator_dna: &PredatorDna,
        handle: Handle<TextureAtlas>,
    ) -> Self {
        Self {
            boid_bundle: BoidBundle::child(pos, dna, handle, true),
            predator: Predator,
            predator_dna: predator_dna.clone(),
        }
    }
}

impl Dna {
    fn rand() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            steering_force: rng.gen_range(0.0005..0.005),
            speed: rng.gen_range(0.5..1.5),
            food_pull: rng.gen_range(-1.5..1.5),
            poison_pull: rng.gen_range(-1.5..1.5),
            predator_pull: rng.gen_range(-1.5..1.5),
            food_perception_radius: rng.gen_range(50.0..120.0),
            poison_perception_radius: rng.gen_range(50.0..120.0),
            predator_perception_radius: rng.gen_range(50.0..120.0),
        }
    }

    fn get_rand_mutation_delta() -> f32 {
        let mut rng = rand::thread_rng();
        if rng.gen_range(0.0..1.0) > BOID_MUTATION_THRESHOLD {
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
        self.predator_pull += Self::get_rand_mutation_delta();
        self.food_perception_radius += Self::get_rand_mutation_delta() * 50.0;
        self.poison_perception_radius += Self::get_rand_mutation_delta() * 50.0;
    }
}

impl PredatorDna {
    fn rand() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            prey_perception: rng.gen_range(30.0..100.0),
            prey_pull: rng.gen_range(-1.0..1.0),
        }
    }

    fn mutate(&mut self) {
        self.prey_perception += Dna::get_rand_mutation_delta() * 50.0;
        self.prey_pull += Dna::get_rand_mutation_delta();
    }
}
