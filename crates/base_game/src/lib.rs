use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use jam4::{GameModuleDescriptor, NativeGameModule, Player};

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

pub fn on_init() {}

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
      mesh: meshes
        .add(shape::RegularPolygon::new(100., 6).into())
        .into(),
      material: materials.add(ColorMaterial::from(Color::rgb(7.5, 0.0, 7.5))),
      transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
      ..default()
    })
    .insert(Player);
}

pub fn some_system(_qry: Query<Entity>) {}
