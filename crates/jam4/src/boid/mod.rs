use crate::moveable::{Moveable, MoveableBounds};
use bevy::prelude::*;

mod components;
mod config;

pub use components::*;
pub use config::*;

// pub fn spawn_boids(mut cmd: Commands, mut spawner: )

pub fn draw_boid_gizmos(
  qry: Query<(&Transform, &Boid)>,
  mut gizmos: Gizmos,
  bconfig: Res<BoidConfig>,
) {
  for (t, b) in qry.iter() {
    let pos = t.translation.xy();
    if bconfig.show_vision {
      gizmos.circle_2d(pos, b.vision, Color::PURPLE);
    }
    if bconfig.show_personal_space {
      gizmos.circle_2d(pos, b.personal_space, Color::RED);
    }
    if bconfig.show_direction {
      gizmos.ray_2d(pos, b.direction.xy() * b.vision, Color::GREEN);
    }
  }
}

pub fn calculate_boid_direction(
  mut qry: Query<(Entity, &Transform, &mut Boid)>,
  mut gizmos: Gizmos,
  bounds: Res<MoveableBounds>,
  bconfig: Res<BoidConfig>,
  time: Res<Time>,
) {
  let changes = qry
    .iter()
    .map(|(e, t1, boid)| {
      let pos = t1.translation.xy();
      let force = boid.calculate_forces(&qry, &bconfig, pos, &bounds);

      if bconfig.show_forces {
        gizmos.ray_2d(pos, force * boid.vision, Color::CYAN);
      }

      (e, force, pos)
    })
    .collect::<Vec<_>>();

  for (e, f, pos) in changes.iter() {
    let (_, _, mut b) = qry.get_mut(*e).unwrap();
    b.direction = (b.direction + (*f * time.delta_seconds() * b.turning_speed)).normalize();

    if bconfig.show_direction {
      gizmos.ray_2d(*pos, b.direction * 100.0, Color::BLUE);
    }
  }
}

pub fn update_boid_velocity(mut qry: Query<(&mut Moveable, &mut Transform, &Boid)>) {
  for (mut mov, mut t, boid) in qry.iter_mut() {
    let normalized = if boid.direction == Vec2::ZERO {
      Vec3::Y
    } else {
      boid.direction.extend(0.0).normalize()
    };

    t.rotation =
      Quat::from_rotation_z(normalized.x.signum() * -1. * normalized.angle_between(Vec3::Y));

    mov.velocity = normalized * boid.speed;
  }
}
