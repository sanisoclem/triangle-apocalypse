use std::collections::HashMap;

use bevy::prelude::*;
use bevy_smud::SmudShape;

use crate::moveable::MoveableBounds;

#[derive(PartialEq, Hash, Eq, Clone, Copy)]
pub struct LevelId(u8);

impl From<u8> for LevelId {
  fn from(value: u8) -> Self {
    Self(value)
  }
}

#[derive(Resource, Default)]
pub struct LevelRegistry {
  pub levels: HashMap<LevelId, LevelInfo>,
  pub start_level: Option<LevelId>,
}

pub struct LevelInfo {
  pub name: String,
  pub bounds: MoveableBounds,
  pub finish_bounds: MoveableBounds,
  pub bounds_sdf: Option<SmudShape>,
  pub finish_bounds_sdf: Option<(SmudShape, Vec3)>,
  pub starting_point: Vec2,
  pub music: Handle<AudioSource>,
  pub next_level: Option<LevelId>,
}

impl LevelRegistry {
  pub fn get_level(&self, id: &LevelId) -> &LevelInfo {
    self.levels.get(id).expect("Level should be in registrry")
  }
}

#[derive(Resource, Default)]
pub struct LevelManager {
  pub current_level: Option<LevelId>,
  pub level_complete: bool,
  pub load_next: Option<LevelId>,
}

impl LevelManager {
  pub fn load_level(&mut self, level_id: &LevelId) {
    self.load_next = Some(*level_id);
  }
}
