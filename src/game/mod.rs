use bevy::prelude::*;
use camera::*;
use music::*;

use jam4::{boid::Boid, Player, SimulationState};
mod camera;
mod music;

#[derive(Resource)]
struct GameNextState<T>(T);
pub trait GameExtensions {
  fn add_game<T: States + Copy>(&mut self, game_state: T) -> &mut Self;
}

impl GameExtensions for App {
  fn add_game<T: States + Copy>(&mut self, game_state: T) -> &mut Self {
    self
      .add_systems(OnEnter(game_state), (setup_music, setup_camera))
      .add_systems(
        Update,
        (calc_player_direction, follow_player).run_if(in_state(SimulationState::Simulating)),
      )
    //     .add_player_camera()
    //     .add_music()
    //     .add_player_ui()
  }
}

pub fn calc_player_direction(
  mut qry: Query<&mut Boid, With<Player>>,
  keyboard_input: Res<Input<KeyCode>>,
  time: Res<Time>,
) {
  if let Ok(mut p) = qry.get_single_mut() {
    let mut turning_force = Vec3::ZERO;

    if keyboard_input.pressed(KeyCode::A) {
      turning_force += Quat::from_rotation_z(90.0f32.to_radians()).mul_vec3(p.direction);
    } else if keyboard_input.pressed(KeyCode::D) {
      turning_force += Quat::from_rotation_z(-90.0f32.to_radians()).mul_vec3(p.direction);
    }

    p.direction = (p.direction + (turning_force * time.delta_seconds() * 3.0)).normalize();
  }
}

// pub fn move_player(
//   mut qry: Query<(&mut Moveable, &mut Transform, &Player)>,
//   mut gizmos: Gizmos,
// ) {
//   if let Ok((mut mov, mut t, p)) = qry.get_single_mut() {
//     let normalized = if p.direction == Vec3::ZERO {
//       Vec3::Y
//     } else {
//       p.direction.normalize()
//     };

//     t.rotation = Quat::from_rotation_z( p.direction.x.signum() * -1. * p.direction.angle_between(Vec3::Y));
//     gizmos.ray_2d(t.translation.xy(), p.direction.xy() * 100.0, Color::BLUE);

//     // TODO: move constant velocity to boid
//     mov.velocity = normalized * 250.0;
//   }
// }
