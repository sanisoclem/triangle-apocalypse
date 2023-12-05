use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use camera::*;
use music::*;

use jam4::{
  boid::{Boid, BoidConfig},
  Player, SimulationState,
};
mod camera;
mod music;

#[derive(Resource)]
struct GameNextState<T>(T);
pub trait GameExtensions {
  fn add_game<T: States + Copy>(&mut self, game_state: T) -> &mut Self;
}

impl GameExtensions for App {
  fn add_game<T: States + Copy>(&mut self, game_state: T) -> &mut Self {
    self
      .add_systems(OnEnter(game_state), (setup_music, setup_camera))
      .add_systems(
        Update,
        (calc_player_direction, follow_player, boid_config)
          .run_if(in_state(SimulationState::Simulating)),
      )
    //     .add_player_camera()
    //     .add_music()
    //     .add_player_ui()
  }
}

pub fn calc_player_direction(
  mut qry: Query<&mut Boid, With<Player>>,
  keyboard_input: Res<Input<KeyCode>>,
  time: Res<Time>,
) {
  if let Ok(mut p) = qry.get_single_mut() {
    let mut turning_force = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::A) {
      turning_force += Mat2::from_angle(90.0f32.to_radians()).mul_vec2(p.direction);
    } else if keyboard_input.pressed(KeyCode::D) {
      turning_force += Mat2::from_angle(-90.0f32.to_radians()).mul_vec2(p.direction);
    }

    p.direction = (p.direction + (turning_force * time.delta_seconds() * 6.0)).normalize();
  }
}

fn boid_config(mut config: ResMut<BoidConfig>, mut contexts: EguiContexts) {
  egui::Window::new("Boid Config").show(contexts.ctx_mut(), |ui| {
    ui.add(egui::Slider::new(&mut config.cohesion, 0.0..=1.0).text("Cohesion"));
    ui.add(egui::Slider::new(&mut config.alignment, 0.0..=1.0).text("Alignment"));
    ui.add(egui::Slider::new(&mut config.repulsion, 0.0..=100.0).text("Repulsion"));
    ui.add(egui::Slider::new(&mut config.boundary, 0.0..=1000.0).text("Boundary"));
    ui.add(egui::Slider::new(&mut config.boid_speed, 0.0..=1000.0).text("Speed"));
    ui.add(egui::Slider::new(&mut config.player_boid_speed, 0.0..=1000.0).text("Player Speed"));
    ui.add(egui::Slider::new(&mut config.turn_rate, 0.0..=100.0).text("Turn Rate"));
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
