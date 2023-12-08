use bevy::{ecs::system::EntityCommands, prelude::*, sprite::Mesh2dHandle};
use bevy_hanabi::prelude::*;

use crate::{boid::Boid, moveable::Moveable};

#[derive(Resource, Default)]
pub struct PlayerInfo {
  pub score: u32,
  pub material: Handle<ColorMaterial>,
  pub mesh: Mesh2dHandle,
}
#[derive(Component, Default)]
pub struct Player {
  pub influence_radius: f32,
}

#[derive(Bundle)]
pub struct PlayerBundle {
  pub mesh: Mesh2dHandle,
  pub material: Handle<ColorMaterial>,
  pub transform: Transform,
  pub global_transform: GlobalTransform,
  pub visibility: Visibility,
  pub inherited_visibility: InheritedVisibility,
  pub view_visibility: ViewVisibility,
  pub player: Player,
  pub moveable: Moveable,
  pub boid: Boid,
}

impl Default for PlayerBundle {
  fn default() -> Self {
    Self {
      moveable: Moveable { ..default() },
      boid: Boid {
        is_player: true,
        personal_space: 100.,
        turning_speed: 5.,
        ..default()
      },
      mesh: default(),
      material: default(),
      transform: default(),
      global_transform: default(),
      visibility: default(),
      inherited_visibility: default(),
      view_visibility: default(),
      player: Player {
        influence_radius: 500.,
      },
    }
  }
}

pub fn spawn_player<'w, 's, 'a>(
  cmd: &'a mut Commands<'w, 's>,
  player: &mut PlayerInfo,
  meshes: &mut Assets<Mesh>,
  materials: &mut Assets<ColorMaterial>,
  effects: &mut Assets<EffectAsset>,
  spawn_point: Vec2,
) -> EntityCommands<'w, 's, 'a> {
  let mut color_gradient1 = Gradient::new();
  color_gradient1.add_key(0.0, Vec4::new(10.0, 10.0, 10.0, 1.0));
  color_gradient1.add_key(0.1, Vec4::new(0.8, 0.8, 10.0, 1.0));
  color_gradient1.add_key(0.2, Vec4::new(5.0, 2.0, 0.0, 1.0));
  color_gradient1.add_key(1.0, Vec4::new(5.0, 0.0, 0.0, 0.0));

  let mut size_gradient1 = Gradient::new();
  size_gradient1.add_key(0.3, Vec2::new(1.0, 5.));
  size_gradient1.add_key(1.0, Vec2::splat(0.0));

  let writer = ExprWriter::new();

  let age = writer.lit(0.).expr();
  let init_age = SetAttributeModifier::new(Attribute::AGE, age);

  let lifetime = writer.lit(0.5).uniform(writer.lit(1.)) .expr();
  let init_lifetime = SetAttributeModifier::new(Attribute::LIFETIME, lifetime);

  let init_pos = SetPositionCircleModifier {
    center: writer.lit(Vec3::ZERO).expr(),
    axis: writer.lit(Vec3::Z).expr(),
    radius: writer.lit(10.0).expr(),
    dimension: ShapeDimension::Surface,
  };

  let init_vel = SetVelocityTangentModifier {
    origin: writer.lit(Vec3::ZERO).expr(),
    axis: writer.lit(Vec3::X).expr(),
    speed: writer.lit(0.).expr(),
  };

  // Create a new effect asset spawning 30 particles per second from a circle
  // and slowly fading from blue-ish to transparent over their lifetime.
  // By default the asset spawns the particles at Z=0.
  let spawner = Spawner::rate(100.0.into());
  let effect = effects.add(
    EffectAsset::new(4096, spawner, writer.finish())
      .with_name("2d")
      .init(init_pos)
      .init(init_vel)
      .init(init_age)
      .init(init_lifetime)
      .render(SizeOverLifetimeModifier {
        gradient: size_gradient1,
        screen_space_size: false,
      })
      .render(ColorOverLifetimeModifier {
        gradient: color_gradient1,
      }),
  );

  player.mesh = meshes
    .add(shape::RegularPolygon::new(20.0, 3).into())
    .into();
  player.material = materials.add(ColorMaterial::from(Color::rgb(7.5, 0.0, 7.5)));

  let mut pcmd = cmd.spawn(PlayerBundle {
    mesh: player.mesh.clone(),
    material: player.material.clone(),
    transform: Transform::from_translation(spawn_point.extend(-1.0))
      .with_scale(Vec3::new(1.0, 2.0, 1.0)),
    ..default()
  });

  pcmd.with_children(|p| {
    p.spawn((
      ParticleEffectBundle {
        effect: ParticleEffect::new(effect),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..Default::default()
      },
    ));
  });

  pcmd
}
