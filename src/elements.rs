use std::time::Duration;

use bevy::{math::vec3, prelude::*, time::common_conditions::on_timer, utils::Instant};
use rand::Rng;

use crate::*;

use self::utils::{get_color, limit_to_world};

pub struct ElementsPlugin;

#[derive(Component)]
pub struct Consumable(pub f32);
#[derive(Component)]
pub struct Food;
#[derive(Component)]
pub struct Poison;
#[derive(Component)]
struct LastReplicationTs(Instant);

#[derive(Bundle)]
struct ConsumableBundle {
    sprite_sheet_bundle: SpriteSheetBundle,
    last_replication_ts: LastReplicationTs,
    consumable: Consumable,
}
#[derive(Bundle)]
pub struct FoodBundle {
    food: Food,
    consumable_bundle: ConsumableBundle,
}
#[derive(Bundle)]
pub struct PoisonBundle {
    poison: Poison,
    consumable_bundle: ConsumableBundle,
}

impl Plugin for ElementsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                replicate_consumables.run_if(on_timer(Duration::from_secs_f32(
                    CONSUMABLE_REPLICATION_RATE,
                ))),
                decay_consumables.run_if(on_timer(Duration::from_secs_f32(CONSUMABLE_DECAY_RATE))),
                despawn_consumables,
                update_color,
            )
                .run_if(in_state(SimState::Simulating)),
        );
    }
}

fn update_color(
    mut consumable_query: Query<(&mut TextureAtlasSprite, &Consumable), With<Consumable>>,
) {
    for (mut sprite, c) in consumable_query.iter_mut() {
        sprite.color = sprite.color.with_a(c.opacity());
    }
}

fn decay_consumables(mut consumable_query: Query<&mut Consumable, With<Consumable>>) {
    if consumable_query.is_empty() {
        return;
    }

    for mut c in consumable_query.iter_mut() {
        c.decay()
    }
}

fn despawn_consumables(
    mut commands: Commands,
    consumable_query: Query<(&Consumable, Entity), With<Consumable>>,
) {
    for (c, e) in consumable_query.iter() {
        if c.is_despawn() {
            commands.entity(e).despawn();
        }
    }
}

fn replicate_consumables(
    mut commands: Commands,
    handle: Res<GlobalTextureHandle>,
    food_query: Query<With<Food>>,
    poison_query: Query<With<Poison>>,
    mut consumable_query: Query<
        (&Transform, &Consumable, &mut LastReplicationTs),
        With<Consumable>,
    >,
) {
    let num_food = food_query.iter().len();
    let num_poison = poison_query.iter().len();
    let is_populate_food = num_food < NUM_FOOD;
    let is_populate_poison = num_poison < NUM_POISON;

    let mut rng = rand::thread_rng();
    for (transform, consumable, mut last_replication_ts) in consumable_query.iter_mut() {
        if last_replication_ts.0.elapsed().as_secs_f32() < REPLICATION_COOLDOWN {
            continue;
        }
        if rng.gen_range(0.0..1.0) < 0.8 {
            continue;
        }

        // Probability to replicate at the center > edges
        let dist_to_center = transform
            .translation
            .truncate()
            .distance_squared(Vec2::ZERO);
        if rng.gen_range(0.0..1.0) < dist_to_center / 1000000.0 {
            continue;
        }

        let replication_radius = if consumable.is_food() {
            REPLICATION_RADIUS_FOOD
        } else {
            REPLICATION_RADIUS_POISON
        };
        let (x, y) = (transform.translation.x, transform.translation.y);
        let (x, y) = (
            x + rng.gen_range(-replication_radius..replication_radius),
            y + rng.gen_range(-replication_radius..replication_radius),
        );
        let (x, y) = limit_to_world((x, y));

        if consumable.is_food() && is_populate_food {
            commands.spawn(FoodBundle::new((x, y), handle.0.clone().unwrap()));
        } else if rng.gen_range(0.0..1.0) > 0.4 && is_populate_poison {
            commands.spawn(PoisonBundle::new((x, y), handle.0.clone().unwrap()));
        }
        last_replication_ts.0 = Instant::now();
    }
}

impl Consumable {
    // 0.1 and -0.1 are min nutrition values for food/poison

    pub fn is_food(&self) -> bool {
        self.0 >= 0.1
    }

    fn is_despawn(&self) -> bool {
        (self.is_food() && self.0 <= 0.1) || (!self.is_food() && self.0 >= -0.1)
    }

    fn decay(&mut self) {
        if self.is_food() {
            self.0 = (self.0 + FOOD_DECAY_RATE).max(0.1);
            return;
        }

        self.0 = (self.0 + POISON_DECAY_RATE).min(-0.1);
    }

    fn opacity(&self) -> f32 {
        if self.is_food() {
            self.0 / FOOD_NUTRITION
        } else {
            self.0 / POISON_DAMAGE
        }
    }
}

impl ConsumableBundle {
    pub fn new(
        (x, y): (f32, f32),
        handle: Handle<TextureAtlas>,
        color: Color,
        nutrition: f32,
    ) -> Self {
        Self {
            sprite_sheet_bundle: SpriteSheetBundle {
                texture_atlas: handle,
                sprite: TextureAtlasSprite {
                    index: 1,
                    color,
                    ..default()
                },
                transform: Transform::from_scale(Vec3::splat(CONSUMABLE_SPRITE_SCALE))
                    .with_translation(vec3(x, y, 2.0)),
                ..default()
            },
            consumable: Consumable(nutrition),
            last_replication_ts: LastReplicationTs(Instant::now()),
        }
    }
}

impl FoodBundle {
    pub fn new((x, y): (f32, f32), handle: Handle<TextureAtlas>) -> Self {
        Self {
            consumable_bundle: ConsumableBundle::new(
                (x, y),
                handle,
                get_color(COLOR_FOOD),
                FOOD_NUTRITION,
            ),
            food: Food,
        }
    }
}

impl PoisonBundle {
    pub fn new((x, y): (f32, f32), handle: Handle<TextureAtlas>) -> Self {
        Self {
            consumable_bundle: ConsumableBundle::new(
                (x, y),
                handle,
                get_color(COLOR_POISON),
                POISON_DAMAGE,
            ),
            poison: Poison,
        }
    }
}
