#define_import_path smud::terrain

#import smud

fn sdf(p_in: vec2<f32>) -> f32 {
  let h = 10000.;
  let w = 2000.;
  let outer = smud::sd_box(p_in, vec2<f32>(w + 3000., h + 3000.));
  let inner = smud::sd_box(p_in, vec2<f32>(w, h));
  let a = 3.0 * w/2.0;
  let h2 = w/(2.0* tan(acos(h/sqrt((a * a) + (h * h) ))));

  let t1 = smud::sd_triangle(p_in, vec2<f32>(-w/2.0,0.), vec2<f32>(w,h), vec2<f32>(w,-h));
  let t2 = smud::sd_triangle(p_in, vec2<f32>(-w,0.0), vec2<f32>(-w,-h), vec2<f32>(w/2.0,-h));
  let t3 = smud::sd_triangle(p_in, vec2<f32>(-w,0.0), vec2<f32>(-w,h), vec2<f32>(w/2.0,h));
  let t4 = smud::sd_box(p_in + vec2<f32>(w/2.0, 0.0), vec2<f32>(w/4.0,h2/2.));

  let ts = smud::op_union(smud::op_union(smud::op_subtract(t4, t1),t2),t3);

  return smud::op_union(smud::op_subtract(inner, outer), ts);
}