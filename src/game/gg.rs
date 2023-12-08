use bevy::prelude::*;
use jam4::PlayerInfo;
use utils::text::TextAnimation;

use utils::colors::*;

#[derive(Component)]
pub struct InGGScreen;

pub fn on_game_complete(mut cmd: Commands, player: Res<PlayerInfo>) {
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
      InGGScreen,
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
            "GG\nYou herded {} shapes\nThank you for playing!",
            player.score
          ),
          animation_speed: 1.0,
        });
    });
}
