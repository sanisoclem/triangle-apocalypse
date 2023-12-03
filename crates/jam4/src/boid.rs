use bevy::prelude::*;

use crate::moveable::{Moveable, MoveableBounds};

#[derive(Component)]
pub struct Boid {
  pub direction: Vec3,
  pub vision: f32,
  pub personal_space: f32,
  pub is_player: bool,
}
impl Default for Boid {
  fn default() -> Self {
    Boid {
      direction: Vec3::Y,
      vision: 70.0,
      personal_space: 30.0,
      is_player: false,
    }
  }
}

pub fn calculate_boid_direction(
  bounds: Res<MoveableBounds>,
  mut qry: Query<(Entity, &Transform, &mut Boid)>,
  time: Res<Time>,
  mut gizmos: Gizmos,
) {
  let lrot = Quat::from_rotation_z(45.0f32.to_radians());
  let rrot = Quat::from_rotation_z(-45.0f32.to_radians());
  let lrot90 = Quat::from_rotation_z(90.0f32.to_radians());
  let rrot90 = Quat::from_rotation_z(-90.0f32.to_radians());
  let changes = qry
    .iter()
    .map(|(e, t1, boid)| {
      if (boid.is_player) {
        return (e, Vec3::ZERO, t1.translation);
      }

      let tx = t1.translation;
      let mut bounds_force = Vec3::ZERO;
      let mut separation_force = Vec3::ZERO;
      let mut cohesion_force = Vec3::ZERO;
      let mut alignment_force = Vec3::ZERO;
      let forcel = lrot90.mul_vec3(boid.direction);
      let forcer = rrot90.mul_vec3(boid.direction);
      let rayf = tx + (boid.direction * boid.vision);
      let rayl = tx + lrot.mul_vec3(boid.direction * boid.vision);
      let rayr = tx + rrot.mul_vec3(boid.direction * boid.vision);

      // gizmos.ray_2d(
      //   tx.xy(),
      //   rrot.mul_vec3(boid.direction).xy() * boid.vision,
      //   Color::BLUE,
      // );
      // gizmos.ray_2d(
      //   tx.xy(),
      //   lrot.mul_vec3(boid.direction).xy() * boid.vision,
      //   Color::RED,
      // );

      let colf = bounds.distance_to_edge(rayf.xy());
      let coll = bounds.distance_to_edge(rayl.xy());
      let colr = bounds.distance_to_edge(rayr.xy());
      if coll > 0.0 && coll > colr {
        bounds_force += forcer;
        //gizmos.ray_2d(tx.xy(), forcel.xy() * 500.0, Color::RED);
      } else if colr > 0.0 || colf > 0.0 {
        bounds_force += forcel;
        // gizmos.ray_2d(tx.xy(), forcer.xy() * 500.0, Color::BLUE);
      }

      // find all neighbors of e1
      for (e2, t2, boid2) in qry.iter() {
        let neg_diff = tx - t2.translation;
        let diff = t2.translation - tx;
        let ndl = neg_diff.length();
        if ndl < boid.personal_space {
          let m2 = (boid.personal_space - ndl) / boid.personal_space;
          separation_force += neg_diff * m2;
          // gizmos.line_2d(tx.xy(), t2.translation.xy(), Color::RED);
        } else if ndl < boid.vision {
          let m3 = (boid.vision - ndl) / boid.vision;
          cohesion_force += diff * m3;
          alignment_force += boid2.direction * m3;
          // cohesion_force += diff;
          // gizmos.line_2d(tx.xy(), t2.translation.xy(), Color::CYAN);
        } else {
          continue;
        }
      }

      let force = (bounds_force * 100.)
        + (separation_force * 10.)
        + (alignment_force * 10.)
        + (cohesion_force * 1.);
      (e, force.normalize_or_zero(), tx)
    })
    .collect::<Vec<_>>();
  for (e, f, t) in changes.iter() {
    let (_, _, mut b) = qry.get_mut(*e).unwrap();
    b.direction = (b.direction + (*f * time.delta_seconds() * 3.0)).normalize();
    gizmos.ray_2d(t.xy(), b.direction.xy() * 100.0, Color::PINK);
  }
}

pub fn update_boid_velocity(mut qry: Query<(&mut Moveable, &mut Transform, &Boid)>) {
  for (mut mov, mut t, boid) in qry.iter_mut() {
    let normalized = if boid.direction == Vec3::ZERO {
      Vec3::Y
    } else {
      boid.direction.normalize()
    };

    t.rotation = Quat::from_rotation_z(
      boid.direction.x.signum() * -1. * boid.direction.angle_between(Vec3::Y),
    );

    // TODO: move constant velocity to boid
    mov.velocity = normalized * 250.0;
  }
}
