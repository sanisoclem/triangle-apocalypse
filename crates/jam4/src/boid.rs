use bevy::prelude::*;

use crate::moveable::{Moveable, MoveableBounds};

#[derive(Component, Default)]
pub struct Boid {
  pub direction: Vec3,
}

pub fn calculate_boid_direction(
  bounds: Res<MoveableBounds>,
  qry: Query<(Entity, &Transform), With<Boid>>,
  mut gizmos: Gizmos,
) {
  for (e1, t1) in qry.iter() {
    let tx = t1.translation;
    let mut forces = Vec3::ZERO;

    // let bounding_force = match bounds {
    //   MoveableBounds::Box(x1,y1 ,x2 ,y2 ) => {
    //     let magnitude = Vec2::new((tx.x - x1).min(x2 - tx.x), (tx.y - y1).min(y2 - tx.y)).length();
    //     Vec3::new(, ,0.0)
    //   }
    //   _ => Vec3::ZERO,
    // };

    // find all neighbors of e1
    for (e2, t2) in qry.iter() {
      let diff = t2.translation - t1.translation;
      if diff.length() > 10.0 {
        continue;
      }
    }
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
    mov.velocity = normalized * 500.0;
  }
}
