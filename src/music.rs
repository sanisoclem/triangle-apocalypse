use std::{sync::Arc, time::Duration};

use bevy::{asset::LoadState, prelude::*};
use rodio::source::{from_iter, Source};
use utils::lerp;

#[derive(Component, Default)]
pub struct FadeTo {
  pub target: Option<(f32, f32)>,
  pub remaining: Duration,
  pub fade_length: Duration,
}

#[derive(Component)]
pub enum BgMusic {
  MainTheme,
  Menu,
  GameOver,
}

#[derive(Resource)]
pub struct Jukebox {
  main_theme_start: Handle<AudioSource>,
  main_theme_loop: Handle<AudioSource>,
  menu: Handle<AudioSource>,
  game_over: Handle<AudioSource>,
  init_result: Option<InitResult>,
}

struct InitResult {
  main_theme: Handle<ProcessedAudio>,
  menu_theme: Handle<ProcessedAudio>,
  game_over_theme: Handle<ProcessedAudio>,
}

trait NewTrait: bevy::audio::Source + Sync + CloneBox + Send + Iterator<Item = i16> {}
trait CloneBox {
  fn clone_box(&self) -> Box<dyn NewTrait + Sync + Send + 'static>;
}

impl<T> CloneBox for T
where
  T: 'static + NewTrait + Clone,
{
  fn clone_box(&self) -> Box<dyn NewTrait + Sync + Send + 'static> {
    Box::new(self.clone())
  }
}

impl Clone for Box<dyn NewTrait + Sync + Send + 'static> {
  fn clone(&self) -> Box<dyn NewTrait + Sync + Send + 'static> {
    self.clone_box()
  }
}

impl<T> NewTrait for T where
  T: 'static + bevy::audio::Source + Clone + Sync + Send + Iterator<Item = i16>
{
}

impl Source for Box<dyn NewTrait + Send + Sync + 'static> {
  #[inline]
  fn current_frame_len(&self) -> Option<usize> {
    (**self).current_frame_len()
  }

  #[inline]
  fn channels(&self) -> u16 {
    (**self).channels()
  }

  #[inline]
  fn sample_rate(&self) -> u32 {
    (**self).sample_rate()
  }

  #[inline]
  fn total_duration(&self) -> Option<Duration> {
    (**self).total_duration()
  }
}

#[derive(Asset, TypePath, Clone)]
pub struct ProcessedAudio {
  pub stream: Box<dyn NewTrait + Sync + Send>,
}

impl Decodable for ProcessedAudio {
  type DecoderItem = i16;
  type Decoder = Box<dyn NewTrait + Sync + Send>;
  fn decoder(&self) -> Self::Decoder {
    self.stream.clone()
  }
}

impl FromWorld for Jukebox {
  fn from_world(world: &mut World) -> Self {
    let asset_server = world.get_resource::<AssetServer>().unwrap();

    let main_theme_start = asset_server.load("preload/battle_0.ogg");
    let main_theme_loop = asset_server.load("preload/battle_1.ogg");
    let menu = asset_server.load("preload/menu_full.ogg");
    let game_over = asset_server.load("preload/menu_full.ogg");
    Self {
      main_theme_start,
      main_theme_loop,
      menu,
      game_over,
      init_result: None,
    }
  }
}

impl Jukebox {
  pub fn try_initialize(
    &mut self,
    cmd: &mut Commands,
    asset_server: &AssetServer,
    audio: &Assets<AudioSource>,
  ) -> bool {
    let Some(main_theme_start_state) = asset_server.get_load_state(self.main_theme_start.clone())
    else {
      return false;
    };
    let Some(main_theme_loop_state) = asset_server.get_load_state(self.main_theme_loop.clone())
    else {
      return false;
    };
    let Some(game_over_state) = asset_server.get_load_state(self.game_over.clone()) else {
      return false;
    };
    let Some(menu_state) = asset_server.get_load_state(self.menu.clone()) else {
      return false;
    };
    let main_theme_loaded =
      main_theme_start_state == LoadState::Loaded && main_theme_loop_state == LoadState::Loaded;
    let menu_loaded = menu_state == LoadState::Loaded;
    let game_over_loaded = game_over_state == LoadState::Loaded;

    let Some(start) = audio.get(self.main_theme_start.clone()) else {
      return false;
    };
    let Some(battle_loop) = audio.get(self.main_theme_loop.clone()) else {
      return false;
    };
    let Some(menu) = audio.get(self.menu.clone()) else {
      return false;
    };
    let Some(game_over) = audio.get(self.game_over.clone()) else {
      return false;
    };
    let srcs: Vec<Box<dyn Source<Item = i16> + Send + Sync>> = vec![
      Box::new(start.decoder().delay(Duration::from_secs(2))),
      Box::new(battle_loop.decoder().repeat_infinite()),
    ];
    let g = from_iter(srcs);
    g.cloned()
    let main_theme = asset_server.add(ProcessedAudio {
      stream: Box::new(),
    });
    let menu_theme = asset_server.add(ProcessedAudio {
      stream: Box::new(menu.decoder().fade_in(Duration::from_secs(1))),
    });
    let game_over_theme = asset_server.add(ProcessedAudio {
      stream: Box::new(game_over.decoder().fade_in(Duration::from_secs(1))),
    });

    // cmd.spawn(AudioSourceBundle)

    self.init_result = Some(InitResult {
      main_theme,
      menu_theme,
      game_over_theme,
    });
    return main_theme_loaded && menu_loaded && game_over_loaded;
  }

  pub fn play_main(w: &mut World) {}
  pub fn play_menu() {}
  pub fn play_game_over() {}
}

fn fade(mut qry: Query<(Entity, &AudioSink, &mut FadeTo)>, dt: Res<Time>) {
  for (e, sink, mut fade) in qry.iter_mut() {
    if let Some((target, from)) = fade.target {
      let new_remaining = if dt.delta() >= fade.remaining {
        Duration::ZERO
      } else {
        fade.remaining - dt.delta()
      };
      let progress = 1.0 - (new_remaining.as_secs_f32() / fade.fade_length.as_secs_f32());
      let new_volume = lerp(from, target, progress);
      sink.set_volume(new_volume);

      fade.remaining = new_remaining;
      if new_remaining == Duration::ZERO {
        fade.target = None;
      }

      if !sink.is_paused() && sink.volume() <= 0.01 {
        sink.pause();
      }
      if sink.is_paused() && sink.volume() > 0.01 {
        sink.play()
      }
    }
  }
}
