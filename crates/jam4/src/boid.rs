use bevy::prelude::*;

use crate::moveable::{Moveable, MoveableBounds};

#[derive(Component)]
pub struct Boid {
  pub direction: Vec3,
  pub direction2: Vec3,
}
impl Default for Boid {
  fn default() -> Self {
    Boid {
      direction: Vec3::X,
      direction2: Vec3::X,
    }
  }
}

pub fn calculate_boid_direction(
  bounds: Res<MoveableBounds>,
  mut qry: Query<(Entity, &Transform, &mut Boid)>,
  time: Res<Time>,
  mut gizmos: Gizmos,
) {
  let ray_length = 100.0;
  let lrot = Quat::from_rotation_z(45.0f32.to_radians());
  let rrot = Quat::from_rotation_z(-45.0f32.to_radians());
  let lrot90 = Quat::from_rotation_z(90.0f32.to_radians());
  let rrot90 = Quat::from_rotation_z(-90.0f32.to_radians());
  let changes = qry
    .iter()
    .map(|(e, t1, boid)| {
      let tx = t1.translation;
      let mut forces = Vec3::ZERO;
      let forcel = lrot90.mul_vec3(boid.direction);
      let forcer = rrot90.mul_vec3(boid.direction);
      let rayf = tx + (boid.direction * ray_length);
      let rayl = tx + lrot.mul_vec3(boid.direction * ray_length);
      let rayr = tx + rrot.mul_vec3(boid.direction * ray_length);

      gizmos.ray_2d(
        tx.xy(),
        rrot.mul_vec3(boid.direction).xy() * ray_length,
        Color::BLUE,
      );
      gizmos.ray_2d(
        tx.xy(),
        lrot.mul_vec3(boid.direction).xy() * ray_length,
        Color::RED,
      );

      let colf = bounds.distance_to_edge(rayf.xy());
      let coll = bounds.distance_to_edge(rayl.xy());
      let colr = bounds.distance_to_edge(rayr.xy());
      if coll > 0.0 && coll > colr {
        forces += forcer;
        gizmos.ray_2d(
          tx.xy(),
          forcel.xy() * 500.0,
          Color::RED,
        );
      } else if colr > 0.0 || colf > 0.0 {
        forces += forcel;
        gizmos.ray_2d(
          tx.xy(),
          forcer.xy() * 500.0,
          Color::BLUE,
        );
      }

      // let to_edge = bounds.get_closest_point(tx.xy()).extend(0.0) - tx;
      // let edge_mag = (200.0 - to_edge.length()).clamp(0.0, 200.0) / 200.0;

      // let edge_force = if to_edge == Vec3::ZERO {
      //   Vec3::ZERO
      // } else {
      //   to_edge.any_orthonormal_vector() * edge_mag
      // };

      // find all neighbors of e1
      for (e2, t2, _) in qry.iter() {
        let diff = t2.translation - t1.translation;
        if diff.length() > 10.0 {
          continue;
        }
      }
      (e, forces.normalize_or_zero(), tx)
    })
    .collect::<Vec<_>>();
  for (e, f, t) in changes.iter() {
    let (_, _, mut b) = qry.get_mut(*e).unwrap();
    b.direction = b.direction + (*f * time.delta_seconds() * 3.0);
    gizmos.ray_2d(t.xy(), b.direction.xy() * 200.0, Color::PINK);
  }
}

pub fn update_boid_velocity(mut qry: Query<(&mut Moveable, &Boid)>) {
  for (mut mov, boid) in qry.iter_mut() {
    let normalized = if boid.direction == Vec3::ZERO {
      Vec3::X
    } else {
      boid.direction.normalize()
    };

    // TODO: move constant velocity to boid
    mov.velocity = normalized * 250.0;
  }
}
