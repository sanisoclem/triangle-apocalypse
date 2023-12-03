use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use jam4::{
  moveable::{Moveable, MoveableBounds},
  GameModuleDescriptor, NativeGameModule, Player, boid::Boid,
};

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
  *bounds = MoveableBounds::Box(Vec2::new(500.0, 300.0))
}

pub fn on_setup(
  mut cmd: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  // TODO:
  // - spawn static terrain
  // - spawn player

  cmd
    .spawn(MaterialMesh2dBundle {
      mesh: meshes.add(shape::RegularPolygon::new(5., 3).into()).into(),
      material: materials.add(ColorMaterial::from(Color::rgb(7.5, 0.0, 7.5))),
      transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
      ..default()
    })
    .insert(Player);

  for x in 0..50 {
    let rot = Quat::from_rotation_z(x as f32);
    cmd
      .spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(5., 3).into()).into(),
        material: materials.add(ColorMaterial::from(Color::rgb(0.5, 10.0, 7.5))),
        transform: Transform::from_rotation(rot).with_translation(Vec3::new(10.0 + (x as f32) * 5., 0., 0.))
          ,//.with_rotation(rot),
        ..default()
      })
      .insert(Moveable {
        velocity: rot.mul_vec3(Vec3::NEG_Y * 250.),
      });
  }

  // for x in 0..50 {
  //   cmd
  //     .spawn(MaterialMesh2dBundle {
  //       mesh: meshes.add(shape::RegularPolygon::new(5., 3).into()).into(),
  //       material: materials.add(ColorMaterial::from(Color::rgb(0.5, 10.0, 7.5))),
  //       transform: Transform::from_translation(Vec3::new(10.0 + (x as f32) * 5., 0., 0.)),
  //       ..default()
  //     })
  //     .insert(Moveable::default())
  //     .insert(Boid::default());
  // }
}

pub fn some_system(_qry: Query<Entity>) {}
