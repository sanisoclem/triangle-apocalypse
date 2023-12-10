#define_import_path smud::terrain

#import smud

fn sdf(p_in: vec2<f32>) -> f32 {
  let h = 10000.;
  let w = 2000.;
  let x = 500.;
  let outer = smud::sd_box(p_in, vec2<f32>(w + 3000., h + 3000.));
  let inner = smud::sd_box(p_in, vec2<f32>(x, h));
  let t1 = smud::sd_triangle(p_in, vec2<f32>(x,h), vec2<f32>(w,0.), vec2<f32>(x,-h));
  let t2 = smud::sd_triangle(p_in, vec2<f32>(-x,h), vec2<f32>(-w,0.), vec2<f32>(-x,-h));

  let ts = smud::op_union(t1,t2);

  return smud::op_smooth_subtract(smud::op_union(ts, inner), outer, 1000.1);
}
