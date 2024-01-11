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
pub const WORLD_W: f32 = 950.0;
pub const WORLD_H: f32 = 530.0;
pub const NUM_BOIDS: usize = 50;
pub const NUM_FOOD: usize = 450;
pub const NUM_POISON: usize = 200;

// Boids
pub const BOID_COLLISION_RADIUS: f32 = 8.0;
pub const BOID_MAX_HEALTH: f32 = 100.0;
pub const BOID_TICK_DAMAGE: f32 = 4.0;
pub const BOID_REPLICATE_INTERVAL: f32 = 10.0;

// Colors
pub const COLOR_FOOD: (u8, u8, u8) = (142, 231, 112);
pub const COLOR_POISON: (u8, u8, u8) = (224, 69, 55);
pub const COLOR_BACKGROUND: (u8, u8, u8) = (35, 53, 64);
// pub const COLOR_BACKGROUND: (u8, u8, u8) = (34, 34, 34);

// Consumables
pub const FOOD_NUTRITION: f32 = 5.0;
pub const POISON_DAMAGE: f32 = -20.0;

// Stats
pub const STAT_COLLECTION_RATE: f32 = 1.0;
pub const MAX_NUM_POINTS: usize = 8000;

// Graphs
pub const PLOT_LINE_WIDTH: f32 = 1.5;
pub const PLOT_ASPECT_RATIO: f32 = 1.8;
