use std::time::Duration;

use bevy::prelude::*;
use bevy_smud::prelude::*;
use jam4::{level::LevelInfo, moveable::MoveableBounds};
use sdfu::{ops::HardMin, Dim2D, Triangle, SDF};

fn build_track(
  w: f32,
  h: f32,
) -> sdfu::ops::Union<
  f32,
  sdfu::ops::Union<
    f32,
    sdfu::ops::Union<
      f32,
      sdfu::ops::Union<
        f32,
        Triangle<bevy::prelude::Vec2, Dim2D>,
        Triangle<bevy::prelude::Vec2, Dim2D>,
        HardMin<f32>,
      >,
      Triangle<bevy::prelude::Vec2, Dim2D>,
      HardMin<f32>,
    >,
    Triangle<bevy::prelude::Vec2, Dim2D>,
    HardMin<f32>,
  >,
  Triangle<bevy::prelude::Vec2, Dim2D>,
  HardMin<f32>,
> {
  let angle = f32::atan(3. * (w / 2.) / h);
  let h2 = w / (2.0 * angle.tan());

  let t1 = sdfu::Triangle::new([Vec2::new(0.0, h2), Vec2::new(0.0, -h2), Vec2::new(w, h)]);
  let t2 = sdfu::Triangle::new([Vec2::new(0.0, -h2), Vec2::new(w, h), Vec2::new(w, -h)]);
  let t3 = sdfu::Triangle::new([Vec2::new(-w, 0.), Vec2::new(-w, h), Vec2::new(w / 2., h)]);
  let t4 = sdfu::Triangle::new([Vec2::new(-w, 0.), Vec2::new(-w, -h), Vec2::new(w / 2., -h)]);
  let t5 = sdfu::Triangle::new([
    Vec2::new(-w / 2.0, h2),
    Vec2::new(-w / 2.0, -h2),
    Vec2::new(-w, 0.),
  ]);
  t1.union(t2).union(t3).union(t4).union(t5)
}

pub fn build_level(asset_server: &AssetServer) -> LevelInfo {
  let w = 2000.;
  let segments = 4.0;
  let sh = 10000.;
  let h = segments * sh;

  let fbounds = Vec4::new(0.0, h + (w - 1000.), w, w);
  let angle = f32::atan(3. * (w / 2.) / h);
  let h2 = w / (2.0 * angle.tan());

  let outer = sdfu::Box::new(Vec2::new(w + 3000., h + 3000.));
  let inner = sdfu::Box::new(Vec2::new(w, h));

  let t1 = build_track(w, sh).translate((0.0, sh * 3.0).into());
  let t2 = build_track(w, sh).translate((0.0, sh * 1.0).into());
  let t3 = build_track(w, sh).translate((0.0, sh * -1.0).into());
  let t4 = build_track(w, sh).translate((0.0, sh * -3.0).into());

  let shape = outer.subtract(inner.subtract(t1.union(t2).union(t3).union(t4)));

  let finish_bounds = sdfu::Box::new(Vec2::new(fbounds.z, fbounds.w)).translate(fbounds.xy());
  let terrain_shader = asset_server.load("preload/terrain6.wgsl");
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
    name: "Level 6".to_owned(),
    next_level: None,
    starting_point: Vec2::new(w * 0.75, -h),
    boids_per_spawn_point: 20,
    spawn_points: vec![
      Vec2::new(-w / 2.0, sh),
      Vec2::new(-w / 2.0, -sh),
      ],
    rescue_goal: 20.into(),
    time_goal: Duration::from_secs(120).into(),
    wander: false,
  };
  lvl
}
