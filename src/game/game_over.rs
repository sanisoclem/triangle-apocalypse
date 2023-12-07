use bevy::prelude::*;
use jam4::GameControlCommand;
use utils::text::TextAnimation;

use utils::colors::*;

#[derive(Component)]
pub struct InGameOverScreen;

pub fn wait_to_retry(
  mut cmds: EventWriter<GameControlCommand>,
  keyboard_input: Res<Input<KeyCode>>,
) {
  if keyboard_input.just_pressed(KeyCode::Space) {
    cmds.send(GameControlCommand::Retry);
  }
}

pub fn on_game_over(mut cmd: Commands) {
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
          text: "Game Over\npress space to retry".to_owned(),
          animation_speed: 1.0,
        });
    });
}
