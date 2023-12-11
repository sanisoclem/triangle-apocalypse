use std::time::Duration;

use bevy::prelude::*;
use bevy_smud::prelude::*;
use jam4::{level::LevelInfo, moveable::MoveableBounds};
use sdfu::SDF;

pub fn build_level(asset_server: &AssetServer) -> LevelInfo {
  let p = 150.;
  let h = p * 20.;
  let w = p * 20.;
  let border = 3000.;

  let fbounds = Vec4::new(0.0, h + (w - 1000.), w, w);

  let outer = sdfu::Box::new(Vec2::new(w + border, h + border));
  let inner = sdfu::Box::new(Vec2::new(w, h));
  let b1 = sdfu::Box::new((p * 6.0, p * 2.0).into()).translate((p * 4.0, p * -14.0).into());
  let b2 = sdfu::Box::new((p * 1.0, p * 3.0).into()).translate((p * 11.0, p * -5.0).into());
  let b3 = sdfu::Box::new((p * 2.0, p * 1.0).into()).translate((p * 15.0, p * -3.0).into());
  let b4 = sdfu::Box::new((p * 1.0, p * 3.0).into()).translate((p * 19.0, p * -5.0).into());

  let b5 = sdfu::Box::new((p * 2.0, p * 9.0).into()).translate((p * -8.0, p * -11.0).into());
  let b6 = sdfu::Box::new((p * 6.0, p * 3.0).into()).translate((p * 0.0, p * -5.0).into());
  let b7 = sdfu::Box::new((p * 1.0, p * 7.0).into()).translate((p * -15.0, p * -9.0).into());

  let b8 = sdfu::Box::new((p * 2.0, p * 2.0).into()).translate((p * -18.0, p * 12.0).into());
  let b9 = sdfu::Box::new((p * 1.0, p * 2.0).into()).translate((p * -19.0, p * 4.0).into());
  let b10 = sdfu::Box::new((p * 2.0, p * 2.0).into()).translate((p * -12.0, p * 4.0).into());
  let b11 = sdfu::Box::new((p * 1.0, p * 4.0).into()).translate((p * -11.0, p * 8.0).into());
  let b12 = sdfu::Box::new((p * 7.0, p * 1.0).into()).translate((p * -3.0, p * 13.0).into());
  let b13 = sdfu::Box::new((p * 1.0, p * 2.0).into()).translate((p * 3.0, p * 10.0).into());

  let b14 = sdfu::Box::new((p * 2.0, p * 3.0).into()).translate((p * -4.0, p * 5.0).into());
  let b15 = sdfu::Box::new((p * 6.0, p * 1.0).into()).translate((p * 4.0, p * 3.0).into());
  let b16 = sdfu::Box::new((p * 1.0, p * 5.0).into()).translate((p * 9.0, p * 7.0).into());
  let b17 = sdfu::Box::new((p * 3.0, p * 2.0).into()).translate((p * 13.0, p * 12.0).into());
  let b18 = sdfu::Box::new((p * 1.0, p * 2.0).into()).translate((p * 15.0, p * 4.0).into());

  let shape = outer.subtract(
    inner.subtract(
      b1.union(b2)
        .union(b3)
        .union(b4)
        .union(b5)
        .union(b6)
        .union(b7)
        .union(b8)
        .union(b9)
        .union(b10)
        .union(b11)
        .union(b12)
        .union(b13)
        .union(b14)
        .union(b15)
        .union(b16)
        .union(b17)
        .union(b18),
    ),
  );

  let finish_bounds = sdfu::Box::new(Vec2::new(fbounds.z, fbounds.w)).translate(fbounds.xy());
  let terrain_shader = asset_server.load("preload/terrain5.wgsl");
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
    name: "Level 5".to_owned(),
    next_level: None,
    starting_point: Vec2::new(p * 15., p * -14.0),
    boids_per_spawn_point: 10,
    spawn_points: vec![
      (p * 15., p * -6.0).into(),
      (p * -4.0 , p * -14.0).into(),
      (p * -18.0, p * -11.0).into(),
      (p * -16.0, p * 4.0).into(),
      (p * 3.0, p * 6.0).into(),
      (p * 12.0, p * 6.0).into(),
    ],
    rescue_goal: 20.into(),
    time_goal: Duration::from_secs(60).into(),
    wander: true,
  };
  lvl
}
