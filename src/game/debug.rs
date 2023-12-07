use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use jam4::boid::BoidConfig;

pub fn boid_config_debug(mut config: ResMut<BoidConfig>, mut contexts: EguiContexts) {
  egui::Window::new("Boid Config").show(contexts.ctx_mut(), |ui| {
    ui.add(egui::Slider::new(&mut config.cohesion, 0.0..=100.0).text("Cohesion"));
    ui.add(egui::Slider::new(&mut config.alignment, 0.0..=100.0).text("Alignment"));
    ui.add(egui::Slider::new(&mut config.repulsion, 0.0..=100.0).text("Repulsion"));
    ui.add(egui::Slider::new(&mut config.boundary, 0.0..=100.0).text("Boundary"));
    ui.add(egui::Slider::new(&mut config.player_influence, 0.0..=1000.0).text("Player Influence"));
    ui.add(egui::Checkbox::new(
      &mut config.show_direction,
      "Show Direction",
    ));
    ui.add(egui::Checkbox::new(&mut config.show_forces, "Show Forces"));
    ui.add(egui::Checkbox::new(
      &mut config.show_personal_space,
      "Show Personal Space",
    ));
    ui.add(egui::Checkbox::new(&mut config.show_vision, "Show Vision"));
    ui.add(egui::Checkbox::new(&mut config.show_bounds, "Show Bounds"));
  });
}
