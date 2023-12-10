#define_import_path smud::terrain

#import smud

fn sdf(p_in: vec2<f32>) -> f32 {
  let h = 10000.;
  let w = 1000.;
  let outer = smud::sd_box(p_in, vec2<f32>(w + 3000., h + 3000.));
  let inner = smud::sd_box(p_in, vec2<f32>(w, h));
  let outer_box = smud::op_subtract(inner, outer);

  return outer_box;
}