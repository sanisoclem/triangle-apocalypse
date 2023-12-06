use bevy::prelude::*;

use crate::moveable::{Moveable, MoveableBounds};

#[derive(Component)]
pub struct Boid {
  pub direction: Vec2,
  pub vision: f32,
  pub personal_space: f32,
  pub is_player: bool,
}
impl Default for Boid {
  fn default() -> Self {
    Boid {
      direction: Vec2::Y,
      vision: 400.0,
      personal_space: 50.0,
      is_player: false,
    }
  }
}

#[derive(Resource)]
pub struct BoidConfig {
  pub boid_speed: f32,
  pub player_boid_speed: f32,
  pub boundary: f32,
  pub cohesion: f32,
  pub alignment: f32,
  pub repulsion: f32,
  pub lprobe: Mat2,
  pub rprobe: Mat2,
  pub lforce: Mat2,
  pub rforce: Mat2,
  pub show_forces: bool,
  pub show_direction: bool,
  pub show_personal_space: bool,
  pub show_vision: bool,
  pub show_bounds: bool,
  pub turn_rate: f32,
  pub player_influence: f32,
}

impl Default for BoidConfig {
  fn default() -> Self {
    BoidConfig {
      boid_speed: 500.,
      player_influence: 10.,
      player_boid_speed: 600.,
      boundary: 5.0,
      cohesion: 1.0,
      alignment: 1.0,
      repulsion: 10.0,
      lprobe: Mat2::from_angle(45.0f32.to_radians()),
      rprobe: Mat2::from_angle(-45.0f32.to_radians()),
      lforce: Mat2::from_angle(90.0f32.to_radians()),
      rforce: Mat2::from_angle(-90.0f32.to_radians()),
      show_forces: false,
      show_direction: false,
      show_personal_space: false,
      show_vision: false,
      show_bounds: true,
      turn_rate: 10.,
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
      if boid.is_player {
        return (e, Vec2::ZERO, t1.translation);
      }

      // TODO: extract/BAP
      let tx = t1.translation.xy();
      let v = boid.direction.xy() * boid.vision;
      let tx2 = tx + v;
      let mut bounds_force = Vec2::ZERO;
      let mut separation_force = Vec2::ZERO;
      let mut cohesion_force = Vec2::ZERO;
      let mut alignment_force = Vec2::ZERO;

      let rayl = tx + bconfig.lprobe.mul_vec2(v);
      let rayr = tx + bconfig.rprobe.mul_vec2(v);

      let colf = bounds.distance_to_edge(tx2);
      let coll = bounds.distance_to_edge(rayl);
      let colr = bounds.distance_to_edge(rayr);
      if coll < 0.0 && coll < colr {
        bounds_force += bconfig.rforce.mul_vec2(boid.direction);
      } else if colr < 0.0 {
        bounds_force += bconfig.lforce.mul_vec2(boid.direction);
      } else if colf < 0.0 {
        bounds_force += bounds.edge_normal(tx2)
      }

      // find all neighbors of e1
      for (_, t_other, boid2) in qry.iter() {
        let tx_other = t_other.translation.xy();
        let factor = if boid2.is_player {
          bconfig.player_influence
        } else {
          1.0
        };
        let diff = tx_other - tx;
        let dist = diff.length();
        let maxpspace = boid.personal_space.max(boid2.personal_space);
        let mag_pspace = dist / maxpspace;
        let mag_vision = dist / boid.vision;

        if dist < boid.personal_space || dist < boid2.personal_space {
          separation_force += -diff * (1.0 - mag_pspace) * factor;
        } else if dist < boid.vision {
          // TODO: customize falloff curve
          cohesion_force += diff;
          alignment_force += boid2.direction * (1.0 - mag_vision) * factor;
        } else {
          continue;
        }
      }

      let force = ((bounds_force.normalize_or_zero() * bconfig.boundary)
        + (separation_force.normalize_or_zero() * bconfig.repulsion)
        + (alignment_force.normalize_or_zero() * bconfig.alignment)
        + (cohesion_force.normalize_or_zero() * bconfig.cohesion))
        .normalize_or_zero();

      if bconfig.show_forces {
        gizmos.ray_2d(tx, force.xy() * boid.vision, Color::CYAN);
      }

      (e, force, t1.translation)
    })
    .collect::<Vec<_>>();
  for (e, f, t) in changes.iter() {
    let (_, _, mut b) = qry.get_mut(*e).unwrap();
    b.direction = (b.direction + (*f * time.delta_seconds() * bconfig.turn_rate)).normalize();

    if bconfig.show_direction {
      gizmos.ray_2d(t.xy(), b.direction * 100.0, Color::BLUE);
    }
  }
}

pub fn update_boid_velocity(
  mut qry: Query<(&mut Moveable, &mut Transform, &Boid)>,
  bconfig: Res<BoidConfig>,
) {
  for (mut mov, mut t, boid) in qry.iter_mut() {
    let normalized = if boid.direction == Vec2::ZERO {
      Vec3::Y
    } else {
      boid.direction.extend(0.0).normalize()
    };

    t.rotation =
      Quat::from_rotation_z(normalized.x.signum() * -1. * normalized.angle_between(Vec3::Y));

    if boid.is_player {
      mov.velocity = normalized * bconfig.player_boid_speed;
    } else {
      mov.velocity = normalized * bconfig.boid_speed;
    }
  }
}
