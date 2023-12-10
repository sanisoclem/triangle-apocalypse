use bevy::prelude::*;
use jam4::{
  level::{LevelInfo, LevelRegistry},
  GameModuleDescriptor, NativeGameModule,
};

mod level1;
mod level2;
mod level3;
mod level4;

pub fn get_module() -> GameModuleDescriptor {
  GameModuleDescriptor::Native(NativeGameModule {
    register_init,
    register_setup,
    register_update,
  })
}

fn register_init(sched: &mut Schedule) {
  sched.add_systems(on_init);
}

fn register_setup(_sched: &mut Schedule) {}

fn register_update(_sched: &mut Schedule) {}

pub fn on_init(mut lvl_registry: ResMut<LevelRegistry>, asset_server: Res<AssetServer>) {
  let lvl_id1 = 1u8.into();
  let lvl_id2 = 2u8.into();
  let lvl_id3 = 3u8.into();
  let lvl_id4 = 4u8.into();
  lvl_registry.levels.insert(
    lvl_id1,
    LevelInfo {
      next_level: Some(lvl_id2),
      ..level1::build_level(&asset_server)
    },
  );
  lvl_registry.levels.insert(
    lvl_id2,
    LevelInfo {
      next_level: Some(lvl_id3),
      ..level2::build_level(&asset_server)
    },
  );
  lvl_registry.levels.insert(
    lvl_id3,
    LevelInfo {
      next_level: Some(lvl_id4),
      ..level3::build_level(&asset_server)
    },
  );
  lvl_registry
    .levels
    .insert(lvl_id4, level4::build_level(&asset_server));
  lvl_registry.start_level = Some(lvl_id1);
}
