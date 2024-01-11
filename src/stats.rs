use std::time::Duration;

use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;

use crate::boid::{BirthTimeStamp, Boid, Dna};
use crate::utils::LimitedVec;
use crate::*;

pub struct StatsPlugin;

#[derive(Resource)]
pub struct SimulationStats {
    pub num_boids: LimitedVec<usize>,
    pub avg_lifespan: LimitedVec<f32>,
    pub food_perception: LimitedVec<f32>,
    pub poison_perception: LimitedVec<f32>,
    pub speed: LimitedVec<f32>,
    pub food_affinity: LimitedVec<f32>,
    pub poison_affinity: LimitedVec<f32>,
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
    boid_query: Query<(&Dna, &BirthTimeStamp), With<Boid>>,
) {
    let num_boids = boid_query.iter().len();
    let mut food_perception_radius = 0.0;
    let mut poison_perception_radius = 0.0;
    let mut food_affinity = 0.0;
    let mut poison_affinity = 0.0;
    let mut speed = 0.0;
    let mut steering_force = 0.0;
    let mut avg_lifespan = 0.0;

    for (dna, birth_ts) in boid_query.iter() {
        food_perception_radius += dna.food_perception_radius;
        poison_perception_radius += dna.poison_perception_radius;
        food_affinity += dna.food_pull;
        poison_affinity += dna.poison_pull;
        speed += dna.speed;
        steering_force += dna.steering_force;
        avg_lifespan += birth_ts.0.elapsed().as_secs_f32();
    }

    stats
        .food_perception
        .push(food_perception_radius / num_boids as f32);
    stats
        .poison_perception
        .push(poison_perception_radius / num_boids as f32);
    stats.food_affinity.push(food_affinity / num_boids as f32);
    stats.steering_force.push(steering_force / num_boids as f32);
    stats
        .poison_affinity
        .push(poison_affinity / num_boids as f32);
    stats.speed.push(speed / num_boids as f32);
    stats.num_boids.push(num_boids);
    stats.avg_lifespan.push(avg_lifespan);
}

impl SimulationStats {
    fn new() -> Self {
        Self {
            num_boids: LimitedVec::new(),
            avg_lifespan: LimitedVec::new(),
            food_perception: LimitedVec::new(),
            poison_perception: LimitedVec::new(),
            speed: LimitedVec::new(),
            food_affinity: LimitedVec::new(),
            poison_affinity: LimitedVec::new(),
            steering_force: LimitedVec::new(),
        }
    }
}
