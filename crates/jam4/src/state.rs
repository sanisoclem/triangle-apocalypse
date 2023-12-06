use bevy::prelude::*;

use crate::Loading;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum SimulationState {
  #[default]
  Disabled,
  Ready,
  Loading,
  Loaded,
  Simulating,
  Unloading,
}

#[derive(Event, Debug)]
pub enum GameControlCommand {
  Initialize,
  NewGame,
  StartGame,
}

pub fn process_game_control_commands(
  mut cmds: EventReader<GameControlCommand>,
  mut next_sim_state: ResMut<NextState<SimulationState>>,
  current_state: Res<State<SimulationState>>,
) {
  for cmd in cmds.read() {
    match ((*current_state).get(), cmd) {
      (SimulationState::Disabled, GameControlCommand::Initialize) => {
        // set state so exclusive system to register module runs
        next_sim_state.set(SimulationState::Ready);
      }
      (SimulationState::Ready, GameControlCommand::NewGame) => {
        // signal to all systems to start loading whatever is required for new game
        next_sim_state.set(SimulationState::Loading);
      },
      (SimulationState::Loaded, GameControlCommand::StartGame) => {
        // game has been loaded, signal that we should start the simulation
        next_sim_state.set(SimulationState::Simulating)
      }
      _ => {
        unimplemented!()
      }
    }
  }
}

pub fn wait_until_loading_complete(
  qry: Query<Entity, With<Loading>>,
  mut next_state: ResMut<NextState<SimulationState>>,
) {
  if qry.is_empty() {
    next_state.set(SimulationState::Loaded);
  }
}
