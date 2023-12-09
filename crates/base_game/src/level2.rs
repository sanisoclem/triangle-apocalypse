use std::time::Duration;

use bevy::prelude::*;
use bevy_smud::prelude::*;
use jam4::{level::LevelInfo, moveable::MoveableBounds};
use sdfu::SDF;

pub fn build_level2(asset_server: &AssetServer) -> LevelInfo {
  let outer = sdfu::Box::new(Vec2::new(2000., 10000.));
  let inner = sdfu::Box::new(Vec2::new(1000., 9000.));
  let m1 = sdfu::Circle::new(200.0).translate(Vec2::new(300., 200.));
  let m2 = sdfu::Circle::new(150.).translate(Vec2::new(-200., 300.));
  let m3 = sdfu::Circle::new(125.).translate(Vec2::new(300., -300.));
  let m4 = sdfu::Circle::new(50.).translate(Vec2::new(100., 1500.));
  let shape = outer.subtract(inner.subtract(m1).subtract(m2).subtract(m3).subtract(m4));
  let finish_bounds = sdfu::Box::new(Vec2::new(2000., 1000.)).translate(Vec2::new(0.0, 9000.));
  let terrain_shader = asset_server.load("preload/terrain_2.wgsl");
  let terrain_finish_shader = asset_server.load("preload/terrain_finish_2.wgsl");

  let s = SmudShape {
    color: Color::TOMATO,
    sdf: terrain_shader,
    frame: Frame::Quad(30000.),
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
    finish_bounds_sdf: Some((fs, Vec2::new(0.0, 9000.))),
    bounds_sdf: Some(s),
    name: "Level 2".to_owned(),
    next_level: None,
    starting_point: Vec2::ZERO,
    boids_per_spawn_point: 10,
    spawn_points: vec![Vec2::new(0., 300.), Vec2::new(0., 600.)],
    rescue_goal: 10.into(),
    time_goal: Duration::from_secs(60).into(),
  };
  lvl
}
