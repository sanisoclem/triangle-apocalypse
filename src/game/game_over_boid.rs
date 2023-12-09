use bevy::prelude::*;
use jam4::{
  level::{LevelManager, LevelRegistry},
  GameControlCommand,
};
use utils::text::TextAnimation;

use utils::colors::*;

use crate::jukebox::{BgMusic, MusicCommand};

use super::game_over_bounds::InGameOverScreen;

pub fn on_game_over_boid(
  mut cmd: Commands,
  mut cmds: EventWriter<MusicCommand>,
  lvl_mgr: ResMut<LevelManager>,
  lvl_reg: Res<LevelRegistry>,
) {
  let Some(level_id) = lvl_mgr.current_level else {
    return;
  };
  let lvl = lvl_reg.get_level(&level_id);
  let target = lvl.rescue_goal.expect("should have a rescue goal");

  cmds.send(MusicCommand::Play(BgMusic::GameOver));
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
      InGameOverScreen,
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
          text: format!(
            "Game Over\nyou need to rescue {} triangles to clear the level\npress space to retry",
            target
          ),
          animation_speed: 1.0,
        });
    });
}
