use bevy::prelude::*;

use crate::moveable::MoveableBounds;

use super::BoidConfig;

#[derive(Component)]
pub struct TamedBoid;

#[derive(Component)]
pub struct Boid {
  pub direction: Vec2,
  pub vision: f32,
  pub personal_space: f32,
  pub is_player: bool,
  pub speed: f32,
  pub turning_speed: f32,
}

impl Default for Boid {
  fn default() -> Self {
    Boid {
      direction: Vec2::Y,
      vision: 400.0,
      personal_space: 50.0,
      is_player: false,
      speed: 500.,
      turning_speed: 10.0,
    }
  }
}

impl Boid {
  pub fn calculate_bounds_force(
    &self,
    bconfig: &BoidConfig,
    position: Vec2,
    bounds: &MoveableBounds,
  ) -> Vec2 {
    let v = self.direction * self.vision;
    let tx2 = position + v;

    let rayl = position + bconfig.lprobe.mul_vec2(v);
    let rayr = position + bconfig.rprobe.mul_vec2(v);

    let colf = bounds.distance_to_edge(tx2);
    let coll = bounds.distance_to_edge(rayl);
    let colr = bounds.distance_to_edge(rayr);

    if coll < 0.0 && coll < colr {
      return bconfig.rforce.mul_vec2(self.direction);
    } else if colr < 0.0 {
      return bconfig.lforce.mul_vec2(self.direction);
    } else if colf < 0.0 {
      return bounds.edge_normal(tx2);
    }

    Vec2::ZERO
  }

  // TODO: clean up this mess
  pub fn calculate_forces(
    &self,
    qry: &Query<(Entity, &Transform, &mut Boid, Option<&TamedBoid>)>,
    bconfig: &BoidConfig,
    position2d: Vec2,
    bounds: &MoveableBounds,
    is_tamed: bool,
    _gizmos: &mut Gizmos,
  ) -> (Vec2, f32) {
    // don't calculate forces for player boid
    if self.is_player {
      return (Vec2::ZERO, self.speed);
    }

    let bounds_force = self.calculate_bounds_force(bconfig, position2d, bounds);
    let mut separation_force = Vec2::ZERO;
    let mut cohesion_force = Vec2::ZERO;
    let mut alignment_force = Vec2::ZERO;
    let max_dest = position2d + (self.direction * bconfig.max_speed);
    let min_dest = position2d + (self.direction * bconfig.min_speed);
    let mut speed_change = 0.0;

    // gizmos.line_2d(position2d, min_dest, Color::BLUE);
    // gizmos.line_2d(position2d, max_dest, Color::RED);

    // TODO: optimize
    for (_, t_other, boid_other, _) in qry.iter() {
      let factor = if boid_other.is_player {
        bconfig.player_influence
      } else {
        1.0
      };
      let position2d_other = t_other.translation.xy();
      let diff = position2d_other - position2d;
      let dist = diff.length();
      let maxpspace = self.personal_space.max(boid_other.personal_space);
      let mag_pspace = dist / maxpspace;
      let mag_vision = dist / self.vision;

      if boid_other.is_player {
        let player_dest = position2d_other + (boid_other.direction * boid_other.speed);

        // gizmos.line_2d(min_dest, player_dest, Color::BLUE);
        // gizmos.line_2d(max_dest, player_dest, Color::RED);
        let max_dist = max_dest.distance(player_dest);
        let min_dist = min_dest.distance(player_dest);
        if max_dist > min_dist && max_dist > boid_other.personal_space + self.personal_space {
          speed_change = bconfig.min_speed;
        } else if min_dist > max_dist && min_dist > boid_other.personal_space + self.personal_space
        {
          speed_change = bconfig.max_speed;
        }
      }

      if dist < self.personal_space.max(boid_other.personal_space) {
        separation_force += -diff * (1.0 - mag_pspace);
      } else if dist < self.vision.max(boid_other.vision) {
        // TODO: customize falloff curve?
        cohesion_force += diff * factor;
        alignment_force += boid_other.direction
          * if boid_other.is_player {
            1.0
          } else {
            1.0 - mag_vision
          }
          * factor;
      } else {
        continue;
      }
    }

    (
      ((bounds_force.normalize_or_zero() * bconfig.boundary)
        + (separation_force.normalize_or_zero() * bconfig.repulsion)
        + (alignment_force.normalize_or_zero()
          * if is_tamed || bconfig.wander {
            bconfig.alignment
          } else {
            0.0
          })
        + (cohesion_force.normalize_or_zero() * bconfig.cohesion))
        .normalize_or_zero(),
      speed_change,
    )
  }
}
