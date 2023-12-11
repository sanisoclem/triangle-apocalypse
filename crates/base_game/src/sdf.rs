use std::{marker::PhantomData, time::Duration};

use bevy::prelude::*;
use bevy_smud::prelude::*;
use jam4::{level::LevelInfo, moveable::MoveableBounds};
use sdfu::{mods::Translate, ops::HardMin, Dim2D, Triangle, SDF};

pub fn build_track(
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

pub fn build_maze(
  p: f32,
) -> sdfu::ops::Union<
  f32,
  sdfu::ops::Union<
    f32,
    sdfu::ops::Union<
      f32,
      sdfu::ops::Union<
        f32,
        sdfu::ops::Union<
          f32,
          sdfu::ops::Union<
            f32,
            sdfu::ops::Union<
              f32,
              sdfu::ops::Union<
                f32,
                sdfu::ops::Union<
                  f32,
                  sdfu::ops::Union<
                    f32,
                    sdfu::ops::Union<
                      f32,
                      sdfu::ops::Union<
                        f32,
                        sdfu::ops::Union<
                          f32,
                          sdfu::ops::Union<
                            f32,
                            sdfu::ops::Union<
                              f32,
                              sdfu::ops::Union<
                                f32,
                                sdfu::ops::Union<
                                  f32,
                                  Translate<
                                    bevy::prelude::Vec2,
                                    sdfu::Box<bevy::prelude::Vec2, Dim2D>,
                                  >,
                                  Translate<
                                    bevy::prelude::Vec2,
                                    sdfu::Box<bevy::prelude::Vec2, Dim2D>,
                                  >,
                                  HardMin<f32>,
                                >,
                                Translate<
                                  bevy::prelude::Vec2,
                                  sdfu::Box<bevy::prelude::Vec2, Dim2D>,
                                >,
                                HardMin<f32>,
                              >,
                              Translate<bevy::prelude::Vec2, sdfu::Box<bevy::prelude::Vec2, Dim2D>>,
                              HardMin<f32>,
                            >,
                            Translate<bevy::prelude::Vec2, sdfu::Box<bevy::prelude::Vec2, Dim2D>>,
                            HardMin<f32>,
                          >,
                          Translate<bevy::prelude::Vec2, sdfu::Box<bevy::prelude::Vec2, Dim2D>>,
                          HardMin<f32>,
                        >,
                        Translate<bevy::prelude::Vec2, sdfu::Box<bevy::prelude::Vec2, Dim2D>>,
                        HardMin<f32>,
                      >,
                      Translate<bevy::prelude::Vec2, sdfu::Box<bevy::prelude::Vec2, Dim2D>>,
                      HardMin<f32>,
                    >,
                    Translate<bevy::prelude::Vec2, sdfu::Box<bevy::prelude::Vec2, Dim2D>>,
                    HardMin<f32>,
                  >,
                  Translate<bevy::prelude::Vec2, sdfu::Box<bevy::prelude::Vec2, Dim2D>>,
                  HardMin<f32>,
                >,
                Translate<bevy::prelude::Vec2, sdfu::Box<bevy::prelude::Vec2, Dim2D>>,
                HardMin<f32>,
              >,
              Translate<bevy::prelude::Vec2, sdfu::Box<bevy::prelude::Vec2, Dim2D>>,
              HardMin<f32>,
            >,
            Translate<bevy::prelude::Vec2, sdfu::Box<bevy::prelude::Vec2, Dim2D>>,
            HardMin<f32>,
          >,
          Translate<bevy::prelude::Vec2, sdfu::Box<bevy::prelude::Vec2, Dim2D>>,
          HardMin<f32>,
        >,
        Translate<bevy::prelude::Vec2, sdfu::Box<bevy::prelude::Vec2, Dim2D>>,
        HardMin<f32>,
      >,
      Translate<bevy::prelude::Vec2, sdfu::Box<bevy::prelude::Vec2, Dim2D>>,
      HardMin<f32>,
    >,
    Translate<bevy::prelude::Vec2, sdfu::Box<bevy::prelude::Vec2, Dim2D>>,
    HardMin<f32>,
  >,
  Translate<bevy::prelude::Vec2, sdfu::Box<bevy::prelude::Vec2, Dim2D>>,
  HardMin<f32>,
> {
  let h = p * 20.;
  let w = p * 20.;

  let b1 = sdfu::Box::new((p * 6.0, p * 2.0).into()).translate(Vec2::new(p * 4.0, p * -14.0));
  let b2 = sdfu::Box::new(Vec2::new(p * 1.0, p * 3.0)).translate(Vec2::new(p * 11.0, p * -5.0));
  let b3 = sdfu::Box::new(Vec2::new(p * 2.0, p * 1.0)).translate(Vec2::new(p * 15.0, p * -3.0));
  let b4 = sdfu::Box::new(Vec2::new(p * 1.0, p * 3.0)).translate(Vec2::new(p * 19.0, p * -5.0));

  let b5 = sdfu::Box::new(Vec2::new(p * 2.0, p * 9.0)).translate(Vec2::new(p * -8.0, p * -11.0));
  let b6 = sdfu::Box::new(Vec2::new(p * 6.0, p * 3.0)).translate(Vec2::new(p * 0.0, p * -5.0));
  let b7 = sdfu::Box::new(Vec2::new(p * 1.0, p * 7.0)).translate(Vec2::new(p * -15.0, p * -9.0));

  let b8 = sdfu::Box::new(Vec2::new(p * 2.0, p * 2.0)).translate(Vec2::new(p * -18.0, p * 12.0));
  let b9 = sdfu::Box::new(Vec2::new(p * 1.0, p * 2.0)).translate(Vec2::new(p * -19.0, p * 4.0));
  let b10 = sdfu::Box::new(Vec2::new(p * 2.0, p * 2.0)).translate(Vec2::new(p * -12.0, p * 4.0));
  let b11 = sdfu::Box::new(Vec2::new(p * 1.0, p * 4.0)).translate(Vec2::new(p * -11.0, p * 8.0));
  let b12 = sdfu::Box::new(Vec2::new(p * 7.0, p * 1.0)).translate(Vec2::new(p * -3.0, p * 13.0));
  let b13 = sdfu::Box::new(Vec2::new(p * 1.0, p * 2.0)).translate(Vec2::new(p * 3.0, p * 10.0));

  let b14 = sdfu::Box::new(Vec2::new(p * 2.0, p * 3.0)).translate(Vec2::new(p * -4.0, p * 5.0));
  let b15 = sdfu::Box::new(Vec2::new(p * 6.0, p * 1.0)).translate(Vec2::new(p * 4.0, p * 3.0));
  let b16 = sdfu::Box::new(Vec2::new(p * 1.0, p * 5.0)).translate(Vec2::new(p * 9.0, p * 7.0));
  let b17 = sdfu::Box::new(Vec2::new(p * 3.0, p * 2.0)).translate(Vec2::new(p * 13.0, p * 12.0));
  let b18 = sdfu::Box::new(Vec2::new(p * 1.0, p * 2.0)).translate(Vec2::new(p * 15.0, p * 4.0));

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
    .union(b18)
}

#[derive(Clone, Copy, Debug)]
pub struct Flip<V, S> {
  pub sdf: S,
  pub p: PhantomData<V>,
}

impl<V,S> Flip<V, S> {
  pub fn new(sdf: S) -> Self {
    Flip {
      sdf,
      p: PhantomData,
    }
  }
}

impl<V, S> SDF<f32, V> for Flip<V, S>
where
  V: sdfu::mathtypes::Vec<f32>,
  S: SDF<f32, V>,
{
  #[inline]
  fn dist(&self, p: V) -> f32 {
    self.sdf.dist(p * -1.)
  }
}
