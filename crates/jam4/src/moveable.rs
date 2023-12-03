use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Moveable {
  pub velocity: Vec3,
}

#[derive(Default, Resource)]
pub enum MoveableBounds {
  #[default]
  None,
  Box(Vec2),
  Sdf,
}

impl MoveableBounds {
  pub fn distance_to_edge(&self, p: Vec2) -> f32 {
    match self {
      MoveableBounds::Box(e) => {
        let d  =  p.abs() - *e;
        d.max(Vec2::ZERO).length() + d.x.max(d.y).min(0.0)
      },
      _ => unimplemented!(),
    }
  }
  pub fn get_closest_point(&self, p: Vec2) -> Vec2 {
    match self {
      MoveableBounds::Box(e) => {
        // let diff = (e.x - e.y);
        // let d2 = Vec2::new()
        if p.x.abs() > p.y.abs() {
          Vec2::new(e.x * p.x.signum(), p.y.clamp(e.y * -1.0, e.y))
        } else {
          Vec2::new(p.x.clamp(e.x * -1.0, e.x), e.y * p.y.signum())
        }
      }
      _ => unimplemented!(),
    }
  }
  pub fn bounce(&self, o: Vec2, p: Vec2) -> (Vec2, Vec2) {
    match self {
      MoveableBounds::Box(e) => {
        let mut po = o + p;
        let mut v = p;
        if po.x.abs() > e.x {
          po.x -= (po.x - (e.x * po.x.signum())) * 2.0;
          v.x *= -1.0;
        }
        if po.y.abs() > e.y {
          po.y -= (po.y - (e.y * po.y.signum())) * 2.0;
          v.y *= -1.0;
        }
        (po, v)
      }
      _ => unimplemented!(),
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

    let tae = travel / time.delta_seconds();
    gizmos.ray_2d(trn.translation.xy(), tae.xy(), Color::GREEN);
    // update position
    *trn = trn.with_translation(new_translation.extend(0.0));
    mov.velocity = mov.velocity.length() * new_v.extend(0.0).normalize();
  }
}
