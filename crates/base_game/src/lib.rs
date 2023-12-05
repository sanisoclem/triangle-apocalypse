use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_smud::prelude::*;
use jam4::{
  boid::Boid,
  moveable::{Moveable, MoveableBounds},
  GameModuleDescriptor, NativeGameModule, Player,
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

pub fn on_init(mut bounds: ResMut<MoveableBounds>) {
  let outer = sdfu::Box::new(Vec2::new(2000., 4000.));
  let inner = sdfu::Box::new(Vec2::new(1000., 3000.));
  let m1 = sdfu::Circle::new(200.0).translate(Vec2::new(300., 200.));
  let m2 = sdfu::Circle::new(150.).translate(Vec2::new(-200., 300.));
  let m3 = sdfu::Circle::new(125.).translate(Vec2::new(300., -300.));
  let m4 = sdfu::Circle::new(50.).translate(Vec2::new(100., 1500.));
  let shape = outer.subtract(inner.subtract(m1).subtract(m2).subtract(m3).subtract(m4));
  *bounds = MoveableBounds::from_sdf(shape);
}

pub fn on_setup(
  mut cmd: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
  asset_server: Res<AssetServer>,
) {
  let terrain_shader = asset_server.load("preload/terrain.wgsl");

  cmd.spawn(ShapeBundle {
    shape: SmudShape {
      color: Color::TOMATO,
      sdf: terrain_shader,
      frame: Frame::Quad(30000.),
      ..default()
    },
    ..default()
  });

  cmd
    .spawn(MaterialMesh2dBundle {
      mesh: meshes.add(shape::RegularPolygon::new(10., 3).into()).into(),
      material: materials.add(ColorMaterial::from(Color::rgb(7.5, 0.0, 7.5))),
      transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
      ..default()
    })
    .insert(Player::default())
    .insert(Moveable {
      // velocity: Vec3::Y * 300.,
      ..default()
    })
    .insert(Boid {
      is_player: true,
      ..default()
    });

  for x in -515..515 {
    cmd
      .spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(20., 3).into()).into(),
        material: materials.add(ColorMaterial::from(Color::rgb(0.5, 1.0, 0.5))),
        transform: Transform::from_translation(Vec3::new(0.0 + (x as f32) * 0.001, 0., 0.)),
        ..default()
      })
      .insert(Moveable::default())
      .insert(Boid::default());
  }
}

pub fn some_system(_qry: Query<Entity>) {}
