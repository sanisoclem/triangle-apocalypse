use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bevy_hanabi::ParticleEffect;
use jam4::{
  boid::{Boid, BoidConfig, TamedBoid},
  level::{LevelManager, LevelRegistry},
  Player, PlayerInfo,
};

use crate::jukebox::{BgMusic, MusicCommand};

#[derive(Component)]
pub struct InPlayingScreen;

#[derive(Component)]
pub struct ScoreBoard;

#[derive(Component)]
pub struct TimeRemaning;

pub fn setup_player_ui(mut cmd: Commands, mut cmds: EventWriter<MusicCommand>) {
  cmds.send(MusicCommand::Play(BgMusic::MainTheme));
  cmd
    .spawn((
      NodeBundle {
        style: Style {
          width: Val::Percent(100.0),
          height: Val::Px(60.0),
          bottom: Val::Px(0.0),
          position_type: PositionType::Absolute,
          padding: UiRect::all(Val::Px(10.0)),
          display: Display::Flex,
          justify_content: JustifyContent::SpaceBetween,
          ..default()
        },
        ..default()
      },
      InPlayingScreen,
    ))
    .with_children(|parent| {
      parent
        .spawn(TextBundle {
          text: Text {
            sections: vec![TextSection {
              value: "".to_owned(),
              style: TextStyle {
                font_size: 40.,
                color: Color::RED,
                ..default()
              },
            }],
            alignment: TextAlignment::Right,
            ..default()
          },
          ..default()
        })
        .insert(TimeRemaning);
      parent
        .spawn(TextBundle {
          text: Text {
            sections: vec![TextSection {
              value: "".to_owned(),
              style: TextStyle {
                font_size: 40.,
                color: utils::colors::FAIRY,
                ..default()
              },
            }],
            alignment: TextAlignment::Left,
            ..default()
          },
          ..default()
        })
        .insert(ScoreBoard);
    });
}

pub fn update_player_ui(
  qry_boid: Query<Entity, (With<Boid>, With<TamedBoid>, Without<Player>)>,
  // qry_boid2: Query<Entity, (With<Boid>, Without<TamedBoid>, Without<Player>)>,
  mut qry_score: Query<&mut Text, (With<ScoreBoard>, Without<TimeRemaning>)>,
  mut qry_time: Query<&mut Text, (With<TimeRemaning>, Without<ScoreBoard>)>,
  lvl_mgr: ResMut<LevelManager>,
  lvl_reg: Res<LevelRegistry>,
) {
  let Ok(mut txt_score) = qry_score.get_single_mut() else {
    return;
  };
  let Ok(mut txt_time) = qry_time.get_single_mut() else {
    return;
  };
  let Some(level_id) = lvl_mgr.current_level else {
    return;
  };
  let lvl = lvl_reg.get_level(&level_id);

  let tamed = qry_boid.iter().count();
  if let Some(rescue_target) = lvl.rescue_goal {
    txt_score.sections.first_mut().unwrap().value = format!("{}/{}", tamed, rescue_target);
  } else {
    txt_score.sections.first_mut().unwrap().value = format!("{} ", tamed);
  }
  if let Some(time_target) = lvl.time_goal {
    let s = time_target.as_secs_f32() - lvl_mgr.watch.elapsed_secs();
    let mm = (s / 60.).floor() as u8;
    let ss = (s.floor() as u16) % 60;
    txt_time.sections.first_mut().unwrap().value = format!("{:02}:{:02}", mm, ss);
  } else {
    txt_time.sections.first_mut().unwrap().value = "".to_owned();
  }
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
  mut cmd: Commands,
  mut qry: Query<(&mut ParticleEffect, &mut Boid, &mut Handle<ColorMaterial>), With<Player>>,
  keyboard_input: Res<Input<KeyCode>>,
  mut player: ResMut<PlayerInfo>,
  bconfig: Res<BoidConfig>,
  mut qry_music : Query<(&mut AudioSink), With<BgMusic>>
) {
  let Ok((mut fx, mut boid, mut mat)) = qry.get_single_mut() else {
    return;
  };

  if keyboard_input.just_pressed(KeyCode::Space) {
    player.in_boost_mode = true;

    cmd.spawn(AudioBundle {
      source: player.audio_boost.clone(),
      settings: PlaybackSettings::DESPAWN,
    });
    fx.handle = player.boost_particles.clone();
    *mat = player.boost_color.clone();
    boid.speed = bconfig.min_speed;
    boid.turning_speed = bconfig.min_turn_speed;

    for m in qry_music.iter_mut() {
      m.set_speed(0.95);
    }
  }

  if keyboard_input.just_released(KeyCode::Space) {
    player.in_boost_mode = false;
    cmd.spawn(AudioBundle {
      source: player.audio_slow.clone(),
      settings: PlaybackSettings::DESPAWN,
    });
    fx.handle = player.normal_particles.clone();
    *mat = player.normal_color.clone();
    boid.speed = bconfig.max_speed;
    boid.turning_speed = bconfig.max_turn_speed;

    for m in qry_music.iter_mut() {
      m.set_speed(1.0);
    }
  }
}
