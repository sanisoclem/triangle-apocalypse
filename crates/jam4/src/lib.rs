use bevy::prelude::*;

pub mod boid;
mod components;
mod mods;
pub mod moveable;
mod player;
mod state;

use boid::{calculate_boid_direction, update_boid_velocity, BoidConfig};
pub use components::*;
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
      .init_resource::<ModManager>()
      .init_resource::<MoveableBounds>()
      .init_resource::<BoidConfig>()
      .add_state::<SimulationState>()
      .add_event::<GameControlCommand>()
      .add_systems(OnExit(SimulationState::Disabled), register_mods)
      .add_systems(OnEnter(SimulationState::Loading), run_mod_init)
      .add_systems(OnEnter(SimulationState::Simulating), run_mod_setup)
      .add_systems(
        Update,
        (
          process_game_control_commands,
          (
            run_mod_update,
            (
              calculate_boid_direction,
              update_boid_velocity,
              move_moveables,
            )
              .chain(),
          )
            .run_if(in_state(SimulationState::Simulating)),
          wait_until_loading_complete.run_if(in_state(SimulationState::Loading)),
        ),
      )
  }
}
