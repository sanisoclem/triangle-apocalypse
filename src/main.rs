use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::GameExtensions;
use jam4::Jam4Extensions;
use splash::SplashExtensions;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum AppState {
  #[default]
  Splash,
  Game,
}

mod game;
mod splash;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(utils::fps::ScreenDiagsTextPlugin)
    .add_state::<AppState>()
    .add_splash_screen(AppState::Splash, AppState::Game)
    .add_plugins(WorldInspectorPlugin::default())
    .add_jam_game()
    .add_game(AppState::Game)
    .run();
}
