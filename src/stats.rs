use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

use crate::boid::{BirthTimeStamp, Boid, Dna, Predator, PredatorDna};
use crate::elements::{Food, Poison};
use crate::utils::LimitedVec;
use crate::*;

pub struct StatsPlugin;

#[derive(Resource)]
pub struct SimulationStats {
    pub num_boids: LimitedVec<f32>,
    pub num_predators: LimitedVec<f32>,
    pub num_food: LimitedVec<f32>,
    pub num_poison: LimitedVec<f32>,
    pub avg_lifespan: LimitedVec<f32>,
    pub avg_predator_lifespan: LimitedVec<f32>,
    pub food_perception: LimitedVec<f32>,
    pub poison_perception: LimitedVec<f32>,
    pub predator_perception: LimitedVec<f32>,
    pub prey_perception: LimitedVec<f32>,
    pub speed: LimitedVec<f32>,
    pub predator_speed: LimitedVec<f32>,
    pub food_affinity: LimitedVec<f32>,
    pub poison_affinity: LimitedVec<f32>,
    pub predator_affinity: LimitedVec<f32>,
    pub prey_affinity: LimitedVec<f32>,
    pub steering_force: LimitedVec<f32>,
}

impl Plugin for StatsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SimulationStats::new()).add_systems(
            Update,
            update_stats
                .run_if(in_state(SimState::Simulating))
                .run_if(on_timer(Duration::from_secs_f32(STAT_COLLECTION_RATE))),
        );
    }
}

fn update_stats(
    mut stats: ResMut<SimulationStats>,
    boid_query: Query<(&Dna, &BirthTimeStamp), (With<Boid>, Without<Predator>)>,
    predators_query: Query<(&Dna, &PredatorDna, &BirthTimeStamp), With<Predator>>,
    food_query: Query<With<Food>>,
    poison_query: Query<With<Poison>>,
) {
    let num_boids = boid_query.iter().len() as f32;
    let num_predators = predators_query.iter().len() as f32;
    let num_food = food_query.iter().len() as f32;
    let num_poison = poison_query.iter().len() as f32;

    let mut food_perception_radius = 0.0;
    let mut poison_perception_radius = 0.0;
    let mut predator_perception_radius = 0.0;
    let mut food_affinity = 0.0;
    let mut poison_affinity = 0.0;
    let mut predator_affinity = 0.0;
    let mut speed = 0.0;
    let mut steering_force = 0.0;
    let mut avg_lifespan = 0.0;

    let mut avg_predator_lifespan = 0.0;
    let mut predator_speed = 0.0;
    let mut predator_boid_affinity = 0.0;
    let mut prey_perception_radius = 0.0;

    for (dna, birth_ts) in boid_query.iter() {
        food_perception_radius += dna.food_perception_radius;
        poison_perception_radius += dna.poison_perception_radius;
        predator_perception_radius += dna.predator_perception_radius;
        food_affinity += dna.food_pull;
        poison_affinity += dna.poison_pull;
        predator_affinity += dna.predator_pull;
        speed += dna.speed;
        steering_force += dna.steering_force;
        avg_lifespan += birth_ts.0.elapsed().as_secs_f32();
    }

    for (dna, pred_dna, birth_ts) in predators_query.iter() {
        avg_predator_lifespan += birth_ts.0.elapsed().as_secs_f32();
        predator_speed += dna.speed;
        predator_boid_affinity += pred_dna.prey_pull;
        prey_perception_radius += pred_dna.prey_perception;
    }

    stats.num_boids.push(num_boids / NUM_BOIDS as f32);
    stats
        .num_predators
        .push(num_predators / NUM_PREDATORS as f32);
    stats.num_food.push(num_food / NUM_FOOD as f32);
    stats.num_poison.push(num_poison / NUM_POISON as f32);
    stats
        .food_perception
        .push(food_perception_radius / num_boids);
    stats
        .poison_perception
        .push(poison_perception_radius / num_boids);
    stats
        .predator_perception
        .push(predator_perception_radius / num_boids);
    stats
        .prey_perception
        .push(prey_perception_radius / num_predators);
    stats.food_affinity.push(food_affinity / num_boids);
    stats.steering_force.push(steering_force / num_boids);
    stats.poison_affinity.push(poison_affinity / num_boids);
    stats.predator_affinity.push(predator_affinity / num_boids);
    stats
        .prey_affinity
        .push(predator_boid_affinity / num_predators);
    stats.speed.push(speed / num_boids);
    stats.predator_speed.push(predator_speed / num_predators);
    stats.avg_lifespan.push(avg_lifespan / num_boids);
    stats
        .avg_predator_lifespan
        .push(avg_predator_lifespan / num_predators);
}

impl SimulationStats {
    fn new() -> Self {
        Self {
            num_boids: LimitedVec::new(),
            num_predators: LimitedVec::new(),
            num_food: LimitedVec::new(),
            num_poison: LimitedVec::new(),
            avg_lifespan: LimitedVec::new(),
            avg_predator_lifespan: LimitedVec::new(),
            food_perception: LimitedVec::new(),
            poison_perception: LimitedVec::new(),
            predator_perception: LimitedVec::new(),
            prey_perception: LimitedVec::new(),
            speed: LimitedVec::new(),
            predator_speed: LimitedVec::new(),
            food_affinity: LimitedVec::new(),
            poison_affinity: LimitedVec::new(),
            predator_affinity: LimitedVec::new(),
            prey_affinity: LimitedVec::new(),
            steering_force: LimitedVec::new(),
        }
    }
}
