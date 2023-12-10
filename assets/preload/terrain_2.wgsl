#define_import_path smud::terrain

#import smud

fn sdf(p_in: vec2<f32>) -> f32 {
  let h = 10000.;
  let w = 2000.;
  let outer = smud::sd_box(p_in, vec2<f32>(w + 3000., h + 3000.));
  let inner = smud::sd_box(p_in, vec2<f32>(w, h));
  let t1 = smud::sd_triangle(p_in, vec2<f32>(w,0.), vec2<f32>(w,-h), vec2<f32>(500.,-h));
  let t2 = smud::sd_triangle(p_in, vec2<f32>(-w,0.), vec2<f32>(-w,-h), vec2<f32>(-500.,-h));
  let t3 = smud::sd_triangle(p_in, vec2<f32>(w,0.), vec2<f32>(w,h), vec2<f32>(500.,h));
  let t4 = smud::sd_triangle(p_in, vec2<f32>(-w,0.), vec2<f32>(-w,h), vec2<f32>(-500.,h));

  let down = smud::op_union(t1,t2);
  let up = smud::op_union(t3,t4);
  let ts = smud::op_union(up,down);

  return smud::op_union(smud::op_subtract(inner, outer), ts);
}