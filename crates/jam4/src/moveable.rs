use std::{default, sync::Arc};

use bevy::prelude::*;
use sdfu::{
  estimate_normals,
  util::{CentralDifferenceEstimator, EstimateNormal},
  Dim2D, SDF,
};

#[derive(Component, Default)]
pub struct Moveable {
  pub velocity: Vec3,
}

pub struct SdfBounds {
  sdf: Arc<dyn SDF<f32, Vec2> + Send + Sync>,
  normals: EstimateNormal<
    f32,
    Vec2,
    Arc<(dyn SDF<f32, Vec2> + Send + Sync)>,
    CentralDifferenceEstimator<f32, Vec2, Dim2D>,
  >,
}

#[derive(Resource, Default)]
pub enum MoveableBounds {
  #[default]
  None,
  Sdf(SdfBounds),
}

impl MoveableBounds {
  pub fn from_sdf<S: SDF<f32, Vec2> + Send + Sync + 'static>(sdf: S) -> Self {
    let a: Arc<(dyn SDF<f32, Vec2> + Send + Sync)> = Arc::new(sdf);
    Self::Sdf(SdfBounds {
      sdf: a.clone(),
      normals: estimate_normals(a, 0.001f32),
    })
  }
  pub fn distance_to_edge(&self, p: Vec2) -> f32 {
    match self {
      MoveableBounds::None => f32::NEG_INFINITY,
      MoveableBounds::Sdf(sdf) => sdf.sdf.dist(p),
    }
  }

  pub fn bounce(&self, o: Vec2, p: Vec2) -> (Vec2, Vec2) {
    let op = o + p;
    match self {
      MoveableBounds::None => (op, p),
      MoveableBounds::Sdf(sdf) => {
        let d = self.distance_to_edge(op);
        if d > 0.0 {
          (op, p)
        } else {
          let incident = p.normalize();
          let normal = sdf.normals.normal_at(op);
          let angle = (90.0f32.to_radians() - normal.angle_between(incident)) * 2.0;
          let newp = Quat::from_rotation_z(angle).mul_vec3(p.extend(0.)).xy();
          if sdf.sdf.dist(o + newp) < 0.{
            return (o, p);
          }
          (o + newp, newp)
        }
      }
    }
  }
}

pub fn move_moveables(
  bounds: Res<MoveableBounds>,
  mut qry: Query<(&mut Transform, &mut Moveable)>,
  time: Res<Time>,
  mut gizmos: Gizmos,
) {
  for (mut trn, mut mov) in qry.iter_mut() {
    // calculate next position
    let travel = mov.velocity * time.delta_seconds();
    let (new_translation, new_v) = bounds.bounce(trn.translation.xy(), travel.xy());

    // let dist = bounds.distance_to_edge(trn.translation.xy());
    // if dist > 0. {
    //   gizmos.circle_2d(trn.translation.xy(), dist, Color::RED);
    // } else {
    //   gizmos.circle_2d(trn.translation.xy(), dist, Color::BLUE);
    // }

    //gizmos.ray_2d(trn.translation.xy(), mov.velocity.xy() * 100., Color::RED);

    // update position
    *trn = trn.with_translation(new_translation.extend(0.0));
    mov.velocity = mov.velocity.length() * new_v.extend(0.0).normalize();
  }
}
