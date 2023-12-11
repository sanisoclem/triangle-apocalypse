use std::time::Duration;

use bevy::prelude::*;
use bevy_smud::prelude::*;
use jam4::{level::LevelInfo, moveable::MoveableBounds};
use sdfu::SDF;

pub fn build_level(asset_server: &AssetServer) -> LevelInfo {
  let w = 500.;
  let h = 10000.;
  let fbounds = Vec4::new(0.0, 10_000., 5_000., 5_000.);

  let outer = sdfu::Box::new(Vec2::new(w + 3000., h + 3000.));
  let inner = sdfu::Box::new(Vec2::new(w, h));
  let shape = outer.subtract(inner);

  let finish_bounds = sdfu::Box::new(Vec2::new(fbounds.z, fbounds.w)).translate(fbounds.xy());
  let terrain_shader = asset_server.load("preload/terrain3.wgsl");
  let fill_shader = asset_server.load("preload/terrain_fill.wgsl");

  let s = SmudShape {
    color: Color::BLACK,
    sdf: terrain_shader,
    frame: Frame::Quad(50000.),
    fill: fill_shader,
  };

  let lvl = LevelInfo {
    bounds: MoveableBounds::from_sdf(shape.subtract(finish_bounds)),
    finish_bounds_box: MoveableBounds::from_sdf(finish_bounds),
    finish_bounds: fbounds,
    bounds_sdf: Some(s),
    name: "Level 3".to_owned(),
    next_level: None,
    starting_point: Vec2::new(0.0, -h),
    boids_per_spawn_point: 40,
    spawn_points: (-2..2)
      .map(|x| Vec2::new(0., h * x as f32 / 4.0))
      .collect(),
    rescue_goal: 100.into(),
    time_goal: Duration::from_secs(120).into(),
    wander: false,
  };
  lvl
}
