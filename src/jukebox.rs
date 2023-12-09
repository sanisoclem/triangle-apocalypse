use std::time::Duration;

use bevy::prelude::*;
use rodio::source::{from_iter, Source};
use utils::music::{FadeTo, ProcessedAudio};

pub trait JukeboxExtensions {
  fn add_jukebox(&mut self) -> &mut Self;
}

impl JukeboxExtensions for App {
  fn add_jukebox(&mut self) -> &mut Self {
    self
      .init_resource::<Jukebox>()
      .add_event::<MusicCommand>()
      .add_systems(
        Update,
        (wait_for_jukebox_init, process_music_commands).chain(),
      )
  }
}

#[derive(Component, Copy, Clone, PartialEq, Debug)]
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

pub struct InitResult {
  pub main_theme: Handle<ProcessedAudio>,
  pub game_over_theme: Handle<ProcessedAudio>,
  pub menu_theme: Handle<ProcessedAudio>,
}

#[derive(Event)]
pub enum MusicCommand {
  Play(BgMusic),
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
  pub fn try_initialize(&mut self, asset_server: &AssetServer, audio: &Assets<AudioSource>) {
    let Some(start) = audio.get(self.main_theme_start.clone()) else {
      return;
    };
    let Some(battle_loop) = audio.get(self.main_theme_loop.clone()) else {
      return;
    };
    let Some(menu) = audio.get(self.menu.clone()) else {
      return;
    };
    let Some(game_over) = audio.get(self.game_over.clone()) else {
      return;
    };

    let main_theme = asset_server.add(ProcessedAudio {
      sources: vec![start.clone(), battle_loop.clone()], // wow
      process: |sources| {
        let copy: Vec<_> = sources
          .iter()
          .enumerate()
          .map(|(i, f)| -> Box<dyn Source<Item = i16> + Send + Sync> {
            if i == 1 {
              Box::new(f.decoder().repeat_infinite())
            } else {
              Box::new(f.decoder())
            }
          })
          .collect();
        Box::new(from_iter(copy.into_iter()))
      },
    });
    let menu_theme = asset_server.add(ProcessedAudio {
      sources: vec![menu.clone()], // wow
      process: |sources| {
        Box::new(
          sources
            .first()
            .unwrap()
            .decoder()
            .repeat_infinite()
            .delay(Duration::from_secs(1)),
        )
      },
    });
    let game_over_theme = asset_server.add(ProcessedAudio {
      sources: vec![game_over.clone()], // wow
      process: |sources| Box::new(sources.first().unwrap().decoder()),
    });

    self.init_result = Some(InitResult {
      game_over_theme,
      main_theme,
      menu_theme,
    });
  }
}

pub fn process_music_commands(
  qry: Query<(Entity, &BgMusic), With<BgMusic>>,
  mut commands: Commands,
  mut cmds: EventReader<MusicCommand>,
  jukebox: Res<Jukebox>,
) {
  let Some(ir) = &jukebox.init_result else {
    return;
  };
  let mut started = false;
  for cmd in cmds.read() {
    if started {
      break;
    }
    let MusicCommand::Play(theme) = cmd;

    let src = match theme {
      BgMusic::MainTheme => &ir.main_theme,
      BgMusic::GameOver => &ir.game_over_theme,
      BgMusic::Menu => &ir.menu_theme,
    };

    // look for currently playing music that matches
    for (e, b) in qry.iter() {
      if b == theme {
        info!("found match {:?}", theme);
        commands
          .entity(e)
          .insert(FadeTo::fade_in(Duration::from_secs_f32(1.0)));
        started = true;
        continue;
      }
      info!("start despawn {:?} {:?}", b, e);
      commands
        .entity(e)
        .insert(FadeTo::fade_out_despawn(Duration::from_secs_f32(0.2)));
    }
    if !started {
      info!("spawning {:?}", theme);
      commands.spawn((
        AudioSourceBundle {
          source: src.clone(),
          settings: PlaybackSettings::LOOP,
        },
        *theme,
      ));
      started = true;
    }
  }
}
pub fn wait_for_jukebox_init(
  mut jukebox: ResMut<Jukebox>,
  asset_server: Res<AssetServer>,
  audio: Res<Assets<AudioSource>>,
) {
  if jukebox.init_result.is_some() {
    return;
  }
  jukebox.try_initialize(&asset_server, &audio);
}
