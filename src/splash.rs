use bevy::{
  asset::LoadedFolder,
  core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
  prelude::*,
  render::{
    camera::ScalingMode,
    mesh::{Indices, MeshVertexBufferLayout},
    render_resource::{
      AsBindGroup, PrimitiveTopology, RenderPipelineDescriptor, ShaderRef,
      SpecializedMeshPipelineError,
    },
  },
  sprite::{Material2d, Material2dKey, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle},
};
use jam4::{GameControlCommand, ModManager, SimulationState};
use utils::{despawn_screen, text::TextAnimation};

use utils::colors::*;

use crate::jukebox::{BgMusic, MusicCommand};

pub trait SplashExtensions {
  fn add_splash_screen<T: States + Copy>(&mut self, show_on_state: T, next_state: T) -> &mut Self;
}

impl SplashExtensions for App {
  fn add_splash_screen<T: States + Copy>(&mut self, show_on_state: T, next_state: T) -> &mut Self {
    self
      .add_plugins(Material2dPlugin::<SplashMaterial>::default())
      .init_resource::<SplashState>()
      .insert_resource(SplashNextState(next_state))
      .add_event::<SplashLog>()
      .add_systems(
        OnEnter(show_on_state),
        (preload_assets, build_bg, (splash_setup, init_game)).chain(),
      )
      .add_systems(
        Update,
        (wait_for_preload_assets, show_logs, go_to_next_state::<T>).run_if(in_state(show_on_state)),
      )
      .add_systems(OnExit(show_on_state), despawn_screen::<OnSplashScreen>)
      // skip menus, create a new game as soon as simulation is ready
      .add_systems(
        OnEnter(SimulationState::Ready),
        on_game_init.run_if(in_state(show_on_state)),
      )
  }
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Component)]
struct LogText;

#[derive(Component)]
struct PressSpace;

#[derive(Resource)]
struct SplashNextState<T>(T);

#[derive(Resource, Default)]
struct SplashState {
  pub loaded_handles: Option<Handle<LoadedFolder>>,
  pub preload_complete: bool,
  pub game_initialized: bool,
}

#[derive(Event)]
pub struct SplashLog(String);
impl From<&str> for SplashLog {
  fn from(value: &str) -> Self {
    Self(value.to_owned())
  }
}

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let icon = asset_server.load("preload/TriangleApocalypse.png");
  let mut cam = Camera2dBundle {
    camera: Camera {
      hdr: true, // 1. HDR is required for bloom
      ..default()
    },
    tonemapping: Tonemapping::TonyMcMapface, /* 2. Using a tonemapper that desaturates to white
                                              *    is recommended */
    ..default()
  };
  cam.projection.scaling_mode = ScalingMode::FixedVertical(1200.0);
  commands.spawn((cam, BloomSettings::default(), OnSplashScreen));
  commands
    .spawn((
      NodeBundle {
        style: Style {
          align_items: AlignItems::Start,
          justify_content: JustifyContent::End,
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          display: Display::Flex,
          flex_direction: FlexDirection::Column,
          ..default()
        },
        ..default()
      },
      OnSplashScreen,
    ))
    .with_children(|p| {
      p.spawn(TextBundle::default()).insert(LogText);
    });

  commands
    .spawn((
      NodeBundle {
        style: Style {
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          display: Display::Flex,
          flex_direction: FlexDirection::Column,
          ..default()
        },
        ..default()
      },
      OnSplashScreen,
    ))
    .with_children(|parent| {
      parent
        .spawn(NodeBundle {
          style: Style {
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(100.0),
            ..default()
          },
          ..default()
        })
        .with_children(|parent2| {
          parent2.spawn(ImageBundle {
            style: Style {
              width: Val::Px(800.0),
              ..default()
            },
            image: UiImage::new(icon),
            ..default()
          });
        });
      parent
        .spawn(
          TextBundle::from_section(
            "",
            TextStyle {
              font_size: 30.0,
              color: MISTY,
              ..default()
            },
          )
          .with_style(Style {
            margin: UiRect::top(Val::Px(50.0)),
            ..default()
          }),
        )
        .insert(PressSpace);
    });
}

fn preload_assets(
  mut log: EventWriter<SplashLog>,
  mut state: ResMut<SplashState>,
  asset_server: Res<AssetServer>,
) {
  state.loaded_handles = Some(asset_server.load_folder("preload"));
  log.send("Preloading assets...".into());
}

fn init_game(
  mut log: EventWriter<SplashLog>,
  mut mod_mgr: ResMut<ModManager>,
  mut cmds: EventWriter<GameControlCommand>,
) {
  // hard code the base game
  mod_mgr.clear().register(base_game::get_module());
  // initialize modules
  cmds.send(GameControlCommand::Initialize);
  log.send("Initializing game modules...".into());
}

fn on_game_init(
  mut log: EventWriter<SplashLog>,
  mut splash_state: ResMut<SplashState>,
  qry: Query<Entity, With<PressSpace>>,
  mut cmd: Commands,
) {
  splash_state.game_initialized = true;
  log.send("Initializing game modules...ok".into());

  cmd.entity(qry.single()).insert(TextAnimation {
    text: "Press space to play".to_owned(),
    animation_speed: 1.0,
  });
}

fn show_logs(mut log: EventReader<SplashLog>, mut qry: Query<&mut Text, With<LogText>>) {
  if let Ok(mut target) = qry.get_single_mut() {
    for l in log.read() {
      if let Some(m) = target
        .sections
        .iter_mut()
        .find(|t| l.0.starts_with(&t.value[1..]))
      {
        m.value = format!("\n{}", l.0);
      } else {
        target.sections.push(TextSection {
          value: format!("\n{}", l.0),
          style: TextStyle {
            color: MISTY,
            ..default()
          },
          ..default()
        })
      }
    }
  }
}

fn wait_for_preload_assets(
  mut log: EventWriter<SplashLog>,
  mut splash_state: ResMut<SplashState>,
  mut events: EventReader<AssetEvent<LoadedFolder>>,
  mut cmds: EventWriter<MusicCommand>,
) {
  if splash_state.preload_complete {
    return;
  }
  for event in events.read() {
    match event {
      AssetEvent::LoadedWithDependencies { id } => {
        if let Some(handle) = &splash_state.loaded_handles {
          if id == &Into::<AssetId<LoadedFolder>>::into(handle) {
            splash_state.preload_complete = true;
            cmds.send(MusicCommand::Play(BgMusic::Menu));
            log.send("Preloading assets...ok".into());
          }
        }
      }
      _ => {}
    }
  }
}

fn go_to_next_state<T: States>(
  mut app_state: ResMut<NextState<T>>,
  splash_state: Res<SplashState>,
  next_state: Res<SplashNextState<T>>,
  keyboard_input: Res<Input<KeyCode>>,
  mut cmds: EventWriter<GameControlCommand>,
) {
  if keyboard_input.just_pressed(KeyCode::Space)
    && splash_state.preload_complete
    && splash_state.game_initialized
  {
    cmds.send(GameControlCommand::StartGame);
    app_state.set(next_state.0.clone());
  }
}

pub fn build_bg(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<SplashMaterial>>,
) {
  let frame_size = Vec2::new(5000., 5000.);
  let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

  let v_pos = vec![
    [-frame_size.x, -frame_size.y, 0.0],
    [-frame_size.x, frame_size.y, 0.0],
    [frame_size.x, frame_size.y, 0.0],
    [frame_size.x, -frame_size.y, 0.0],
  ];
  mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

  let uvs: Vec<[f32; 2]> = vec![[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]];

  mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

  let indices = vec![0, 2, 1, 2, 0, 3];
  mesh.set_indices(Some(Indices::U32(indices)));

  // commands
  //   .spawn(MaterialMesh2dBundle {
  //     mesh: Mesh2dHandle(meshes.add(mesh)),
  //     material: materials.add(SplashMaterial {}),
  //     transform: Transform::from_translation(Vec3::new(0.0, 0.0, -100.)),
  //     ..default()
  //   })
  //   .insert(OnSplashScreen);
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct SplashMaterial {}

impl Material2d for SplashMaterial {
  fn fragment_shader() -> ShaderRef {
    "preload/splash.wgsl".into()
  }
  fn vertex_shader() -> ShaderRef {
    "preload/splash.wgsl".into()
  }

  fn specialize(
    descriptor: &mut RenderPipelineDescriptor,
    layout: &MeshVertexBufferLayout,
    _key: Material2dKey<Self>,
  ) -> Result<(), SpecializedMeshPipelineError> {
    let vertex_layout = layout.get_layout(&[
      Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
      Mesh::ATTRIBUTE_UV_0.at_shader_location(1),
    ])?;
    descriptor.vertex.buffers = vec![vertex_layout];
    Ok(())
  }
}
