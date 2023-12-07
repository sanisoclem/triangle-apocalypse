use std::fmt::format;

use bevy::prelude::*;
use jam4::{
  level::{LevelManager, LevelRegistry},
  GameControlCommand,
};
use utils::text::TextAnimation;

use utils::colors::*;

#[derive(Component)]
pub struct InLevelCompleteScreen;

pub fn wait_to_next_level(
  mut cmds: EventWriter<GameControlCommand>,
  keyboard_input: Res<Input<KeyCode>>,
) {
  if keyboard_input.just_pressed(KeyCode::Space) {
    cmds.send(GameControlCommand::NextLevel);
  }
}

pub fn setup_level_complete(
  mut cmd: Commands,
  lvl_mgr: ResMut<LevelManager>,
  lvl_reg: Res<LevelRegistry>,
  mut cmds: EventWriter<GameControlCommand>,
) {
  let level_id = lvl_mgr.current_level.unwrap();
  let lvl = lvl_reg.get_level(&level_id);
  let Some(_nxt_level_id) = lvl.next_level else {
    cmds.send(GameControlCommand::NextLevel);
    return;
  };
  cmd
    .spawn((
      NodeBundle {
        style: Style {
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          display: Display::Flex,
          flex_direction: FlexDirection::Column,
          ..default()
        },
        background_color: BackgroundColor(RAISIN.with_a(0.8)),
        ..default()
      },
      InLevelCompleteScreen,
    ))
    .with_children(|parent| {
      parent
        .spawn(
          TextBundle::from_section(
            "",
            TextStyle {
              font_size: 30.0,
              color: MISTY,
              ..default()
            },
          )
          .with_text_alignment(TextAlignment::Center)
          .with_style(Style {
            margin: UiRect::top(Val::Px(50.0)),
            ..default()
          }),
        )
        .insert(TextAnimation {
          text: format!("{} complete\nPress space to continue", lvl.name),
          animation_speed: 1.0,
        });
    });
}
