use std::time::Duration;

use bevy::prelude::*;
use bevy_smud::prelude::*;
use jam4::{level::LevelInfo, moveable::MoveableBounds};
use sdfu::SDF;

pub fn build_level1(asset_server: &AssetServer) -> LevelInfo {
  let w = 2000.;
  let h = 10000.;
  let outer = sdfu::Box::new(Vec2::new(w + 3000., h + 3000.));
  let inner = sdfu::Box::new(Vec2::new(w, h));
  let t1 = sdfu::Triangle::new([Vec2::new(w, 0.), Vec2::new(w, -h), Vec2::new(500., -h)]);
  let t2 = sdfu::Triangle::new([Vec2::new(-w, 0.), Vec2::new(-w, -h), Vec2::new(-500., -h)]);
  let t3 = sdfu::Triangle::new([Vec2::new(w, 0.), Vec2::new(w, h), Vec2::new(500., h)]);
  let t4 = sdfu::Triangle::new([Vec2::new(-w, 0.), Vec2::new(-w, h), Vec2::new(-500., h)]);
  let shape = outer
    .subtract(inner)
    .union(t1.union(t2).union(t3).union(t4));
  let finish_bounds = sdfu::Box::new(Vec2::new(3000., 1000.)).translate(Vec2::new(0.0, 10000.));
  let terrain_shader = asset_server.load("preload/terrain.wgsl");
  let terrain_finish_shader = asset_server.load("preload/terrain_finish.wgsl");

  let s = SmudShape {
    color: Color::BLACK,
    sdf: terrain_shader,
    frame: Frame::Quad(50000.),
    ..default()
  };
  let fs = SmudShape {
    color: utils::colors::FAIRY.with_a(0.2),
    sdf: terrain_finish_shader,
    frame: Frame::Quad(3000.),
    ..default()
  };

  let lvl = LevelInfo {
    bounds: MoveableBounds::from_sdf(shape),
    finish_bounds: MoveableBounds::from_sdf(finish_bounds),
    finish_bounds_sdf: Some((fs, Vec2::new(0.0, 10000.))),
    bounds_sdf: Some(s),
    name: "Level 1".to_owned(),
    next_level: None,
    starting_point: Vec2::new(0.0, -10000.),
    // starting_point: Vec2::ZERO,
    boids_per_spawn_point: 20,
    spawn_points: vec![Vec2::new(0., 1000.), Vec2::new(0., -1000.)],
    rescue_goal: 10.into(),
    time_goal: Duration::from_secs(120).into(),
    wander: false,
  };
  lvl
}