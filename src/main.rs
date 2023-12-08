use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_hanabi::HanabiPlugin;
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

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

//mod audio;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn main_wasm() {
  main();
}

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins((
      utils::fps::ScreenDiagsTextPlugin,
      utils::text::TextAnimationPlugin,
    ))
    .add_state::<AppState>()
    .add_splash_screen(AppState::Splash, AppState::Game)
    .add_plugins(EguiPlugin)
    .add_plugins(WorldInspectorPlugin::default())
    .add_plugins(HanabiPlugin)
    .add_jam_game()
    .add_game(AppState::Game)
    .run();
}
