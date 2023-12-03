use bevy::{asset::LoadedFolder, prelude::*};
use jam4::{GameControlCommand, Loading, ModManager, SimulationState};
use utils::despawn_screen;

pub trait SplashExtensions {
  fn add_splash_screen<T: States + Copy>(&mut self, show_on_state: T, next_state: T) -> &mut Self;
}

impl SplashExtensions for App {
  fn add_splash_screen<T: States + Copy>(&mut self, show_on_state: T, next_state: T) -> &mut Self {
    self
      .init_resource::<SplashState>()
      .insert_resource(SplashNextState(next_state))
      .add_systems(
        OnEnter(show_on_state),
        (preload_assets, (splash_setup, init_game)).chain(),
      )
      .add_systems(
        Update,
        (mark_loaded, countdown, go_to_next_state::<T>).run_if(in_state(show_on_state)),
      )
      .add_systems(OnExit(show_on_state), despawn_screen::<OnSplashScreen>)
      // skip menus, create a new game as soon as simulation is ready
      .add_systems(OnEnter(SimulationState::Ready), new_game)
      .add_systems(OnEnter(SimulationState::Simulating), mark_started)
  }
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

#[derive(Resource)]
struct SplashNextState<T>(T);

#[derive(Resource, Default)]
struct SplashState {
  pub loaded_handles: Option<Handle<LoadedFolder>>,
  pub preload_complete: bool,
  pub game_started: bool,
}

fn preload_assets(mut state: ResMut<SplashState>, asset_server: Res<AssetServer>) {
  state.loaded_handles = Some(asset_server.load_folder("preload"));
}

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let icon = asset_server.load("splash.png");
  commands.spawn((Camera2dBundle::default(), OnSplashScreen));
  commands
    .spawn((
      NodeBundle {
        style: Style {
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          ..default()
        },
        ..default()
      },
      OnSplashScreen,
    ))
    .with_children(|parent| {
      parent.spawn(ImageBundle {
        style: Style {
          width: Val::Px(200.0),
          ..default()
        },
        image: UiImage::new(icon),
        ..default()
      });
      parent.spawn(
        TextBundle::from_section(
          "Loading...",
          TextStyle {
            font_size: 80.0,
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
          },
        )
        .with_style(Style {
          margin: UiRect::all(Val::Px(50.0)),
          ..default()
        }),
      );
    });

  // spawn an entity that indicate it is still loading
  // we can signal the game the the splash screen is done
  // so simulation starts at the same time as the splash screen decides to disappear
  commands.spawn(OnSplashScreen).insert(Loading);

  commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
}

fn mark_loaded(
  mut splash_state: ResMut<SplashState>,
  mut events: EventReader<AssetEvent<LoadedFolder>>,
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
          }
        }
      }
      _ => {}
    }
  }
}

fn init_game(mut mod_mgr: ResMut<ModManager>, mut cmds: EventWriter<GameControlCommand>) {
  // hard code the base game
  mod_mgr.clear().register(base_game::get_module());
  // initialize modules
  cmds.send(GameControlCommand::Initialize);
}

fn new_game(mut cmds: EventWriter<GameControlCommand>) {
  cmds.send(GameControlCommand::NewGame);
}

fn mark_started(mut splash_state: ResMut<SplashState>) {
  splash_state.game_started = true;
}

fn countdown(
  mut cmd: Commands,
  qry: Query<Entity, (With<Loading>, With<OnSplashScreen>)>,
  mut timer: ResMut<SplashTimer>,
  time: Res<Time>,
) {
  if timer.tick(time.delta()).finished() {
    if let Ok(e) = qry.get_single() {
      cmd.entity(e).despawn();
    }
  }
}

fn go_to_next_state<T: States>(
  mut app_state: ResMut<NextState<T>>,
  splash_state: Res<SplashState>,
  next_state: Res<SplashNextState<T>>,
) {
  if splash_state.preload_complete && splash_state.game_started {
    app_state.set(next_state.0.clone());
  }
}
