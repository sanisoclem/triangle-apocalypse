use std::time::Duration;

use bevy::prelude::*;
use bevy_smud::prelude::*;
use jam4::{level::LevelInfo, moveable::MoveableBounds};
use sdfu::SDF;

pub fn build_level(asset_server: &AssetServer) -> LevelInfo {
  let w = 4000.;
  let h = w * 2.0 + 1000.;
  let wp = 500.;
  let border = 3000.;
  let hw = (h - 2.0 * w)/2. + 200. ;

  let fbounds = Vec4::new(0.0, h + 4_000., 5_000., 5_000.);

  let outer = sdfu::Box::new(Vec2::new(w + border, h + border));
  let inner1 = sdfu::Box::new(Vec2::new(w, hw)).translate(Vec2::new(0.0, h -hw));
  let inner2 = sdfu::Box::new(Vec2::new(w, hw)).translate(Vec2::new(0.0, -h +hw));
  let inner = inner1.union(inner2);
  let mid_box = sdfu::Box::new(Vec2::new(wp, wp));

  let s1p = Vec2::new(0.0, -w);
  let s1a = sdfu::Circle::new( w).translate(s1p);
  let s1b = sdfu::Circle::new( w - wp).translate(s1p);
  let s1c = sdfu::Circle::new( w + wp).translate(s1p);
  // let s1 = smud::op_union(smud::op_subtract(s1a,s1c), s1b);

  let s2p = Vec2::new(0.0, w);
  let s2a = sdfu::Circle::new( w).translate(s2p);
  let s2b = sdfu::Circle::new( w - wp).translate(s2p);
  let s2c = sdfu::Circle::new( w + wp).translate(s2p);

  let sa = s1a.union(s2a);
  let sb = s1b.union(s2b);
  let sc = s1c.union(s2c);

  let shape = outer.subtract(sa.subtract(sb).union(mid_box).union(inner));

  let finish_bounds = sdfu::Box::new(Vec2::new(fbounds.z, fbounds.w)).translate(fbounds.xy());
  let terrain_shader = asset_server.load("preload/terrain4.wgsl");
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
    starting_point: Vec2::new(0.0, -h + 10.),
    boids_per_spawn_point: 40,
    spawn_points: vec![Vec2::new(-w * 0.75, -h), Vec2::new(w * 0.75, -h)],
    rescue_goal: 20.into(),
    time_goal: Duration::from_secs(60).into(),
    wander: false,
  };
  lvl
}
