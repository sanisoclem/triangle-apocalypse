use bevy::{prelude::*, sprite::Mesh2dHandle};

use crate::{boid::Boid, moveable::Moveable};

#[derive(Component, Default)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
  pub mesh: Mesh2dHandle,
  pub material: Handle<ColorMaterial>,
  pub transform: Transform,
  pub global_transform: GlobalTransform,
  pub visibility: Visibility,
  pub inherited_visibility: InheritedVisibility,
  pub view_visibility: ViewVisibility,
  pub player: Player,
  pub moveable: Moveable,
  pub boid: Boid,
}

impl Default for PlayerBundle {
  fn default() -> Self {
    Self {
      moveable: Moveable { ..default() },
      boid: Boid {
        is_player: true,
        personal_space: 100.,
        ..default()
      },
      mesh: default(),
      material: default(),
      transform: default(),
      global_transform: default(),
      visibility: default(),
      inherited_visibility: default(),
      view_visibility: default(),
      player: default(),
    }
  }
}
