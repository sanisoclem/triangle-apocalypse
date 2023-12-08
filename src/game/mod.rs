use bevy::{prelude::*, sprite::Material2dPlugin};

use camera::*;

use jam4::SimulationState;
use utils::despawn_screen;

use self::{
  controls::{
    calc_player_direction, setup_player_ui, toggle_player_mode, update_player_ui, InPlayingScreen,
  },
  game_over::{on_game_over, wait_to_retry, InGameOverScreen},
  gg::on_game_complete,
  grid::{build_grid, GridMaterial},
  level_complete::{setup_level_complete, wait_to_next_level, InLevelCompleteScreen},
};

mod controls;
mod game_over;
mod gg;
mod level_complete;

mod camera;
#[cfg(feature = "debug")]
mod debug;
mod grid;

#[derive(Resource)]
struct GameNextState<T>(T);
pub trait GameExtensions {
  fn add_game<T: States + Copy>(&mut self, game_state: T) -> &mut Self;
}

impl GameExtensions for App {
  fn add_game<T: States + Copy>(&mut self, game_state: T) -> &mut Self {
    self
      .add_plugins(Material2dPlugin::<GridMaterial>::default())
      .add_systems(OnEnter(game_state), setup_camera)
      .add_systems(OnEnter(SimulationState::GameComplete), on_game_complete)
      .add_systems(
        OnExit(SimulationState::GameComplete),
        despawn_screen::<InLevelCompleteScreen>,
      )
      .add_systems(OnEnter(SimulationState::GameOver), on_game_over)
      .add_systems(
        OnExit(SimulationState::GameOver),
        despawn_screen::<InGameOverScreen>,
      )
      .add_systems(
        OnEnter(SimulationState::LevelComplete),
        setup_level_complete,
      )
      .add_systems(
        OnExit(SimulationState::LevelComplete),
        despawn_screen::<InLevelCompleteScreen>,
      )
      .add_systems(
        OnEnter(SimulationState::Simulating),
        (setup_player_ui, build_grid),
      )
      .add_systems(
        OnExit(SimulationState::Simulating),
        despawn_screen::<InPlayingScreen>,
      )
      .add_systems(
        Update,
        (
          (
            calc_player_direction,
            toggle_player_mode,
            follow_player,
            update_player_ui,
          )
            .run_if(in_state(SimulationState::Simulating)),
          wait_to_retry.run_if(in_state(SimulationState::GameOver)),
          wait_to_next_level.run_if(in_state(SimulationState::LevelComplete)),
        ),
      );
    #[cfg(feature = "debug")]
    self.add_systems(
      Update,
      debug::boid_config_debug.run_if(in_state(SimulationState::Simulating)),
    );

    self
  }
}
