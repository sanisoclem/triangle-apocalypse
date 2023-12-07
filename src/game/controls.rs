use bevy::prelude::*;
use jam4::{
  boid::{Boid, BoidConfig},
  moveable::Moveable,
  Player,
};

use crate::colors::*;

#[derive(Component)]
pub struct InPlayingScreen;

#[derive(Component)]
pub struct SpeedBar;

pub fn setup_player_ui(mut cmd: Commands) {
  cmd
    .spawn((
      NodeBundle {
        style: Style {
          width: Val::Percent(100.0),
          height: Val::Px(10.0),
          bottom: Val::Px(0.0),
          position_type: PositionType::Absolute,
          ..default()
        },
        background_color: BackgroundColor(RAISIN.with_a(0.8)),
        ..default()
      },
      InPlayingScreen,
    ))
    .with_children(|parent| {
      parent
        .spawn(NodeBundle {
          style: Style {
            position_type: PositionType::Relative,
            width: Val::Percent(50.0),
            height: Val::Percent(100.0),
            ..default()
          },
          background_color: BackgroundColor(Color::rgb_u8(244, 175, 45)),
          ..default()
        })
        .insert(SpeedBar);
    });
}

pub fn update_player_ui(
  mut qry_ui: Query<&mut Style, With<SpeedBar>>,
  qry: Query<(&Boid, &Moveable), With<Player>>,
  bconfig: Res<BoidConfig>,
) {
  let Ok(mut style) = qry_ui.get_single_mut() else {
    return;
  };
  let Ok((boid, _m)) = qry.get_single() else {
    return;
  };

  style.width = Val::Percent(
    10. + (boid.speed - bconfig.min_speed) / (bconfig.max_speed - bconfig.min_speed) * 90.0,
  );
}

pub fn calc_player_direction(
  mut qry: Query<&mut Boid, With<Player>>,
  keyboard_input: Res<Input<KeyCode>>,
  time: Res<Time>,
  bconfig: Res<BoidConfig>,
) {
  if let Ok(mut p) = qry.get_single_mut() {
    let acceleration = 600.;
    let mut turning_force = Vec2::ZERO;
    let mut delta_acceleration = 0.;

    if keyboard_input.pressed(KeyCode::A) {
      turning_force += Mat2::from_angle(90.0f32.to_radians()).mul_vec2(p.direction);
    } else if keyboard_input.pressed(KeyCode::D) {
      turning_force += Mat2::from_angle(-90.0f32.to_radians()).mul_vec2(p.direction);
    }

    if keyboard_input.pressed(KeyCode::W) {
      delta_acceleration = acceleration;
    } else if keyboard_input.pressed(KeyCode::S) {
      delta_acceleration = -acceleration;
    }

    p.direction =
      (p.direction + (turning_force * time.delta_seconds() * p.turning_speed)).normalize();
    p.speed = (p.speed + (delta_acceleration * time.delta_seconds()))
      .clamp(bconfig.min_speed, bconfig.max_speed);
  }
}
