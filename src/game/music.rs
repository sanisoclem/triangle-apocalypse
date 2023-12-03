use bevy::prelude::*;

pub fn setup_music(mut cmd: Commands, asset_server: Res<AssetServer>) {
  cmd.spawn(AudioBundle {
    source: asset_server.load("preload/battle_1.ogg"),
    settings: PlaybackSettings::LOOP,
    ..default()
  });
}
