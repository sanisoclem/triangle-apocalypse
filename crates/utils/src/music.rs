use crate::lerp;
use bevy::{audio::AddAudioSource, prelude::*};
use std::time::Duration;

use rodio::source::Source;

pub struct MusicPlugin;

impl Plugin for MusicPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_audio_source::<ProcessedAudio>()
      .add_systems(Update, fade);
  }
}

#[derive(Component, Default)]
pub struct FadeTo {
  pub target: Option<f32>,
  pub fade_length: Duration,
  pub despawn_on_zero: bool,
  pub linger: Timer,
}

impl FadeTo {
  pub fn fade_in(duration: Duration) -> Self {
    Self {
      target: Some(1.0),
      fade_length: duration,
      despawn_on_zero: false,
      linger: Timer::from_seconds(3.0, TimerMode::Once),
    }
  }
  pub fn fade_out_despawn(duration: Duration) -> Self {
    Self {
      target: Some(0.0),
      fade_length: duration,
      despawn_on_zero: true,
      linger: Timer::from_seconds(3.0, TimerMode::Once),
    }
  }
}

#[derive(Asset, TypePath, Clone)]
pub struct ProcessedAudio {
  pub sources: Vec<AudioSource>,
  pub process: fn(sources: &Vec<AudioSource>) -> Box<dyn Source<Item = i16> + Sync + Send>,
}

impl Decodable for ProcessedAudio {
  type DecoderItem = i16;
  type Decoder = Box<dyn Source<Item = i16> + Sync + Send>;
  fn decoder(&self) -> Self::Decoder {
    (self.process)(&self.sources)
  }
}

pub fn fade(mut cmd: Commands, mut qry: Query<(Entity, &AudioSink, &mut FadeTo)>, dt: Res<Time>) {
  for (e, sink, mut fade) in qry.iter_mut() {
    if let Some(target) = fade.target {
      let new_volume = lerp(
        sink.volume(),
        target,
        dt.delta_seconds() / fade.fade_length.as_secs_f32(),
      );
      sink.set_volume(new_volume);
      let target_reached = (new_volume - target).abs() <= 0.001;

      if target_reached {
        info!("target reached {:?}", e);
        fade.target = None;
      }
    }

    if sink.volume() <= 0.001 {
      if fade.despawn_on_zero {
        fade.linger.tick(dt.delta());
        if fade.linger.just_finished() {
          info!("depawned {:?}", e);
          cmd.entity(e).despawn();
        }
      } else if !sink.is_paused() {
        sink.pause();
      }
    }
    if sink.is_paused() && sink.volume() > 0.001 {
      sink.play()
    }
  }
}
