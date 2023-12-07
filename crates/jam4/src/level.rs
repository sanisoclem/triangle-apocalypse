use std::collections::HashMap;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_smud::{Frame, ShapeBundle, SmudShape};
use utils::tex_settings_tiled;

use crate::{
  boid::Boid,
  moveable::{Moveable, MoveableBounds},
  Player, Simulation, SimulationState,
};

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

#[derive(Resource, Default)]
pub struct LevelManager {
  pub current_level: Option<LevelId>,
  pub level_complete: bool,
}

pub struct LevelInfo {
  pub name: String,
  pub bounds: MoveableBounds,
  pub finish_bounds: MoveableBounds,
  pub bounds_sdf: Option<SmudShape>,
  pub starting_point: Vec2,
  pub music: Handle<AudioSource>,
  pub next_level: Option<LevelId>,
}

pub fn check_if_level_complete() {
  // check if exit bounds have been crossed
  // set simulation state to level complete
}

pub fn check_if_game_over() {
  // TODO: check if bounds have been crossed
  // set simulation state to game over
}

fn load_level(
  mut bounds: ResMut<MoveableBounds>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  mut cmd: Commands,
  to_despawn: Query<Entity, With<Simulation>>,
  lvl_id: &LevelId,
  lvl: &LevelInfo,
  mut lvl_mgr: ResMut<LevelManager>,
  asset_server: Res<AssetServer>,
) {
  // despawn all prev level entities
  for entity in &to_despawn {
    cmd.entity(entity).despawn_recursive();
  }

  // spawn level entities
  if let Some(shape) = &lvl.bounds_sdf {
    cmd
      .spawn(ShapeBundle {
        shape: shape.clone(),
        ..default()
      })
      .insert(Simulation);
  }

  *bounds = lvl.bounds.clone();

  // TODO: create resource to define player entity
  cmd
    .spawn(MaterialMesh2dBundle {
      mesh: meshes.add(shape::RegularPolygon::new(20., 3).into()).into(),
      material: materials.add(ColorMaterial::from(Color::rgb(7.5, 0.0, 7.5))),
      transform: Transform::from_translation(Vec3::new(0., 0., -100.0))
        .with_scale(Vec3::new(1.0, 2.0, 1.0)),
      ..default()
    })
    .insert(Player::default())
    .insert(Moveable { ..default() })
    .insert(Boid {
      is_player: true,
      personal_space: 100., // TODO: make this configurable
      ..default()
    })
    .insert(Simulation);

  // TODO: start playing when game starts
  cmd
    .spawn(AudioBundle {
      source: lvl.music.clone(),
      settings: PlaybackSettings::LOOP,
      ..default()
    })
    .insert(Simulation);

  // update level manager
  lvl_mgr.current_level = Some(*lvl_id);
  lvl_mgr.level_complete = false;
}

pub fn on_loading(
  bounds: ResMut<MoveableBounds>,
  lvl_mgr: ResMut<LevelManager>,
  lvl_reg: Res<LevelRegistry>,
  mut next_sim_state: ResMut<NextState<SimulationState>>,
  meshes: ResMut<Assets<Mesh>>,
  materials: ResMut<Assets<ColorMaterial>>,
  cmd: Commands,
  to_despawn: Query<Entity, With<Simulation>>,
  asset_server: Res<AssetServer>,
) {
  if let Some(cur_lvl) = lvl_mgr.current_level {
    if !lvl_mgr.level_complete {
      // game over, reload same level
      let current_lvl = lvl_reg
        .levels
        .get(&cur_lvl)
        .expect("Current level should be in registry");
      load_level(
        bounds,
        meshes,
        materials,
        cmd,
        to_despawn,
        &cur_lvl,
        current_lvl,
        lvl_mgr,
        asset_server,
      )
    } else {
      // level complete, load next level
      let cur = lvl_reg
        .levels
        .get(&cur_lvl)
        .expect("to find current level in registry");
      if let Some(next) = cur.next_level {
        let next_lvl = lvl_reg
          .levels
          .get(&next)
          .expect("Next level should be in registry");
        load_level(
          bounds,
          meshes,
          materials,
          cmd,
          to_despawn,
          &next,
          next_lvl,
          lvl_mgr,
          asset_server,
        )
      } else {
        next_sim_state.set(SimulationState::GameComplete)
      }
    }
  } else if lvl_mgr.current_level.is_none() {
    // no curent level, load start level
    let start = lvl_reg
      .start_level
      .expect("to have start level when starting a new game");
    let start_lvl = lvl_reg
      .levels
      .get(&start)
      .expect("Start level should be in registry");
    load_level(
      bounds,
      meshes,
      materials,
      cmd,
      to_despawn,
      &start,
      start_lvl,
      lvl_mgr,
      asset_server,
    )
  }
}
