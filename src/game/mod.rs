use bevy::prelude::*;
use camera::*;
use music::*;

use jam4::SimulationState;
mod camera;
mod music;

#[derive(Resource)]
struct GameNextState<T>(T);
pub trait GameExtensions {
  fn add_game<T: States + Copy>(&mut self, game_state: T) -> &mut Self;
}

impl GameExtensions for App {
  fn add_game<T: States + Copy>(&mut self, game_state: T) -> &mut Self {
    self
      .add_systems(OnEnter(game_state), (setup_music,setup_camera))
      .add_systems(Update, follow_player.run_if(in_state(SimulationState::Simulating)) )
    //     .add_player_camera()
    //     .add_music()
    //     .add_player_ui()
  }
}
