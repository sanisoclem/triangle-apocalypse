use bevy::prelude::*;
use bevy_hanabi::HanabiPlugin;
use game::GameExtensions;
use jam4::Jam4Extensions;
use splash::SplashExtensions;

#[cfg(feature = "debug")]
use bevy_egui::EguiPlugin;
#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum AppState {
  #[default]
  Splash,
  Game,
}

mod game;
mod splash;
mod music;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

//mod audio;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn main_wasm() {
  main();
}

fn main() {
  let mut app = App::new();
  app
    .add_plugins(DefaultPlugins)
    .add_plugins((

      utils::text::TextAnimationPlugin,
    ))
    .add_state::<AppState>()
    .add_splash_screen(AppState::Splash, AppState::Game)
    .add_plugins(HanabiPlugin)
    .add_jam_game()
    .add_game(AppState::Game);

  #[cfg(feature = "debug")]
  app
    .add_plugins((EguiPlugin,utils::fps::ScreenDiagsTextPlugin,WorldInspectorPlugin::default()));

  app.run();
}
