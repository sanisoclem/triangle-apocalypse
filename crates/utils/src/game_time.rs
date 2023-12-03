use bevy::{prelude::*, time::Stopwatch};

#[derive(Resource, Deref, DerefMut)]
struct GameWatch(Stopwatch);

impl Default for GameWatch {
  fn default() -> Self {
    let mut w = Stopwatch::new();
    w.pause();
    w.reset();
    Self(w)
  }
}

#[derive(Event)]
pub enum GameTimeCommand {
  Restart,
  Suspend,
  Resume,
}

pub struct GameTimePlugin;
impl Plugin for GameTimePlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<GameWatch>()
      .add_event::<GameTimeCommand>()
      .add_systems(Update, (tick_watch, process_commands));
  }
}

fn tick_watch(mut game_timer: ResMut<GameWatch>, time: Res<Time>) {
  game_timer.0.tick(time.delta());
}

fn process_commands(mut events: EventReader<GameTimeCommand>, mut game_timer: ResMut<GameWatch>) {
  for evt in events.read() {
    match evt {
      GameTimeCommand::Restart => {
        game_timer.0.reset();
        game_timer.0.unpause();
      }
      GameTimeCommand::Suspend => {
        game_timer.0.pause();
      }
      GameTimeCommand::Resume => {
        game_timer.0.unpause();
      }
    }
  }
}
