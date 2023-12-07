use bevy::prelude::*;

#[derive(Resource)]
pub struct BoidConfig {
  pub max_speed: f32,
  pub min_speed: f32,
  pub boundary: f32,
  pub cohesion: f32,
  pub alignment: f32,
  pub repulsion: f32,
  pub lprobe: Mat2,
  pub rprobe: Mat2,
  pub lforce: Mat2,
  pub rforce: Mat2,
  pub show_forces: bool,
  pub show_direction: bool,
  pub show_personal_space: bool,
  pub show_vision: bool,
  pub show_bounds: bool,
  pub player_influence: f32,
}

impl Default for BoidConfig {
  fn default() -> Self {
    BoidConfig {
      max_speed: 1000.,
      min_speed: 500.,
      player_influence: 10.,
      boundary: 5.0,
      cohesion: 1.0,
      alignment: 1.0,
      repulsion: 10.0,
      lprobe: Mat2::from_angle(45.0f32.to_radians()),
      rprobe: Mat2::from_angle(-45.0f32.to_radians()),
      lforce: Mat2::from_angle(90.0f32.to_radians()),
      rforce: Mat2::from_angle(-90.0f32.to_radians()),
      show_forces: false,
      show_direction: false,
      show_personal_space: false,
      show_vision: false,
      show_bounds: true
    }
  }
}
