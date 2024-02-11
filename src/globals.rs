use bevy::prelude::*;

#[derive(Resource)]
pub struct GlobalTextureHandle(pub Option<Handle<TextureAtlas>>);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum SimState {
    #[default]
    Loading,
    InitSim,
    Simulating,
}

#[derive(Resource)]
pub struct Settings {
    pub camera_follow_boid: bool,
    pub camera_follow_predator: bool,
    pub camera_clamp_center: bool,
    pub enable_gizmos: bool,
    pub show_plots: bool,
    pub show_plot_settings: bool,
    pub plot_options: PlotOptions,
}

pub struct PlotOptions {
    pub num_boids: bool,
    pub lifespan: bool,
    pub perception: bool,
    pub affinity: bool,
    pub steering: bool,
    pub speed: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            camera_follow_boid: false,
            camera_follow_predator: false,
            camera_clamp_center: true,
            enable_gizmos: false,
            show_plots: false,
            show_plot_settings: false,
            plot_options: PlotOptions::default(),
        }
    }
}

impl Default for PlotOptions {
    fn default() -> Self {
        Self {
            num_boids: false,
            lifespan: true,
            perception: true,
            affinity: true,
            steering: false,
            speed: false,
        }
    }
}
