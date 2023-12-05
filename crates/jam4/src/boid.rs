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
      vision: 100.0,
      personal_space: 10.0,
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
  pub lprobe: Quat,
  pub rprobe: Quat,
  pub lforce: Quat,
  pub rforce: Quat,
  pub show_forces: bool,
  pub show_direction: bool,
  pub show_personal_space: bool,
  pub show_vision: bool,
  pub show_bounds: bool,
}

impl Default for BoidConfig {
  fn default() -> Self {
    BoidConfig {
      boid_speed: 250.,
      player_boid_speed: 300.,
      boundary: 100.0,
      cohesion: 0.001,
      alignment: 0.5,
      repulsion: 100.0,
      lprobe: Quat::from_rotation_z(45.0f32.to_radians()),
      rprobe: Quat::from_rotation_z(-45.0f32.to_radians()),
      lforce: Quat::from_rotation_z(90.0f32.to_radians()),
      rforce: Quat::from_rotation_z(-90.0f32.to_radians()),
      show_forces: false,
      show_direction: false,
      show_personal_space: false,
      show_vision: false,
      show_bounds: true,
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
        return (e, Vec3::ZERO, t1.translation);
      }

      let tx = t1.translation;
      let mut bounds_force = Vec3::ZERO;
      let mut separation_force = Vec3::ZERO;
      let mut cohesion_force = Vec3::ZERO;
      let mut alignment_force = Vec3::ZERO;

      let rayl: Vec3 = tx + bconfig.lprobe.mul_vec3(boid.direction * boid.vision);
      let rayr = tx + bconfig.rprobe.mul_vec3(boid.direction * boid.vision);

      let colf = bounds.distance_to_edge((boid.direction * boid.vision).xy());
      let coll = bounds.distance_to_edge(rayl.xy());
      let colr = bounds.distance_to_edge(rayr.xy());
      if coll < 0.0 && coll < colr {
        let forcer = bconfig.rforce.mul_vec3(boid.direction);
        bounds_force += forcer;
      } else if colr < 0.0 || colf < 0.0 {
        let forcel = bconfig.lforce.mul_vec3(boid.direction);
        bounds_force += forcel;
      }

      // find all neighbors of e1
      for (_, t2, boid2) in qry.iter() {
        let neg_diff = tx - t2.translation;
        let diff = t2.translation - tx;
        let ndl = neg_diff.length();
        if ndl < boid.personal_space {
          let m2 = (boid.personal_space - ndl) / boid.personal_space;
          separation_force += neg_diff * m2;
        } else if ndl < boid.vision {
          let m3 = (boid.vision - ndl) / boid.vision;
          cohesion_force += diff * m3;
          alignment_force += boid2.direction * m3;
        } else {
          continue;
        }
      }

      let force = ((bounds_force * bconfig.boundary)
        + (separation_force * bconfig.repulsion)
        + (alignment_force * bconfig.alignment)
        + (cohesion_force * bconfig.cohesion))
        .normalize_or_zero();

      if bconfig.show_forces {
        gizmos.ray_2d(tx.xy(), force.xy() * boid.vision, Color::CYAN);
      }

      (e, force, tx)
    })
    .collect::<Vec<_>>();
  for (e, f, t) in changes.iter() {
    let (_, _, mut b) = qry.get_mut(*e).unwrap();
    b.direction = (b.direction + (*f * time.delta_seconds() * 3.0)).normalize();

    if bconfig.show_direction {
      gizmos.ray_2d(t.xy(), b.direction.xy() * 100.0, Color::BLUE);
    }
  }
}

pub fn update_boid_velocity(
  mut qry: Query<(&mut Moveable, &mut Transform, &Boid)>,
  bconfig: Res<BoidConfig>,
) {
  for (mut mov, mut t, boid) in qry.iter_mut() {
    let normalized = if boid.direction == Vec3::ZERO {
      Vec3::Y
    } else {
      boid.direction.normalize()
    };

    t.rotation = Quat::from_rotation_z(
      boid.direction.x.signum() * -1. * boid.direction.angle_between(Vec3::Y),
    );

    if boid.is_player {
      mov.velocity = normalized * bconfig.player_boid_speed;
    } else {
      mov.velocity = normalized * bconfig.boid_speed;
    }
  }
}
