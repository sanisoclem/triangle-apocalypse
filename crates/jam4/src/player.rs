use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Player;


pub struct PlayerInfo {
  personal_space: f32,
  mesh: Handle<Mesh>,
  material: Handle<ColorMaterial>
}