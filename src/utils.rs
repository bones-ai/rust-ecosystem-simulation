use std::f32::consts::PI;

use bevy::{
    math::{vec2, vec3},
    prelude::*,
};
use rand::Rng;

use crate::*;

pub struct LimitedVec<T> {
    pub items: Vec<T>,
    pub max_size: usize,
}

pub fn get_rand_unit_vec3() -> Vec3 {
    let mut rng = rand::thread_rng();
    let x = rng.gen_range(-100.0..100.0);
    let y = rng.gen_range(-100.0..100.0);

    vec3(x, y, 0.0).normalize()
}

pub fn get_rand_unit_vec2() -> Vec2 {
    let rand_vec3 = get_rand_unit_vec3();
    vec2(rand_vec3.x, rand_vec3.y)
}

pub fn calc_rotation_angle(v1: Vec3, v2: Vec3) -> f32 {
    let dx = v1.x - v2.x;
    let dy = v1.y - v2.y;

    // Calculate the angle using arctangent (atan2) function
    let mut angle_rad = dy.atan2(dx);

    // Ensure the angle is within [0, 2*PI) range
    if angle_rad < 0.0 {
        angle_rad += 2.0 * PI;
    }
    angle_rad
}

pub fn get_world_bounds() -> (f32, f32, f32, f32) {
    let min_x = -WORLD_W;
    let min_y = -WORLD_H;
    let max_x = WORLD_W;
    let max_y = WORLD_H;

    (min_x, min_y, max_x, max_y)
}

pub fn get_steering_force(target: Vec2, pos: Vec2, velocity: Vec2) -> Vec2 {
    let desired = target - pos;
    desired - velocity
}

impl<T> LimitedVec<T> {
    pub fn new() -> Self {
        LimitedVec {
            items: Vec::new(),
            max_size: MAX_NUM_POINTS,
        }
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
        if self.items.len() > self.max_size {
            self.items.remove(0);
        }
    }
}
