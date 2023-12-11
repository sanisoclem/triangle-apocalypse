use bevy::{
  prelude::*,
  render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
  },
  sprite::{MaterialMesh2dBundle, Mesh2d},
};
use rodio::source::{from_iter, Source};
use spectrum_analyzer::{samples_fft_to_spectrum, scaling::divide_by_N_sqrt, FrequencyLimit};
use std::time::Duration;
use utils::{music::{FadeTo, ProcessedAudio, AUDIO_ANALYZER_SAMPLE_SIZE}, lerp};

pub trait JukeboxExtensions {
  fn add_jukebox(&mut self) -> &mut Self;
}

impl JukeboxExtensions for App {
  fn add_jukebox(&mut self) -> &mut Self {
    self
      .init_resource::<Jukebox>()
      .add_event::<MusicCommand>()
      .add_systems(Startup, setup_audio_texture)
      .add_systems(
        Update,
        (
          wait_for_jukebox_init,
          process_music_commands,
          read_audio_samples,
        )
          .chain(),
      )
  }
}

#[derive(Component, Copy, Clone, PartialEq, Debug)]
pub enum BgMusic {
  MainTheme,
  Menu,
  GameOver,
}

#[derive(Resource)]
pub struct Jukebox {
  main_theme_start: Handle<AudioSource>,
  main_theme_loop: Handle<AudioSource>,
  menu: Handle<AudioSource>,
  game_over: Handle<AudioSource>,
  init_result: Option<InitResult>,
}

pub struct InitResult {
  pub main_theme: Handle<ProcessedAudio>,
  pub game_over_theme: Handle<ProcessedAudio>,
  pub menu_theme: Handle<ProcessedAudio>,
}

#[derive(Event)]
pub enum MusicCommand {
  Play(BgMusic),
}

impl FromWorld for Jukebox {
  fn from_world(world: &mut World) -> Self {
    let asset_server = world.get_resource::<AssetServer>().unwrap();

    let main_theme_start = asset_server.load("preload/battle_0.ogg");
    let main_theme_loop = asset_server.load("preload/battle_1.ogg");
    let menu = asset_server.load("preload/menu_full.ogg");
    let game_over = asset_server.load("preload/menu_full.ogg");
    Self {
      main_theme_start,
      main_theme_loop,
      menu,
      game_over,
      init_result: None,
    }
  }
}

impl Jukebox {
  pub fn try_initialize(&mut self, asset_server: &AssetServer, audio: &Assets<AudioSource>) {
    let Some(start) = audio.get(self.main_theme_start.clone()) else {
      return;
    };
    let Some(battle_loop) = audio.get(self.main_theme_loop.clone()) else {
      return;
    };
    let Some(menu) = audio.get(self.menu.clone()) else {
      return;
    };
    let Some(game_over) = audio.get(self.game_over.clone()) else {
      return;
    };

    let main_theme = asset_server.add(ProcessedAudio::new(
      vec![start.clone(), battle_loop.clone()], // wow
      |sources| {
        let copy: Vec<_> = sources
          .iter()
          .enumerate()
          .map(|(i, f)| -> Box<dyn Source<Item = i16> + Send + Sync> {
            if i == 1 {
              Box::new(f.decoder().repeat_infinite())
            } else {
              Box::new(f.decoder())
            }
          })
          .collect();
        Box::new(from_iter(copy.into_iter()))
      },
    ));

    let menu_theme = asset_server.add(ProcessedAudio::new_sampled(
      vec![menu.clone()], // wow
      1.0,
      |sources| {
        Box::new(
          sources
            .first()
            .unwrap()
            .decoder()
            .repeat_infinite()
            .convert_samples()
            .delay(Duration::from_secs(1)),
        )
      },
    ));

    let game_over_theme = asset_server.add(ProcessedAudio::new(
      vec![game_over.clone()], // wow
      |sources| Box::new(sources.first().unwrap().decoder()),
    ));

    self.init_result = Some(InitResult {
      game_over_theme,
      main_theme,
      menu_theme,
    });
  }
}

pub fn process_music_commands(
  qry: Query<(Entity, &BgMusic), With<BgMusic>>,
  mut commands: Commands,
  mut cmds: EventReader<MusicCommand>,
  jukebox: Res<Jukebox>,
) {
  let Some(ir) = &jukebox.init_result else {
    return;
  };
  let mut started = false;
  for cmd in cmds.read() {
    if started {
      break;
    }
    let MusicCommand::Play(theme) = cmd;

    let src = match theme {
      BgMusic::MainTheme => &ir.main_theme,
      BgMusic::GameOver => &ir.game_over_theme,
      BgMusic::Menu => &ir.menu_theme,
    };

    // look for currently playing music that matches
    for (e, b) in qry.iter() {
      if b == theme {
        info!("found match {:?}", theme);
        commands
          .entity(e)
          .insert(FadeTo::fade_in(Duration::from_secs_f32(1.0)));
        started = true;
        continue;
      }
      info!("start despawn {:?} {:?}", b, e);
      commands
        .entity(e)
        .insert(FadeTo::fade_out_despawn(Duration::from_secs_f32(0.2)));
    }
    if !started {
      info!("spawning {:?}", theme);
      commands.spawn((
        AudioSourceBundle {
          source: src.clone(),
          settings: PlaybackSettings::LOOP,
        },
        *theme,
      ));
      started = true;
    }
  }
}
pub fn wait_for_jukebox_init(
  mut jukebox: ResMut<Jukebox>,
  asset_server: Res<AssetServer>,
  audio: Res<Assets<AudioSource>>,
) {
  if jukebox.init_result.is_some() {
    return;
  }
  jukebox.try_initialize(&asset_server, &audio);
}

const AUDIO_TEX_HEIGHT: usize = 200;
const AUDIO_TEX_WIDTH: usize = 1000;
const AUDIO_TEX_HANDLE: Handle<Image> = Handle::weak_from_u128(18187448111173546254);
const MAX_MEL: usize = 3250;

pub fn setup_audio_texture(
  mut commands: Commands,
  mut images: ResMut<Assets<Image>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ColorMaterial>>,
) {
  let size = Extent3d {
    height: AUDIO_TEX_HEIGHT as u32,
    width: AUDIO_TEX_WIDTH as u32,
    depth_or_array_layers: 1,
  };
  let mut image = Image {
    texture_descriptor: TextureDescriptor {
      label: None,
      size,
      dimension: TextureDimension::D2,
      format: TextureFormat::Rgba8Unorm, // here we set R-G-B-A
      mip_level_count: 1,
      sample_count: 1,
      usage: TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING,
      view_formats: &[],
    },
    ..default()
  };
  // seems that we have to call resize to allocate the memory
  image.resize(size);
  // make it all white for a start
  image.data.fill(0);

  images.insert(AUDIO_TEX_HANDLE, image);
  commands.spawn((SpriteBundle {
    texture: AUDIO_TEX_HANDLE,
    ..default()
  },));
  // commands.spawn(MaterialMesh2dBundle {
  //   mesh: meshes
  //     .add(Mesh::from(shape::Quad::new(Vec2::splat(500.))))
  //     .into(),
  //   material: materials.add(ColorMaterial {
  //     color: Color::rgba(1.0, 1.0, 1.0, 1.0),
  //     texture: Some(AUDIO_TEX_HANDLE),
  //   }),
  //   transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.)),
  //   ..default()
  // });
}

#[derive(Default)]
struct Stats {
  pub min: f32,
  pub max: f32,
  pub mean: f32,
  pub median: f32,
  pub ctr: usize,
}

fn read_audio_samples(
  qry: Query<&Handle<ProcessedAudio>>,
  mut audio: ResMut<Assets<ProcessedAudio>>,
  mut local_buf: Local<Vec<f32>>,
  mut stats: Local<Stats>,
  mut images: ResMut<Assets<Image>>,
  mut time: Res<Time>,
) {
  for hpa in qry.iter() {
    let Some(pa) = audio.get_mut(hpa) else {
      continue;
    };
    let Some(rx_mutex) = pa.sample_receiver.as_mut() else {
      continue;
    };
    let Ok(rx) = rx_mutex.get_mut() else {
      return;
    };
    while let Ok(data) = rx.try_recv() {
      let Some(img) = images.get_mut(AUDIO_TEX_HANDLE) else {
        return;
      };

      local_buf.clear();
      local_buf.extend(data.into_iter().map(|x| cpal::Sample::to_float_sample(x)));
      let windowed = spectrum_analyzer::windows::hann_window(&local_buf);

      // get data from audio source
      if let Ok(spec) = samples_fft_to_spectrum(
        &windowed,
        44100,
        FrequencyLimit::All,
        Some(&divide_by_N_sqrt),
      ) {
        let freq_intervals = MAX_MEL as f32 / AUDIO_TEX_WIDTH as f32;
        let (_, min) = spec.min();
        let (_, max) = spec.max();
        let mean = spec.average().val();
        let median = spec.median().val();

        // if  mean > 1.0 { //mean > stats.mean { //
        //   img.data.chunks_mut(4).enumerate().for_each(|(i, chunk)| {
        //     let x = i % AUDIO_TEX_WIDTH;
        //     let sample = lerp(chunk[0]as f32/255., 0., time.delta_seconds());
        //     chunk[0] = (sample * 255.) as u8;
        //     chunk[1] = (sample * 255.) as u8;
        //     chunk[2] = (sample * 255.) as u8;
        //     chunk[3] = 255;
        //   });
        //   return;
        //   info!("new mean {:?}", mean);
        //   stats.mean = mean;
        // }
        // if median > stats.median {
        //   info!("new median {:?}", median);
        //   stats.median = median;
        // }

        let r = max.val() - min.val();
        if stats.ctr >= AUDIO_TEX_HEIGHT {
          stats.ctr = 0;
        }
        img.data.chunks_mut(4).skip(stats.ctr * AUDIO_TEX_WIDTH).take(AUDIO_TEX_WIDTH).enumerate().for_each(|(i, chunk)| {
          let x = i % AUDIO_TEX_WIDTH;
          let sample = spec.mel_val(freq_intervals * x as f32).val();
          chunk[0] = (sample * 255.) as u8;
          chunk[1] = (sample * 255.) as u8;
          chunk[2] = (sample * 255.) as u8;
          chunk[3] = 255;
        });
        stats.ctr += 1;
      }
    }
  }
}

// fn generate_if_necessary(
//   settings: Res<Settings>,
//   noise_map: ResMut<LandscapeNoiseMap>,
//   mut images: ResMut<Assets<Image>>,
//   mut materials: ResMut<Assets<StandardMaterial>>,
// ) {

// //...

// if settings.is_changed() {
//   // ...

//   if let Some(mat) = materials.get_mut(noise_map.material_handle.id()) {
//       let old = (*mat).base_color_texture.clone().unwrap().id().clone();

//       (*mat).base_color_texture = Some(images.add(Image::new(
//           size,
//           TextureDimension::D2,
//           gen.get_noise_map_vec_rgba_8_u_norm(),
//           TextureFormat::Rgba8Unorm,
//       )));

//       images.remove(old);
//   }
// }
