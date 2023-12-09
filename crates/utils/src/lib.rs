use std::ops::{Deref, Mul, Sub, Add};

use bevy::{
  prelude::*,
  render::texture::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
};
use once_cell::sync::Lazy;

pub mod game_time;
// pub mod grid;
pub mod fps;
pub mod text;
pub mod colors;

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
  for entity in &to_despawn {
    commands.entity(entity).despawn_recursive();
  }
}

static TEX_SAMPLER_DESC_TILED: Lazy<ImageSamplerDescriptor> =
  Lazy::new(|| ImageSamplerDescriptor {
    address_mode_u: ImageAddressMode::Repeat,
    address_mode_v: ImageAddressMode::Repeat,
    ..Default::default()
  });

pub fn tex_settings_tiled(s: &mut ImageLoaderSettings) {
  s.sampler = ImageSampler::Descriptor(TEX_SAMPLER_DESC_TILED.deref().clone());
}


pub fn lerp<T: Copy + Mul<f32, Output = T> + Sub<T, Output = T> + Add<T, Output = T>>(
  from: T,
  to: T,
  f: f32,
) -> T {
  from + ((to - from) * f)
}
