use bevy::{asset::LoadedFolder, prelude::*};
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
      .init_resource::<SplashState>()
      .insert_resource(SplashNextState(next_state))
      .add_event::<SplashLog>()
      .add_systems(
        OnEnter(show_on_state),
        (preload_assets, (splash_setup, init_game)).chain(),
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

fn splash_setup(mut commands: Commands, _asset_server: Res<AssetServer>) {
  // let icon = asset_server.load("splash.png");
  commands.spawn((Camera2dBundle::default(), OnSplashScreen));
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
        background_color: BackgroundColor(RAISIN.with_a(0.8)),
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
        background_color: BackgroundColor(RAISIN.with_a(0.8)),
        ..default()
      },
      OnSplashScreen,
    ))
    .with_children(|parent| {
      parent.spawn(
        TextBundle::from_section(
          "Triangle Apocalypse",
          TextStyle {
            font_size: 80.0,
            color: LILAC,
            ..default()
          },
        )
        .with_style(Style {
          margin: UiRect::all(Val::Px(50.0)),
          ..default()
        }),
      );
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
    text: "Press space to continue".to_owned(),
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
