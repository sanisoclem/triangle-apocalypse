use bevy::prelude::*;

use crate::Loading;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum SimulationState {
  #[default]
  Disabled, // initial state
  /// All modules have been initialized
  /// this is only done once
  Ready,
  /// Loading a level
  Loading,
  /// Level has been loaded, waiting for start signal
  Loaded,
  /// Simulation is running
  /// this can only transition to LevelComplete or GameOver
  Simulating, // main phase, create UI here
  /// level has been completed
  /// chance to show summary before next level is loaded
  /// can transition to either Complete or Loading
  LevelComplete,
  /// End state
  GameComplete,
  /// Player failed objectives
  /// can only transition to Loading (to reload the level)
  GameOver,
}

#[derive(Event, Debug)]
pub enum GameControlCommand {
  Initialize,
  NewGame,
  StartGame,
  NextLevel,
  Retry
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
      }
      (SimulationState::Loaded, GameControlCommand::StartGame) => {
        // game has been loaded, signal that we should start the simulation
        next_sim_state.set(SimulationState::Simulating)
      },
      (SimulationState::LevelComplete, GameControlCommand::NextLevel) => {
        // level has been completed, signal that we want to unload current level and load next level
        next_sim_state.set(SimulationState::Loading)
      },
      (SimulationState::GameOver, GameControlCommand::Retry) => {
        next_sim_state.set(SimulationState::Loading)
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
