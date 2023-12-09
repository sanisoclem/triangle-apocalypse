use bevy::prelude::*;
use bevy_hanabi::prelude::*;

#[derive(Resource)]
pub struct BoidConfig {
  pub max_speed: f32,
  pub min_speed: f32,
  pub min_turn_speed: f32,
  pub max_turn_speed: f32,
  pub safe_turn_speed: f32,
  pub boundary: f32,
  pub cohesion: f32,
  pub alignment: f32,
  pub repulsion: f32,
  pub lprobe: Mat2,
  pub rprobe: Mat2,
  pub lforce: Mat2,
  pub rforce: Mat2,
  pub show_forces: bool,
  pub show_direction: bool,
  pub show_personal_space: bool,
  pub show_vision: bool,
  pub show_bounds: bool,
  pub player_influence: f32,
  pub cotrails: Handle<EffectAsset>,
  pub color_wild: Handle<ColorMaterial>,
  pub color_tamed: Handle<ColorMaterial>,
  pub color_tamed_boosted: Handle<ColorMaterial>,
}

impl FromWorld for BoidConfig {
  fn from_world(world: &mut World) -> Self {
    let mut effects = world.get_resource_mut::<Assets<EffectAsset>>().unwrap();

    let mut color_gradient1 = Gradient::new();
    color_gradient1.add_key(0.0, Vec4::new(10.0, 10.0, 10.0, 1.0));
    color_gradient1.add_key(0.4, Vec4::new(5.0, 2.0, 0.0, 1.0));
    color_gradient1.add_key(1.0, Vec4::new(5.0, 0.0, 0.0, 0.0));

    let mut size_gradient1 = Gradient::new();
    size_gradient1.add_key(0.3, Vec2::new(1.0, 5.));
    size_gradient1.add_key(1.0, Vec2::splat(0.0));

    let writer = ExprWriter::new();

    let age = writer.lit(0.).expr();
    let init_age = SetAttributeModifier::new(Attribute::AGE, age);

    let lifetime = writer.lit(1.).uniform(writer.lit(10.)).expr();
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
    let spawner = Spawner::rate(20.0.into());
    let effect = effects.add(
      EffectAsset::new(4096, spawner, writer.finish())
        .with_name("boid_effects")
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

    let (color_wild, color_tamed, color_tamed_boosted) = {
      let mut mats = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
      let color_wild = mats.add(ColorMaterial::from(Color::rgb_u8(244, 175, 45)));
      let color_tame = mats.add(ColorMaterial::from(Color::rgb(0.5, 5.0, 0.5)));
      let color_tamed_boosted = mats.add(ColorMaterial::from(Color::rgb(0.5, 5.0, 5.0)));
      (color_wild, color_tame, color_tamed_boosted)
    };

    BoidConfig {
      max_speed: 1000.,
      min_speed: 500.,
      min_turn_speed: 1.0,
      max_turn_speed: 5.0,
      safe_turn_speed: 10.0,
      player_influence: 100.,
      boundary: 50.0,
      cohesion: 1.0,
      alignment: 1.0,
      repulsion: 10.0,
      lprobe: Mat2::from_angle(45.0f32.to_radians()),
      rprobe: Mat2::from_angle(-45.0f32.to_radians()),
      lforce: Mat2::from_angle(90.0f32.to_radians()),
      rforce: Mat2::from_angle(-90.0f32.to_radians()),
      show_forces: false,
      show_direction: false,
      show_personal_space: false,
      show_vision: false,
      show_bounds: true,
      cotrails: effect,
      color_wild,
      color_tamed,
      color_tamed_boosted,
    }
  }
}
