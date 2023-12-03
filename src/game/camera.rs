use bevy::{prelude::*, core_pipeline::{tonemapping::Tonemapping, bloom::BloomSettings}};
use jam4::Player;

#[derive(Component, Default)]
pub struct PlayerCamera;

pub fn follow_player(
  qry_transform: Query<&Transform, (With<Player>, Without<Camera>)>,
  mut qry_camera: Query<
    (&mut Transform, &mut PlayerCamera),
    (Without<Player>, With<Camera>),
  >,
  time: Res<Time>,
) {
  for (mut cam_transform, mut cam) in qry_camera.iter_mut() {
    if let Ok(target_transform) = qry_transform.get_single() {
      // TODO
    }
  }
}

pub fn setup_camera(mut cmd: Commands) {
  cmd.spawn((
    Camera2dBundle {
      camera: Camera {
        hdr: true, // 1. HDR is required for bloom
        ..default()
      },
      tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
      ..default()
    },
    BloomSettings::default(), // 3. Enable bloom for the camera
  ));
}
