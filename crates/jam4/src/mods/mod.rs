use bevy::{ecs::schedule::ScheduleLabel, prelude::*};
pub use manager::*;

mod manager;

/// one-off schedule, run during loading phase
/// note the player camera is not yet created during loading phase
/// use for loading assets, expensive calculations, spawning huge amounts of entities
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct ModInit;

/// one-off schedule, run after loading phase
/// upon entering simulation phase
/// player camera may have already been created
/// set everything to visible here or load UI?
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct ModSetup;

#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct ModUpdate;

pub fn register_mods(world: &mut World) {
  let init_sched;
  let setup_sched: Schedule;
  let update_sched;
  {
    let mgr = world.resource::<ModManager>();
    init_sched = mgr.build_init_schedule(ModInit);
    setup_sched = mgr.build_setup_schedule(ModSetup);
    update_sched = mgr.build_update_schedule(ModUpdate);
  }

  world.add_schedule(init_sched);
  world.add_schedule(setup_sched);
  world.add_schedule(update_sched);
}

pub fn run_mod_init(world: &mut World) {
  world.run_schedule(ModInit);
}

pub fn run_mod_setup(world: &mut World) {
  world.run_schedule(ModSetup);
}

pub fn run_mod_update(world: &mut World) {
  world.run_schedule(ModUpdate);
}

