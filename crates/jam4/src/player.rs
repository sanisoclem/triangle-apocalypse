use bevy::{ecs::system::EntityCommands, prelude::*, sprite::Mesh2dHandle};
use bevy_hanabi::prelude::*;

use crate::{
  boid::{Boid, BoidConfig},
  moveable::Moveable,
};

#[derive(Resource)]
pub struct PlayerInfo {
  pub score: u32,
  pub in_boost_mode: bool,
  pub mesh: Mesh2dHandle,
  pub boost_color: Handle<ColorMaterial>,
  pub normal_color: Handle<ColorMaterial>,
  pub boost_particles: Handle<EffectAsset>,
  pub normal_particles: Handle<EffectAsset>,
  pub audio_boost: Handle<AudioSource>,
  pub audio_slow: Handle<AudioSource>,
  // pub audio_die: Handle<AudioSource>,
}

impl FromWorld for PlayerInfo {
  fn from_world(world: &mut World) -> Self {
    let (boost_color, normal_color) = {
      let mut mats = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
      (
        mats.add(ColorMaterial::from(Color::rgb(0.0, 0.0, 10.0))),
        mats.add(ColorMaterial::from(Color::rgb(10.0, 0.0, 0.0))),
      )
    };

    let (boost_particles, normal_particles) = {
      let mut fx = world.get_resource_mut::<Assets<EffectAsset>>().unwrap();

      let mut color_gradient1 = Gradient::new();
      color_gradient1.add_key(0.0, Vec4::new(10.0, 10.0, 10.0, 1.0));
      color_gradient1.add_key(0.1, Vec4::new(10.0, 5.8, 5.8, 1.0));
      color_gradient1.add_key(0.2, Vec4::new(10.0, 2.0, 0.0, 1.0));
      color_gradient1.add_key(1.0, Vec4::new(10.0, 0.0, 0.0, 0.0));

      let mut boost_color = Gradient::new();
      boost_color.add_key(0.0, Vec4::new(10.0, 10.0, 10.0, 1.0));
      boost_color.add_key(0.1, Vec4::new(5.8, 5.8, 10.0, 1.0));
      boost_color.add_key(0.2, Vec4::new(0.0, 0.0, 10.0, 1.0));
      boost_color.add_key(1.0, Vec4::new(0.0, 0.0, 10.0, 0.0));

      let mut size_gradient1 = Gradient::new();
      size_gradient1.add_key(0.3, Vec2::new(1.0, 5.));
      size_gradient1.add_key(1.0, Vec2::splat(0.0));

      let writer = ExprWriter::new();

      let age = writer.lit(0.).expr();
      let init_age = SetAttributeModifier::new(Attribute::AGE, age);

      let lifetime = writer.lit(0.5).uniform(writer.lit(1.)).expr();
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

      let boost_vel = SetVelocityTangentModifier {
        origin: writer.lit(Vec3::ZERO).expr(),
        axis: writer.lit(Vec3::X).expr(),
        speed: writer.lit(2000.).expr(),
      };

      let module = writer.finish();

      // Create a new effect asset spawning 30 particles per second from a circle
      // and slowly fading from blue-ish to transparent over their lifetime.
      // By default the asset spawns the particles at Z=0.
      let spawner = Spawner::rate(100.0.into());
      let normal_particles = fx.add(
        EffectAsset::new(4096, spawner, module.clone())
          .with_name("player_normal")
          .init(init_pos)
          .init(init_vel)
          .init(init_age)
          .init(init_lifetime)
          .render(SizeOverLifetimeModifier {
            gradient: size_gradient1.clone(),
            screen_space_size: false,
          })
          .render(ColorOverLifetimeModifier {
            gradient: color_gradient1,
          }),
      );
      let boost_particles = fx.add(
        EffectAsset::new(4096, spawner, module)
          .with_name("player_boost")
          .init(init_pos)
          .init(boost_vel)
          .init(init_age)
          .init(init_lifetime)
          .render(SizeOverLifetimeModifier {
            gradient: size_gradient1,
            screen_space_size: false,
          })
          .render(ColorOverLifetimeModifier {
            gradient: boost_color,
          }),
      );

      (boost_particles, normal_particles)
    };

    let mesh = {
      let mut meshes = world.get_resource_mut::<Assets<Mesh>>().unwrap();
      let mesh: Mesh2dHandle = meshes
        .add(shape::RegularPolygon::new(20.0, 3).into())
        .into();
      mesh
    };
    let (audio_boost, audio_slow) = {
      let asset_server = world.get_resource_mut::<AssetServer>().unwrap();
      (
        asset_server.load("preload/607409__colorscrimsontears__upgrade.ogg"),
        asset_server.load("preload/169994__peepholecircus__power-down.ogg"),
      )
    };
    Self {
      boost_color,
      boost_particles,
      mesh,
      in_boost_mode: false,
      normal_color,
      normal_particles,
      score: 0,
      audio_boost,
      audio_slow,
    }
  }
}

#[derive(Component, Default)]
pub struct Player;

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
  pub effect: ParticleEffect,
  pub compiled_effect: CompiledParticleEffect,
}

pub fn spawn_player<'w, 's, 'a>(
  cmd: &'a mut Commands<'w, 's>,
  player: &PlayerInfo,
  spawn_point: Vec2,
  bconfig: &BoidConfig,
) -> EntityCommands<'w, 's, 'a> {
  cmd.spawn(PlayerBundle {
    moveable: Moveable::default(),
    boid: Boid {
      is_player: true,
      personal_space: 100.,
      turning_speed: bconfig.min_turn_speed,
      vision: 800.0,
      direction: Vec2::Y,
      speed: bconfig.max_speed,
    },
    mesh: player.mesh.clone(),
    effect: ParticleEffect::new(player.normal_particles.clone()),
    compiled_effect: CompiledParticleEffect::default(),
    material: player.normal_color.clone(),
    transform: Transform::from_translation(spawn_point.extend(-1.0))
      .with_scale(Vec3::new(1.0, 2.0, 1.0)),
    global_transform: default(),
    visibility: default(),
    inherited_visibility: default(),
    view_visibility: default(),
    player: Player,
  })
}
