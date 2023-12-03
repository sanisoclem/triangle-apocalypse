use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::GameExtensions;
use splash::SplashExtensions;
use jam4::Jam4Extensions;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum AppState {
  #[default]
  Splash,
  Game,
}

mod splash;
mod game;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_state::<AppState>()
    .add_splash_screen(AppState::Splash, AppState::Game)
    .add_plugins(WorldInspectorPlugin::default())
    .add_jam_game()
    .add_game(AppState::Game)
    .run();
}
