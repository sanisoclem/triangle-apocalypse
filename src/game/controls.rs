use bevy::prelude::*;
use bevy_hanabi::ParticleEffect;
use jam4::{
  boid::{Boid, BoidConfig, TamedBoid},
  Player, PlayerInfo,
};

#[derive(Component)]
pub struct InPlayingScreen;

#[derive(Component)]
pub struct ScoreBoard;

pub fn setup_player_ui(mut cmd: Commands) {
  cmd
    .spawn((
      NodeBundle {
        style: Style {
          width: Val::Percent(100.0),
          height: Val::Px(60.0),
          bottom: Val::Px(0.0),
          position_type: PositionType::Absolute,
          padding: UiRect::all(Val::Px(10.0)),
          ..default()
        },
        ..default()
      },
      InPlayingScreen,
    ))
    .with_children(|parent| {
      parent
        .spawn(TextBundle {
          style: Style {
            width: Val::Percent(100.0),
            ..default()
          },
          text: Text {
            sections: vec![TextSection {
              value: "".to_owned(),
              style: TextStyle {
                font_size: 40.,
                color: utils::colors::FAIRY,
                ..default()
              },
            }],
            alignment: TextAlignment::Right,
            ..default()
          },
          ..default()
        })
        .insert(ScoreBoard);
    });
}

pub fn update_player_ui(
  qry_boid: Query<Entity, (With<Boid>, With<TamedBoid>, Without<Player>)>,
  qry_boid2: Query<Entity, (With<Boid>, Without<TamedBoid>, Without<Player>)>,
  mut qry_ui: Query<&mut Text, With<ScoreBoard>>,
  player: Res<PlayerInfo>,
) {
  let Ok(mut txt) = qry_ui.get_single_mut() else {
    return;
  };
  let tamed = qry_boid.iter().count();
  let wild = qry_boid2.iter().count();
  txt.sections.first_mut().unwrap().value =
    format!("{} + {}/{}", player.score, tamed, tamed + wild);
}

pub fn calc_player_direction(
  mut qry: Query<&mut Boid, With<Player>>,
  keyboard_input: Res<Input<KeyCode>>,
  time: Res<Time>,
) {
  if let Ok(mut p) = qry.get_single_mut() {
    let mut turning_force = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::A) {
      turning_force += Mat2::from_angle(90.0f32.to_radians()).mul_vec2(p.direction);
    } else if keyboard_input.pressed(KeyCode::D) {
      turning_force += Mat2::from_angle(-90.0f32.to_radians()).mul_vec2(p.direction);
    }

    p.direction =
      (p.direction + (turning_force * time.delta_seconds() * p.turning_speed)).normalize();
  }
}

pub fn toggle_player_mode(
  mut qry: Query<(&mut ParticleEffect, &mut Boid, &mut Handle<ColorMaterial>), With<Player>>,
  keyboard_input: Res<Input<KeyCode>>,
  mut player: ResMut<PlayerInfo>,
  bconfig: Res<BoidConfig>,
) {
  if !keyboard_input.just_pressed(KeyCode::Space) {
    return;
  }
  let Ok((mut fx, mut boid, mut mat)) = qry.get_single_mut() else {
    return;
  };

  player.in_boost_mode = !player.in_boost_mode;

  if player.in_boost_mode {
    fx.handle = player.boost_particles.clone();
    *mat = player.boost_color.clone();
    boid.speed = bconfig.max_speed;
    boid.turning_speed = bconfig.min_turn_speed;
  } else {
    fx.handle = player.normal_particles.clone();
    *mat = player.normal_color.clone();
    boid.speed = bconfig.min_speed;
    boid.turning_speed = bconfig.max_turn_speed;
  }
}
