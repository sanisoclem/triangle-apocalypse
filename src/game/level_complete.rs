use bevy::prelude::*;
use jam4::GameControlCommand;

#[derive(Component)]
pub struct InLevelCompleteScreen;

pub fn on_level_complete(mut cmds: EventWriter<GameControlCommand>) {
  // TODO: show UI with level summary
  // wait for input or time
  // send NextLevel command
  info!("TODO: level complete screen");
  cmds.send(GameControlCommand::NextLevel)
}
