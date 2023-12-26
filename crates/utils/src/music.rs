use crate::lerp;
use bevy::{audio::AddAudioSource, prelude::*};
use std::{
  sync::{
    mpsc::{Receiver, SyncSender},
    Mutex,
  },
  time::Duration,
};

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

#[derive(Asset, TypePath)]
pub struct ProcessedAudio {
  pub sources: Vec<AudioSource>,
  pub process: fn(sources: &Vec<AudioSource>) -> Box<dyn Source<Item = i16> + Sync + Send>,
  pub sample_receiver: Option<Mutex<Receiver<[i16; AUDIO_ANALYZER_SAMPLE_SIZE]>>>,
  pub sample_sender: Option<SyncSender<[i16; AUDIO_ANALYZER_SAMPLE_SIZE]>>,
  pub sample_rate: f32,
}

impl ProcessedAudio {
  pub fn new_sampled(
    sources: Vec<AudioSource>,
    sample_rate: f32,
    process: fn(sources: &Vec<AudioSource>) -> Box<dyn Source<Item = i16> + Sync + Send>,
  ) -> Self {
    let (tx, rx) = std::sync::mpsc::sync_channel::<[i16; AUDIO_ANALYZER_SAMPLE_SIZE]>(10);
    Self {
      sources,
      process,
      sample_receiver: Some(Mutex::new(rx)),
      sample_sender: Some(tx),
      sample_rate,
    }
  }
  pub fn new(
    sources: Vec<AudioSource>,
    process: fn(sources: &Vec<AudioSource>) -> Box<dyn Source<Item = i16> + Sync + Send>,
  ) -> Self {
    Self {
      sources,
      process,
      sample_receiver: None,
      sample_sender: None,
      sample_rate: 0.,
    }
  }
}

impl Decodable for ProcessedAudio {
  type DecoderItem = i16;
  type Decoder = Box<dyn Source<Item = i16> + Sync + Send>;
  fn decoder(&self) -> Self::Decoder {
    if let Some(tx) = &self.sample_sender {
      Box::new(AnalyzerDecoder::new(
        (self.process)(&self.sources),
        self.sample_rate,
        tx.clone(),
      ))
    } else {
      (self.process)(&self.sources)
    }
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

pub const AUDIO_ANALYZER_SAMPLE_SIZE: usize = 4096;

pub struct AnalyzerDecoder<D, S> {
  pub inner: D,
  sample_buffer: [S; AUDIO_ANALYZER_SAMPLE_SIZE], // no variadic generics, hard code sample size
  buffer_index: usize,
  samples_to_skip: usize,
  samples_skipped: usize,
  enabled: bool,
  sender: Option<std::sync::mpsc::SyncSender<[S; AUDIO_ANALYZER_SAMPLE_SIZE]>>,
}

impl<D> AnalyzerDecoder<D, i16> {
  pub fn no_sampling(inner: D) -> AnalyzerDecoder<D, i16> {
    Self {
      inner,
      enabled: false,
      samples_skipped: 0,
      samples_to_skip: 0,
      sample_buffer: [0; AUDIO_ANALYZER_SAMPLE_SIZE],
      buffer_index: 0,
      sender: None,
    }
  }
  pub fn new(
    inner: D,
    sample_rate: f32,
    sender: std::sync::mpsc::SyncSender<[i16; AUDIO_ANALYZER_SAMPLE_SIZE]>,
  ) -> AnalyzerDecoder<D, i16> {
    let cycle_length = (AUDIO_ANALYZER_SAMPLE_SIZE as f32 / sample_rate).floor() as usize;

    Self {
      inner,
      enabled: true,
      samples_skipped: 0,
      samples_to_skip: cycle_length - AUDIO_ANALYZER_SAMPLE_SIZE,
      sample_buffer: [0; AUDIO_ANALYZER_SAMPLE_SIZE],
      buffer_index: 0,
      sender: Some(sender),
    }
  }
}

impl<D> Iterator for AnalyzerDecoder<D, i16>
where
  D: Iterator<Item = i16>,
{
  type Item = i16;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    let Some(sample) = self.inner.next() else {
      return None;
    };

    if !self.enabled {
      // no sampling
    } else if self.samples_skipped < self.samples_to_skip {
      self.samples_skipped += 1;
    } else if self.buffer_index < AUDIO_ANALYZER_SAMPLE_SIZE {
      self.sample_buffer[self.buffer_index] = sample;
      self.buffer_index += 1;
    } else {
      self.samples_skipped = 0;
      self.buffer_index = 0;
      if let Some(sender) = &self.sender {
        if let Err(e) = sender.send(self.sample_buffer.clone()) {
          warn!("unable to send audio samples: {:?}", e);
        }
      }
    }
    return Some(sample);
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.inner.size_hint()
  }
}

impl<D> Source for AnalyzerDecoder<D, i16>
where
  D: Source<Item = i16>,
{
  fn current_frame_len(&self) -> Option<usize> {
    self.inner.current_frame_len()
  }

  fn channels(&self) -> u16 {
    self.inner.channels()
  }

  fn sample_rate(&self) -> u32 {
    self.inner.sample_rate()
  }

  fn total_duration(&self) -> Option<Duration> {
    self.inner.total_duration()
  }
}
