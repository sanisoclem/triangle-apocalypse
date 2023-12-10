use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_hanabi::prelude::*;
use bevy_smud::{Frame, ShapeBundle};

use crate::{
  boid::{Boid, BoidConfig, TamedBoid},
  finish_line::{build_finish_line, FinishLineMaterial},
  grid::{build_grid, GridMaterial},
  moveable::{CollidedWithBounds, Moveable, MoveableBounds},
  spawn_player, Player, PlayerInfo, Simulation, SimulationState,
};

mod registry;

pub use registry::*;

pub fn time_level(
  mut next_sim_state: ResMut<NextState<SimulationState>>,
  mut lvl_mgr: ResMut<LevelManager>,
  lvl_reg: Res<LevelRegistry>,
  time: Res<Time>,
) {
  let Some(level_id) = lvl_mgr.current_level else {
    return;
  };
  let lvl = lvl_reg.get_level(&level_id);
  lvl_mgr.watch.tick(time.delta());

  if let Some(time_goal) = lvl.time_goal {
    if lvl_mgr.watch.elapsed() > time_goal {
      next_sim_state.set(SimulationState::GameOver(crate::GameOverReason::OutOfTime));
      return;
    }
  }
}

pub fn check_if_level_complete(
  qry: Query<&Transform, With<Player>>,
  mut next_sim_state: ResMut<NextState<SimulationState>>,
  mut lvl_mgr: ResMut<LevelManager>,
  lvl_reg: Res<LevelRegistry>,
  qry_boid: Query<Entity, (With<Boid>, With<TamedBoid>, Without<Player>)>,
  mut player: ResMut<PlayerInfo>,
) {
  let Ok(t) = qry.get_single() else {
    return;
  };
  let level_id = lvl_mgr.current_level.unwrap();
  let lvl = lvl_reg.get_level(&level_id);

  if lvl.finish_bounds_box.distance_to_edge(t.translation.xy()) < 0.0 {
    // hit the finish line

    let score = qry_boid.iter().count();
    if let Some(rescue_goal) = lvl.rescue_goal {
      if score < rescue_goal as usize {
        next_sim_state.set(SimulationState::GameOver(crate::GameOverReason::OutOfBoids));
        return;
      }
    }

    lvl_mgr.level_complete = true;
    player.score += score as u32;
    next_sim_state.set(SimulationState::LevelComplete);
  }
}

pub fn check_if_game_over(
  qry: Query<Entity, (With<Player>, Added<CollidedWithBounds>)>,
  mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
  if qry.is_empty() {
    return;
  }

  next_sim_state.set(SimulationState::GameOver(
    crate::GameOverReason::OutOfBounds,
  ));
}

pub fn on_load_level_requested(
  mut cmd: Commands,
  mut lvl_mgr: ResMut<LevelManager>,
  lvl_reg: Res<LevelRegistry>,
  mut bconfig: ResMut<BoidConfig>,
  mut player: ResMut<PlayerInfo>,
  mut bounds: ResMut<MoveableBounds>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut grid_mats: ResMut<Assets<GridMaterial>>,
  mut fline_mats: ResMut<Assets<FinishLineMaterial>>,
  to_despawn: Query<Entity, With<Simulation>>,
  mut next_sim_state: ResMut<NextState<SimulationState>>,
) {
  let Some(id_to_load) = lvl_mgr.load_next else {
    return;
  };
  let to_load = lvl_reg.get_level(&id_to_load);

  // despawn all prev level entities
  for entity in &to_despawn {
    cmd.entity(entity).despawn_recursive();
  }

  // reset
  player.in_boost_mode = false;
  lvl_mgr.watch.reset();

  bconfig.wander = to_load.wander;

  // spawn level entities
  if let Some(shape) = &to_load.bounds_sdf {
    cmd
      .spawn(ShapeBundle {
        shape: shape.clone(),
        transform: Transform::from_translation(Vec2::splat(0.0).extend(-10.0)),
        ..default()
      })
      .insert(Simulation);
    let Frame::Quad(frame_size) = shape.frame;
    build_grid(
      &mut cmd,
      &mut meshes,
      &mut grid_mats,
      Vec2::splat(frame_size),
    );
  }
  build_finish_line(
    &mut cmd,
    &mut meshes,
    &mut fline_mats,
    to_load.finish_bounds,
  );

  // update bounds
  *bounds = to_load.bounds.clone();

  spawn_player(&mut cmd, &player, to_load.starting_point, &bconfig).insert(Simulation);

  for point in to_load.spawn_points.iter() {
    for x in 0..to_load.boids_per_spawn_point {
      cmd
        .spawn(MaterialMesh2dBundle {
          mesh: meshes.add(shape::RegularPolygon::new(10., 3).into()).into(),
          material: bconfig.color_wild.clone(),
          transform: Transform::from_translation(
            point.extend(0.0) + Quat::from_rotation_z(x as f32 * 27. / 5.).mul_vec3(Vec3::Y),
          )
          .with_scale(Vec3::new(1.0, 2.0, 1.0))
          .looking_at(point.extend(0.0), Vec3::Z),
          ..default()
        })
        .insert((
          Moveable::default(),
          Boid {
            direction: Mat2::from_angle(x as f32).mul_vec2(Vec2::Y),
            turning_speed: bconfig.wild_turn_speed,
            speed: bconfig.wild_speed,
            is_player: false,
            personal_space: 20.,
            vision: 400.,
          },
          Simulation,
        ))
        .with_children(|p| {
          p.spawn((ParticleEffectBundle {
            effect: ParticleEffect::new(bconfig.cotrails.clone()),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..Default::default()
          },));
        });
    }
  }

  lvl_mgr.current_level = Some(id_to_load);
  lvl_mgr.level_complete = false;
  lvl_mgr.load_next = None;

  next_sim_state.set(SimulationState::Simulating)
}

pub fn find_level_to_load(
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
