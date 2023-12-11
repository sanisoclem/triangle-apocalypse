use bevy::{
  core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
  prelude::*,
  render::camera::ScalingMode,
};
use jam4::{boid::Boid, Player};

#[derive(Component, Default)]
pub struct PlayerCamera;

pub fn follow_player(
  qry_transform: Query<(&Transform, &Boid), (With<Player>, Without<Camera>)>,
  mut qry_camera: Query<&mut Transform, (Without<Player>, With<Camera>)>,
  time: Res<Time>,
) {
  for mut cam_transform in qry_camera.iter_mut() {
    if let Ok((target_transform, boid)) = qry_transform.get_single() {
      let target = (target_transform.translation.xy() + boid.direction * boid.speed )
        .extend(cam_transform.translation.z);

      let lookahead = 2000.;
      if target.distance_squared(cam_transform.translation) > lookahead * lookahead  {
        cam_transform.translation = target;
      } else {
        cam_transform.translation = cam_transform.translation.lerp(target, time.delta_seconds()*1.2);
      }
    }
  }
}

pub fn setup_camera(mut cmd: Commands) {
  let mut cam = Camera2dBundle {
    camera: Camera {
      hdr: true, // 1. HDR is required for bloom
      ..default()
    },
    tonemapping: Tonemapping::TonyMcMapface, /* 2. Using a tonemapper that desaturates to white
                                              *    is recommended */
    ..default()
  };
  cam.projection.scaling_mode = ScalingMode::FixedVertical(2000.0);
  cmd.spawn((cam, BloomSettings::default()));
}
