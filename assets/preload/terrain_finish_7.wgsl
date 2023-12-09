#define_import_path smud::terrain_finish

#import smud

fn sdf(p_in: vec2<f32>) -> f32 {
  return smud::sd_box(p_in, vec2<f32>(3000., 1000.));
}