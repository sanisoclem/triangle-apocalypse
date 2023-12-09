use bevy::prelude::*;

use crate::Initializing;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum SimulationState {
  #[default]
  Disabled, // initial state
  /// All modules start initialization
  /// this is only done once
  Initializing,
  // all modules initialized
  Ready,
  /// Phase to determine which level to start
  ChoosingLevel,
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
  GameOver(GameOverReason),
}

#[derive(Copy, Debug, Clone, Eq,PartialEq, Hash)]
pub enum GameOverReason {
  OutOfBounds,
  OutOfTime,
  OutOfBoids
}

#[derive(Event, Debug)]
pub enum GameControlCommand {
  Initialize,
  StartGame,
  NextLevel,
  Retry,
}

pub fn process_game_control_commands(
  mut cmds: EventReader<GameControlCommand>,
  mut next_sim_state: ResMut<NextState<SimulationState>>,
  current_state: Res<State<SimulationState>>,
) {
  for cmd in cmds.read() {
    info!("{current_state:?} {cmd:?}");
    match ((*current_state).get(), cmd) {
      (SimulationState::Disabled, GameControlCommand::Initialize) => {
        // set state so exclusive system to register module runs
        next_sim_state.set(SimulationState::Initializing);
      }
      (SimulationState::Ready, GameControlCommand::StartGame) => {
        // signal that we want to load a level and play
        next_sim_state.set(SimulationState::ChoosingLevel)
      }
      (SimulationState::LevelComplete, GameControlCommand::NextLevel) => {
        // level has been completed, signal that we want to unload current level and load next level
        next_sim_state.set(SimulationState::ChoosingLevel)
      }
      (SimulationState::GameOver(_), GameControlCommand::Retry) => {
        next_sim_state.set(SimulationState::ChoosingLevel)
      }
      _ => {
        unimplemented!()
      }
    }
  }
}

pub fn wait_until_initialization_complete(
  qry: Query<Entity, With<Initializing>>,
  mut next_state: ResMut<NextState<SimulationState>>,
) {
  if qry.is_empty() {
    next_state.set(SimulationState::Ready);
  }
}
