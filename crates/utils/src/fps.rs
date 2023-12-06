use std::fmt::Write;

use bevy::{
  diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
  prelude::*,
  utils::Duration,
};

const FONT_SIZE: f32 = 32.0;
const FONT_COLOR: Color = Color::RED;

const UPDATE_INTERVAL: Duration = Duration::from_secs(1);
const STRING_FORMAT: &str = "FPS: ";
const STRING_INITIAL: &str = "FPS: ...";

pub struct ScreenDiagsPlugin;

impl Plugin for ScreenDiagsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(FrameTimeDiagnosticsPlugin::default())
      .add_systems(Update, update_frame_rate)
      .init_resource::<ScreenDiagsState>()
      .init_resource::<FrameRate>()
      .add_systems(Update, update_text);
  }
}

pub struct ScreenDiagsTextPlugin;

impl Plugin for ScreenDiagsTextPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(ScreenDiagsPlugin)
      .add_systems(Startup, spawn_text);
  }
}

#[derive(Resource)]
pub struct ScreenDiagsState {
  pub timer: Timer,
  pub update_now: bool,
}

impl Default for ScreenDiagsState {
  fn default() -> Self {
    Self {
      timer: Timer::new(UPDATE_INTERVAL, TimerMode::Repeating),
      update_now: true,
    }
  }
}

impl ScreenDiagsState {
  pub fn enable(&mut self) {
    self.timer.unpause();
    self.update_now = true;
  }

  pub fn disable(&mut self) {
    self.timer.pause();
    self.update_now = true;
  }

  pub fn enabled(&self) -> bool {
    !self.timer.paused()
  }
}

#[derive(Resource, Default)]
pub struct FrameRate(pub f64);

fn update_frame_rate(
  time: Res<Time>,
  diagnostics: Res<DiagnosticsStore>,
  state_resource: Option<ResMut<ScreenDiagsState>>,
  mut frame_rate: ResMut<FrameRate>,
) {
  if let Some(mut state) = state_resource {
    if state.update_now || state.timer.tick(time.delta()).just_finished() {
      if state.timer.paused() {
        return;
      } else {
        let fps_diags = extract_fps(&diagnostics);

        if let Some(fps) = fps_diags {
          frame_rate.0 = fps;
        } else {
          frame_rate.0 = 0.0;
        }
      }
    }
  }
}

#[derive(Component)]
pub struct ScreenDiagsText;

fn update_text(
  time: Res<Time>,
  state_resource: Option<ResMut<ScreenDiagsState>>,
  mut text_query: Query<&mut Text, With<ScreenDiagsText>>,
  frame_rate: Res<FrameRate>,
) {
  if let Some(mut state) = state_resource {
    if state.update_now || state.timer.tick(time.delta()).just_finished() {
      if state.timer.paused() {
        // Time is paused so remove text
        for mut text in text_query.iter_mut() {
          let value = &mut text.sections[0].value;
          value.clear();
        }
      } else {
        for mut text in text_query.iter_mut() {
          let value = &mut text.sections[0].value;
          value.clear();

          write!(value, "{}{:.0}", STRING_FORMAT, frame_rate.0).unwrap();
        }
      }
    }
  }
}

fn extract_fps(diagnostics: &DiagnosticsStore) -> Option<f64> {
  diagnostics
    .get(FrameTimeDiagnosticsPlugin::FPS)
    .and_then(|fps| fps.average())
}

fn spawn_text(mut commands: Commands) {
  commands
    .spawn((
      NodeBundle {
        style: Style {
          align_items: AlignItems::End,
          justify_content: JustifyContent::Center,
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          ..default()
        },
        ..default()
      },
    ))
    .with_children(|parent| {
      parent
        .spawn(TextBundle {
          text: Text {
            sections: vec![TextSection {
              value: STRING_INITIAL.to_string(),
              style: TextStyle {
                font_size: FONT_SIZE,
                color: FONT_COLOR,
                ..default()
              },
            }],
            alignment: TextAlignment::Right,
            ..default()
          },
          ..default()
        })
        .insert(ScreenDiagsText);
    });
}
