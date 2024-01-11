use bevy::math::vec2;
use bevy::prelude::*;
use bevy::window::close_on_esc;
use bevy_pancam::{PanCam, PanCamPlugin};

use ecosim::*;
use ecosim::{boid::BoidPlugin, gui::GuiPlugin, stats::StatsPlugin, world::WorldPlugin};

fn main() {
    App::new()
        .add_state::<SimState>()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: bevy::window::WindowMode::Fullscreen,
                        resolution: (WW as f32, WH as f32).into(),
                        title: "eco-sim".to_string(),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(PanCamPlugin)
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::rgba_u8(
            COLOR_BACKGROUND.0,
            COLOR_BACKGROUND.1,
            COLOR_BACKGROUND.2,
            0,
        )))
        .insert_resource(GlobalTextureHandle(None))
        .insert_resource(Settings::new())
        .add_plugins(BoidPlugin)
        .add_plugins(GuiPlugin)
        .add_plugins(StatsPlugin)
        .add_plugins(WorldPlugin)
        .add_systems(OnEnter(SimState::Loading), setup)
        .add_systems(Update, handle_keyboard_input)
        .add_systems(Update, close_on_esc)
        .run();
}

fn setup(
    mut commands: Commands,
    mut handle: ResMut<GlobalTextureHandle>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut next_state: ResMut<NextState<SimState>>,
) {
    let texture_handle = asset_server.load(SPRITE_SHEET_PATH);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        vec2(TILE_W, TILE_H),
        SPRITE_SHEET_ROWS,
        SPRITE_SHEET_COLS,
        None,
        None,
    );
    handle.0 = Some(texture_atlases.add(texture_atlas));

    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());
    next_state.set(SimState::InitSim);
}

fn handle_keyboard_input(keys: Res<Input<KeyCode>>, mut settings: ResMut<Settings>) {
    if keys.just_pressed(KeyCode::Back) {
        settings.enable_camera_follow = !settings.enable_camera_follow;
    }
    if keys.just_pressed(KeyCode::Tab) {
        settings.enable_gizmos = !settings.enable_gizmos;
    }
    if keys.just_pressed(KeyCode::Backslash) {
        settings.enable_plots = !settings.enable_plots;
    }
}
