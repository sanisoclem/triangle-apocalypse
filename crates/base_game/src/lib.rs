use bevy::prelude::*;
use bevy_smud::prelude::*;
use jam4::{
  level::{LevelInfo, LevelRegistry},
  moveable::MoveableBounds,
  GameModuleDescriptor, NativeGameModule,
};
use sdfu::SDF;

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

fn register_setup(sched: &mut Schedule) {
  sched.add_systems(on_setup);
}

fn register_update(sched: &mut Schedule) {
  sched.add_systems(some_system);
}

pub fn on_init(mut lvl_registry: ResMut<LevelRegistry>, asset_server: Res<AssetServer>) {
  let lvl_id1 = 1u8.into();
  let lvl_id2 = 2u8.into();
  lvl_registry.levels.insert(
    lvl_id1,
    LevelInfo {
      next_level: Some(lvl_id2),
      ..build_level1(&asset_server)
    },
  );
  lvl_registry
    .levels
    .insert(lvl_id2, build_level2(&asset_server));
  lvl_registry.start_level = Some(lvl_id1);
}

pub fn on_setup(// mut cmd: Commands,
  // mut meshes: ResMut<Assets<Mesh>>,
  // mut materials: ResMut<Assets<ColorMaterial>>,
) {
  // for x in -515..515 {
  //   cmd
  //     .spawn(MaterialMesh2dBundle {
  //       mesh: meshes.add(shape::RegularPolygon::new(10., 3).into()).into(),
  //       material: materials.add(ColorMaterial::from(Color::rgb(0.5, 5.0, 0.5))),
  //       transform: Transform::from_translation(Vec3::new(0.0 + (x as f32) * 0.001, 0., 0.))
  //         .with_scale(Vec3::new(1.0, 2.0, 1.0)),
  //       ..default()
  //     })
  //     .insert(Moveable::default())
  //     .insert(Boid::default());
  // }
}

pub fn some_system(_qry: Query<Entity>) {}

fn build_level1(asset_server: &AssetServer) -> LevelInfo {
  let outer = sdfu::Box::new(Vec2::new(3000., 10000.));
  let inner = sdfu::Box::new(Vec2::new(2000., 9000.));
  let m1 = sdfu::Circle::new(200.0).translate(Vec2::new(300., 200.));
  let m2 = sdfu::Circle::new(150.).translate(Vec2::new(-200., 300.));
  let m3 = sdfu::Circle::new(125.).translate(Vec2::new(300., -300.));
  let m4 = sdfu::Circle::new(50.).translate(Vec2::new(100., 1500.));
  let shape = outer.subtract(inner.subtract(m1).subtract(m2).subtract(m3).subtract(m4));
  let finish_bounds = sdfu::Box::new(Vec2::new(3000., 1000.)).translate(Vec2::new(0.0, 9000.));
  let terrain_shader = asset_server.load("preload/terrain.wgsl");
  let terrain_finish_shader = asset_server.load("preload/terrain_finish.wgsl");

  let s = SmudShape {
    color: Color::TOMATO,
    sdf: terrain_shader,
    frame: Frame::Quad(30000.),
    ..default()
  };
  let fs = SmudShape {
    color: utils::colors::FAIRY.with_a(0.2),
    sdf: terrain_finish_shader,
    frame: Frame::Quad(3000.),
    ..default()
  };

  let lvl = LevelInfo {
    bounds: MoveableBounds::from_sdf(shape),
    finish_bounds: MoveableBounds::from_sdf(finish_bounds),
    finish_bounds_sdf: Some((fs, Vec3::new(0.0, 9000., 0.0))),
    bounds_sdf: Some(s),
    music: asset_server.load("preload/battle_1.ogg"),
    name: "Level 1".to_owned(),
    next_level: None,
    starting_point: Vec2::ZERO,
  };
  lvl
}

fn build_level2(asset_server: &AssetServer) -> LevelInfo {
  let outer = sdfu::Box::new(Vec2::new(2000., 10000.));
  let inner = sdfu::Box::new(Vec2::new(1000., 9000.));
  let m1 = sdfu::Circle::new(200.0).translate(Vec2::new(300., 200.));
  let m2 = sdfu::Circle::new(150.).translate(Vec2::new(-200., 300.));
  let m3 = sdfu::Circle::new(125.).translate(Vec2::new(300., -300.));
  let m4 = sdfu::Circle::new(50.).translate(Vec2::new(100., 1500.));
  let shape = outer.subtract(inner.subtract(m1).subtract(m2).subtract(m3).subtract(m4));
  let finish_bounds = sdfu::Box::new(Vec2::new(2000., 1000.)).translate(Vec2::new(0.0, 9000.));
  let terrain_shader = asset_server.load("preload/terrain_2.wgsl");
  let terrain_finish_shader = asset_server.load("preload/terrain_finish_2.wgsl");

  let s = SmudShape {
    color: Color::TOMATO,
    sdf: terrain_shader,
    frame: Frame::Quad(30000.),
    ..default()
  };
  let fs = SmudShape {
    color: utils::colors::FAIRY.with_a(0.2),
    sdf: terrain_finish_shader,
    frame: Frame::Quad(3000.),
    ..default()
  };

  let lvl = LevelInfo {
    bounds: MoveableBounds::from_sdf(shape),
    finish_bounds: MoveableBounds::from_sdf(finish_bounds),
    finish_bounds_sdf: Some((fs, Vec3::new(0.0, 9000., 0.0))),
    bounds_sdf: Some(s),
    music: asset_server.load("preload/battle_1.ogg"),
    name: "Level 2".to_owned(),
    next_level: None,
    starting_point: Vec2::ZERO,
  };
  lvl
}
