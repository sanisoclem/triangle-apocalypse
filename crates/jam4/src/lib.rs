use bevy::prelude::*;

mod components;
mod mods;
mod state;

pub use components::*;
pub use mods::*;
pub use state::*;

pub trait Jam4Extensions {
  fn add_jam_game(&mut self) -> &mut Self;
}

impl Jam4Extensions for App {
  fn add_jam_game(&mut self) -> &mut Self {
    self
      .init_resource::<ModManager>()
      .add_state::<SimulationState>()
      .add_event::<GameControlCommand>()
      .add_systems(OnExit(SimulationState::Disabled), register_mods)
      .add_systems(OnEnter(SimulationState::Loading), run_mod_init)
      .add_systems(OnEnter(SimulationState::Simulating), run_mod_setup)
      .add_systems(
        Update,
        (
          process_game_control_commands,
          run_mod_update.run_if(in_state(SimulationState::Simulating)),
          wait_until_loading_complete.run_if(in_state(SimulationState::Loading)),
        ),
      )
  }
}
