use bevy::{prelude::*, sprite::Material2dPlugin};
use bevy_smud::SmudPlugin;

pub mod boid;
mod components;
mod finish_line;
mod grid;
pub mod level;
mod mods;
pub mod moveable;
mod player;
mod state;

use boid::{
  calc_tamed_boids, calculate_boid_direction, despawn_collided_boids, draw_boid_gizmos,
  update_boid_velocity, update_tamed_boids, BoidConfig,
};
pub use components::*;
use finish_line::FinishLineMaterial;
use grid::GridMaterial;
use level::{
  check_if_game_over, check_if_level_complete, find_level_to_load, on_load_level_requested,
  time_level, LevelManager, LevelRegistry,
};
pub use mods::*;
use moveable::{move_moveables, MoveableBounds};
pub use player::*;
pub use state::*;

pub trait Jam4Extensions {
  fn add_jam_game(&mut self) -> &mut Self;
}

impl Jam4Extensions for App {
  fn add_jam_game(&mut self) -> &mut Self {
    self
      .add_plugins(SmudPlugin)
      .add_plugins(Material2dPlugin::<GridMaterial>::default())
      .add_plugins(Material2dPlugin::<FinishLineMaterial>::default())
      .init_resource::<PlayerInfo>()
      .init_resource::<ModManager>()
      .init_resource::<MoveableBounds>()
      .init_resource::<BoidConfig>()
      .init_resource::<LevelRegistry>()
      .init_resource::<LevelManager>()
      .add_state::<SimulationState>()
      .add_event::<GameControlCommand>()
      .add_systems(OnExit(SimulationState::Disabled), register_mods)
      .add_systems(OnEnter(SimulationState::Initializing), run_mod_init)
      .add_systems(
        OnEnter(SimulationState::ChoosingLevel),
        (
          find_level_to_load,
          on_load_level_requested,
          apply_deferred,
          run_mod_setup,
        )
          .chain(),
      )
      .add_systems(
        FixedUpdate,
        (
          calc_tamed_boids,
          apply_deferred,
          update_tamed_boids,
          calculate_boid_direction,
          update_boid_velocity,
          (
            time_level,
            check_if_game_over,
            despawn_collided_boids,
            check_if_level_complete,
          ),
        )
          .chain()
          .run_if(in_state(SimulationState::Simulating)),
      )
      .add_systems(
        Update,
        (
          move_moveables,
          process_game_control_commands,
          (run_mod_update, draw_boid_gizmos).run_if(in_state(SimulationState::Simulating)),
          wait_until_initialization_complete.run_if(in_state(SimulationState::Initializing)),
        ),
      )
  }
}
