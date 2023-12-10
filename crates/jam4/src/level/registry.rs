use std::{collections::HashMap, time::Duration};

use bevy::{prelude::*, time::Stopwatch};
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
  pub finish_bounds_box: MoveableBounds,
  pub finish_bounds: Vec4,
  pub bounds_sdf: Option<SmudShape>,
  pub starting_point: Vec2,
  pub next_level: Option<LevelId>,
  pub spawn_points: Vec<Vec2>,
  pub boids_per_spawn_point: i32,
  pub rescue_goal: Option<u16>,
  pub time_goal: Option<Duration>,
  pub wander: bool,
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
  pub watch: Stopwatch
}

impl LevelManager {
  pub fn load_level(&mut self, level_id: &LevelId) {
    self.load_next = Some(*level_id);
  }
}
