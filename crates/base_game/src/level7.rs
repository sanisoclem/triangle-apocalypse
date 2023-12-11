use std::time::Duration;

use bevy::{math::vec2, prelude::*};
use bevy_smud::prelude::*;
use jam4::{level::LevelInfo, moveable::MoveableBounds};
use sdfu::SDF;

use crate::sdf::{build_maze, build_track, Flip};

pub fn build_level(asset_server: &AssetServer) -> LevelInfo {
  let p = 200.;
  let w = p * 20.;
  let mh = p * 20.;
  let th = 10000.0;
  let h = th + mh + mh;
  let border = 3000.;

  let fbounds = Vec4::new(0.0, h + (w - 1000.), w, w);
  let outer = sdfu::Box::new(Vec2::new(w + border, h + border));
  let inner = sdfu::Box::new(Vec2::new(w, h));

  let m1 = Flip::new(build_maze(p)).translate(Vec2::new(0.0, (th + mh / 1.0)));
  let m2 = build_maze(p).translate(Vec2::new(0.0, (-th - mh / 1.0)));
  let t1 = build_track(w, th);

  let shape = outer.subtract(inner.subtract(m1.union(m2).union(t1)));

  let finish_bounds = sdfu::Box::new(Vec2::new(fbounds.z, fbounds.w)).translate(fbounds.xy());
  let terrain_shader = asset_server.load("preload/terrain7.wgsl");
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
    name: "Level 7".to_owned(),
    next_level: None,
    starting_point: Vec2::new(p * 15., p * -14.0 - (th + mh)),
    boids_per_spawn_point: 40,
    spawn_points: vec![
      (p * 15., p * -6.0),
      (p * -4.0, p * -14.0),
      (p * -18.0, p * -11.0),
      (p * -16.0, p * 4.0),
      (p * 3.0, p * 6.0),
      (p * 12.0, p * 6.0),
    ]
    .into_iter()
    .map(|(x, y)| Vec2::new(x, y - (th + mh)))
    .collect(),
    rescue_goal: 100.into(),
    time_goal: Duration::from_secs(120).into(),
    wander: true,
  };
  lvl
}
