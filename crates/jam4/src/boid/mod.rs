use crate::{
  moveable::{CollidedWithBounds, Moveable, MoveableBounds},
  Player,
};
use bevy::prelude::*;

mod components;
mod config;

pub use components::*;
pub use config::*;

pub fn despawn_collided_boids(
  mut cmd: Commands,
  qry: Query<Entity, (With<Boid>, Added<CollidedWithBounds>)>,
) {
  for e in qry.iter() {
    cmd.entity(e).despawn_recursive();
  }
}

pub fn update_boid_color(
  mut cmd: Commands,
  mut qry: Query<(Entity, &Transform, &mut Handle<ColorMaterial>), (With<Boid>, Without<Player>)>,
  qry_player: Query<(&Transform, &Player)>,
  mut gizmos: Gizmos,
  bconfig: Res<BoidConfig>,
) {
  let Ok((p_trans, p)) = qry_player.get_single() else {
    return;
  };

  gizmos.circle_2d(p_trans.translation.xy(), p.influence_radius, Color::RED);

  for (e, transform, mut boid_color) in qry.iter_mut() {
    if transform.translation.distance_squared(p_trans.translation)
      <= p.influence_radius * p.influence_radius
    {
      cmd.entity(e).insert(TamedBoid);
      *boid_color = bconfig.color_tamed.clone();
    } else {
      cmd.entity(e).remove::<TamedBoid>();
      *boid_color = bconfig.color_wild.clone();
    }
  }
}

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
