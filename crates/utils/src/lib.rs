use std::ops::Deref;

use bevy::{
  prelude::*,
  render::texture::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
};
use once_cell::sync::Lazy;

pub mod game_time;
// pub mod grid;
pub mod fps;
pub mod text;

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
