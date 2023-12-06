use std::time::Duration;

use bevy::prelude::*;

pub struct TextAnimationPlugin;

impl Plugin for TextAnimationPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, (start_animate_text, animate_text));
  }
}

#[derive(Component)]
pub struct TextAnimation {
  pub text: String,
  pub animation_speed: f32,
  // pub animation_type: ?,
  // pub easing_func: ?
}

// pub struct ScriptSegment {
//   pub text: String,
//   pub animation_speed: f32,
//   pub display_for_seconds: f32,
// }
// pub enum ScriptTransitionType {
//   Keyboard(KeyCode),
//   Click(MouseButton),
//   KeyboardOrClick(KeyCode, MouseButton),
//   Duration(Duration)
// }

#[derive(Component)]
pub struct AnimatingText {
  pub progress: f32,
}

#[derive(Component)]
pub struct AnimatedText;

pub fn start_animate_text(
  mut cmd: Commands,
  qry: Query<Entity, (Changed<TextAnimation>, With<Text>)>,
) {
  for e in qry.iter() {
    cmd.entity(e).insert(AnimatingText { progress: 0.0 });
  }
}

pub fn animate_text(
  mut cmd: Commands,
  mut qry: Query<(Entity, &mut Text, &mut AnimatingText, &TextAnimation)>,
  time: Res<Time>,
) {
  for (e, mut txt, mut prog, anim) in qry.iter_mut() {
    prog.progress += anim.animation_speed * time.delta_seconds();

    if prog.progress >= 1.0 {
      cmd.entity(e).remove::<AnimatingText>().insert(AnimatedText);
      txt.sections.first_mut().unwrap().value = anim.text.clone();
    } else {
      let l = (prog.progress * anim.text.len() as f32) as usize;
      txt.sections.first_mut().unwrap().value = anim.text[..l].to_owned();
    }
  }
}
