use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use jam4::{
  boid::Boid,
  moveable::{Moveable, MoveableBounds},
  GameModuleDescriptor, NativeGameModule, Player,
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
  *bounds = MoveableBounds::Box(Vec2::new(700.0, 400.0))
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
      mesh: meshes.add(shape::RegularPolygon::new(10., 3).into()).into(),
      material: materials.add(ColorMaterial::from(Color::rgb(7.5, 0.0, 7.5))),
      transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
      ..default()
    })
    .insert(Player::default())
    .insert(Moveable::default())
    .insert(Boid {
      is_player: true,
      ..default()
    });

  for x in -250..250 {
    cmd
      .spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(10., 3).into()).into(),
        material: materials.add(ColorMaterial::from(Color::rgb(0.5, 1.0, 0.5))),
        transform: Transform::from_translation(Vec3::new(10.0 + (x as f32) * 5., 0., 0.)),
        ..default()
      })
      .insert(Moveable::default())
      .insert(Boid::default());
  }
}

pub fn some_system(_qry: Query<Entity>, mut gizmos: Gizmos) {
  gizmos.rect_2d(Vec2::ZERO, 0.0, Vec2::new(1400., 800.), Color::GREEN)
}
