use bevy::prelude::*;
use bevy_smud::ShapeBundle;

use crate::{
  moveable::{CollidedWithBounds, MoveableBounds},
  Player, PlayerBundle, Simulation, SimulationState,
};

mod registry;

pub use registry::*;

pub fn check_if_level_complete() {
  // check if exit bounds have been crossed
  // set simulation state to level complete
}

pub fn check_if_game_over(
  qry: Query<Entity, (With<Player>, Added<CollidedWithBounds>)>,
  mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
  if qry.is_empty() {
    return;
  }

  next_sim_state.set(SimulationState::GameOver);
}

pub fn on_load_level_requested(
  mut cmd: Commands,
  mut lvl_mgr: ResMut<LevelManager>,
  lvl_reg: Res<LevelRegistry>,

  mut bounds: ResMut<MoveableBounds>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  to_despawn: Query<Entity, With<Simulation>>,
) {
  if let Some(id_to_load) = lvl_mgr.load_next {
    let to_load = lvl_reg.get_level(&id_to_load);

    // despawn all prev level entities
    for entity in &to_despawn {
      cmd.entity(entity).despawn_recursive();
    }

    // spawn level entities
    if let Some(shape) = &to_load.bounds_sdf {
      cmd
        .spawn(ShapeBundle {
          shape: shape.clone(),
          ..default()
        })
        .insert(Simulation);
    }

    // update bounds
    *bounds = to_load.bounds.clone();

    // TODO: create resource to define player entity
    cmd
      .spawn(PlayerBundle {
        mesh: meshes.add(shape::RegularPolygon::new(20., 3).into()).into(),
        material: materials.add(ColorMaterial::from(Color::rgb(7.5, 0.0, 7.5))),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.0))
          .with_scale(Vec3::new(1.0, 2.0, 1.0)),
        ..default()
      })
      .insert(Simulation);

    // TODO: start playing when game starts
    cmd
      .spawn(AudioBundle {
        source: to_load.music.clone(),
        settings: PlaybackSettings::LOOP,
        ..default()
      })
      .insert(Simulation);

    lvl_mgr.current_level = Some(id_to_load);
    lvl_mgr.level_complete = false;
    lvl_mgr.load_next = None;
  }
}

pub fn on_loading(
  mut lvl_mgr: ResMut<LevelManager>,
  lvl_reg: Res<LevelRegistry>,
  mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
  if let Some(cur_lvl) = lvl_mgr.current_level {
    if !lvl_mgr.level_complete {
      lvl_mgr.load_level(&cur_lvl);
    } else {
      // level complete
      let cur = lvl_reg.get_level(&cur_lvl);

      if let Some(next) = cur.next_level {
        // load next level
        lvl_mgr.load_level(&next);
      } else {
        // no more levels, GG
        next_sim_state.set(SimulationState::GameComplete)
      }
    }
  } else {
    // no curent level, load start level
    let start = lvl_reg
      .start_level
      .expect("to have start level when starting a new game");
    lvl_mgr.load_level(&start);
  }
}
