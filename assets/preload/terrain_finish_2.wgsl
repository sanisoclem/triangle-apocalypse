#define_import_path smud::terrain_finish_2

#import smud

fn sdf(p_in: vec2<f32>) -> f32 {
  return smud::sd_box(p_in, vec2<f32>(2000., 1000.));
}