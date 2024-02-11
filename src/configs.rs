// Window
pub const WW: usize = 1600;
pub const WH: usize = 1000;

// Sprite
pub const SPRITE_SHEET_PATH: &str = "assets.png";
pub const TILE_H: f32 = 16.0;
pub const TILE_W: f32 = 16.0;
pub const SPRITE_SHEET_ROWS: usize = 10;
pub const SPRITE_SHEET_COLS: usize = 10;
pub const BOID_SPRITE_SCALE: f32 = 1.5;
pub const CONSUMABLE_SPRITE_SCALE: f32 = 1.0;

// Sim
pub const WORLD_W: f32 = 1305.0;
pub const WORLD_H: f32 = 730.0;
pub const NUM_BOIDS: usize = 100;
pub const NUM_PREDATORS: usize = 10;
pub const NUM_FOOD: usize = 600;
pub const NUM_POISON: usize = 150;

// Boids
pub const BOID_COLLISION_RADIUS: f32 = 8.0;
pub const BOID_MAX_HEALTH: f32 = 100.0;
pub const BOID_TICK_DAMAGE: f32 = 4.0;
pub const BOID_REPLICATE_INTERVAL: f32 = 5.0;
pub const BOID_REPLICATE_PROBABILITY: f32 = 0.2;
pub const BOID_MUTATION_THRESHOLD: f32 = 0.8;

// Predators
pub const BOID_NUTRITION: f32 = 30.0;
pub const PREDATOR_COLLISION_RADIUS: f32 = 10.0;

// Colors
pub const COLOR_FOOD: (u8, u8, u8) = (142, 231, 112);
pub const COLOR_POISON: (u8, u8, u8) = (235, 86, 75);
pub const COLOR_BOID: (u8, u8, u8) = (255, 255, 255);
pub const COLOR_PREDATOR: (u8, u8, u8) = (255, 236, 179);
pub const COLOR_PREDATOR_LOW_HEALTH: (u8, u8, u8) = (255, 145, 102);
pub const COLOR_BOID_LOW_HEALTH: (u8, u8, u8) = (102, 255, 227);
pub const COLOR_BACKGROUND: (u8, u8, u8) = (50, 62, 79);

// Consumables
pub const FOOD_NUTRITION: f32 = 5.0;
pub const POISON_DAMAGE: f32 = -30.0;
pub const FOOD_DECAY_RATE: f32 = -0.1;
pub const POISON_DECAY_RATE: f32 = 0.5;
pub const REPLICATION_RADIUS_FOOD: f32 = 40.0;
pub const REPLICATION_RADIUS_POISON: f32 = 5.0;
pub const CONSUMABLE_REPLICATION_RATE: f32 = 0.2;
pub const CONSUMABLE_DECAY_RATE: f32 = 1.5;
pub const REPLICATION_COOLDOWN: f32 = 2.0;

// Stats
pub const STAT_COLLECTION_RATE: f32 = 1.0;
pub const MAX_NUM_POINTS: usize = 8000;

// Graphs
pub const PLOT_LINE_WIDTH: f32 = 1.5;
pub const PLOT_ASPECT_RATIO: f32 = 1.8;
