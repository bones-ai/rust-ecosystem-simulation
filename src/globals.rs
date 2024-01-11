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
    pub enable_camera_follow: bool,
    pub enable_gizmos: bool,
    pub enable_plots: bool,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            enable_camera_follow: false,
            enable_gizmos: false,
            enable_plots: false,
        }
    }
}
