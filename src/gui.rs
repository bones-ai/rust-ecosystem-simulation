use bevy::{math::vec2, prelude::*};
use bevy_egui::{
    egui::{self, epaint, Color32},
    EguiContexts, EguiPlugin, EguiSettings,
};
use egui_plot::{Line, Plot, PlotPoints};

use crate::boid::{Boid, Dna};
use crate::stats::*;
use crate::*;

pub struct GuiPlugin;

impl Plugin for GuiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .add_systems(OnEnter(SimState::InitSim), setup_egui)
            .add_systems(
                Update,
                (
                    update_egui_plots,
                    draw_boid_debug_gizmos,
                    draw_world_boundary,
                )
                    .run_if(in_state(SimState::Simulating)),
            );
    }
}

fn setup_egui(mut egui_settings: ResMut<EguiSettings>) {
    egui_settings.scale_factor = 1.5;
}

fn draw_world_boundary(mut gizmos: Gizmos) {
    gizmos.rect_2d(
        Vec2::ZERO,
        0.0,
        vec2(WORLD_W * 2.0, WORLD_H * 2.0),
        Color::GRAY,
    );
}

fn draw_boid_debug_gizmos(
    mut gizmos: Gizmos,
    settings: Res<Settings>,
    boids_query: Query<(&Transform, &Dna), With<Boid>>,
) {
    if boids_query.is_empty() {
        return;
    }
    if !settings.enable_gizmos {
        return;
    }

    for (transform, dna) in boids_query.iter() {
        gizmos.circle_2d(
            transform.translation.truncate(),
            dna.food_perception_radius,
            Color::GREEN,
        );
        gizmos.circle_2d(
            transform.translation.truncate(),
            dna.poison_perception_radius,
            Color::RED,
        );
    }
}

fn update_egui_plots(
    mut contexts: EguiContexts,
    stats: Res<SimulationStats>,
    settings: Res<Settings>,
) {
    if !settings.enable_plots {
        return;
    }

    let ctx = contexts.ctx_mut();
    let old = ctx.style().visuals.clone();
    ctx.set_visuals(egui::Visuals {
        window_fill: Color32::from_rgba_premultiplied(0, 0, 0, 130),
        panel_fill: Color32::from_rgba_premultiplied(0, 0, 0, 130),
        window_stroke: egui::Stroke {
            color: Color32::TRANSPARENT,
            width: 0.0,
        },
        window_shadow: epaint::Shadow {
            color: Color32::TRANSPARENT,
            ..old.window_shadow
        },
        ..old
    });
    egui::Window::new("")
        .title_bar(false)
        .default_pos(egui::pos2(1000.0, 10.0))
        .show(contexts.ctx_mut(), |ui| {
            // egui::CollapsingHeader::new("Number of Boids")
            //     .default_open(true)
            //     .show_unindented(ui, |ui| {
            //         let num_boids_curve: PlotPoints = (0..stats.num_boids.items.len())
            //             .map(|i| [i as f64, (stats.num_boids.items[i]) as f64])
            //             .collect();
            //         let line = Line::new(num_boids_curve)
            //             .width(PLOT_LINE_WIDTH)
            //             .color(Color32::WHITE);
            //         Plot::new("num_boids")
            //             .show_axes(false)
            //             .show_background(false)
            //             .show_grid(false)
            //             .view_aspect(PLOT_ASPECT_RATIO)
            //             .auto_bounds_x()
            //             .auto_bounds_y()
            //             .show(ui, |plot_ui| plot_ui.line(line));
            //     });
            egui::CollapsingHeader::new("Lifespan")
                .default_open(true)
                .show_unindented(ui, |ui| {
                    let lifespan_curve: PlotPoints = (0..stats.avg_lifespan.items.len())
                        .map(|i| [i as f64, (stats.avg_lifespan.items[i]) as f64])
                        .collect();
                    let lifespan_line = Line::new(lifespan_curve)
                        .width(PLOT_LINE_WIDTH)
                        .color(Color32::WHITE);
                    Plot::new("lifespan")
                        .show_axes(false)
                        .show_background(false)
                        .show_grid(false)
                        .view_aspect(PLOT_ASPECT_RATIO)
                        .auto_bounds_x()
                        .auto_bounds_y()
                        .show(ui, |plot_ui| {
                            plot_ui.line(lifespan_line);
                        });
                });
            egui::CollapsingHeader::new("Perception")
                .default_open(true)
                .show_unindented(ui, |ui| {
                    let food_perception_curve: PlotPoints = (0..stats.food_perception.items.len())
                        .map(|i| [i as f64, (stats.food_perception.items[i]) as f64])
                        .collect();
                    let poison_perception_curve: PlotPoints =
                        (0..stats.poison_perception.items.len())
                            .map(|i| [i as f64, (stats.poison_perception.items[i]) as f64])
                            .collect();
                    let food_line = Line::new(food_perception_curve)
                        .width(PLOT_LINE_WIDTH)
                        .color(Color32::GREEN);
                    let poison_line = Line::new(poison_perception_curve)
                        .width(PLOT_LINE_WIDTH)
                        .color(Color32::RED);
                    Plot::new("perception_radius")
                        .show_axes(false)
                        .show_background(false)
                        .show_grid(false)
                        .view_aspect(2.0)
                        .auto_bounds_x()
                        .auto_bounds_y()
                        .show(ui, |plot_ui| {
                            plot_ui.line(food_line);
                            plot_ui.line(poison_line);
                        });
                });
            egui::CollapsingHeader::new("Affinity")
                .default_open(true)
                .show_unindented(ui, |ui| {
                    let food_affinity_curve: PlotPoints = (0..stats.food_affinity.items.len())
                        .map(|i| [i as f64, (stats.food_affinity.items[i]) as f64])
                        .collect();
                    let poison_affinity_curve: PlotPoints = (0..stats.poison_affinity.items.len())
                        .map(|i| [i as f64, (stats.poison_affinity.items[i]) as f64])
                        .collect();
                    let food_line = Line::new(food_affinity_curve)
                        .width(PLOT_LINE_WIDTH)
                        .color(Color32::GREEN);
                    let poison_line = Line::new(poison_affinity_curve)
                        .width(PLOT_LINE_WIDTH)
                        .color(Color32::RED);
                    Plot::new("affinity")
                        .show_axes(false)
                        .show_background(false)
                        .show_grid(false)
                        .view_aspect(PLOT_ASPECT_RATIO)
                        .auto_bounds_x()
                        .auto_bounds_y()
                        .show(ui, |plot_ui| {
                            plot_ui.line(food_line);
                            plot_ui.line(poison_line);
                        });
                });
            // egui::CollapsingHeader::new("Steering Force")
            //     .default_open(true)
            //     .show_unindented(ui, |ui| {
            //         let steering_curve: PlotPoints = (0..stats.steering_force.items.len())
            //             .map(|i| [i as f64, (stats.steering_force.items[i].abs()) as f64])
            //             .collect();
            //         let steering_line = Line::new(steering_curve)
            //             .width(PLOT_LINE_WIDTH)
            //             .color(Color32::WHITE);
            //         Plot::new("steering")
            //             .show_axes(false)
            //             .show_background(false)
            //             .show_grid(false)
            //             .view_aspect(2.0)
            //             .auto_bounds_x()
            //             .auto_bounds_y()
            //             .show(ui, |plot_ui| plot_ui.line(steering_line));
            //     });
            // egui::CollapsingHeader::new("Speed")
            //     .default_open(true)
            //     .show_unindented(ui, |ui| {
            //         let speed_curve: PlotPoints = (0..stats.speed.items.len())
            //             .map(|i| [i as f64, (stats.speed.items[i]) as f64])
            //             .collect();
            //         let speed_line = Line::new(speed_curve)
            //             .width(PLOT_LINE_WIDTH)
            //             .color(Color32::WHITE);
            //         Plot::new("speed")
            //             .show_axes(false)
            //             .show_background(false)
            //             .show_grid(false)
            //             .view_aspect(2.0)
            //             .auto_bounds_x()
            //             .auto_bounds_y()
            //             .show(ui, |plot_ui| plot_ui.line(speed_line));
            //     });
        });
}
