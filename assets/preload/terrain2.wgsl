#define_import_path smud::terrain

#import smud

fn sdf(p_in: vec2<f32>) -> f32 {
  let h = 10000.;
  let w = 2000.;
  let wh = w/2.0;
  let a = 3.0 * wh;
  let angle = atan(3. * (w/2.)/h);
  let h2 = w/(2.0* tan(angle));
  let border = 3000.;
  let outer = smud::sd_box(p_in, vec2<f32>(w + border, h + border));
  let inner = smud::sd_box(p_in + vec2<f32>(w/4.0, 0.0), vec2<f32>(w/4.0,h2));

  let t1 = smud::sd_triangle(p_in, vec2<f32>(-w/2.0,h2), vec2<f32>(-w/2.0,-h2), vec2<f32>(-w,0.));
  let t2 = smud::sd_triangle(p_in, vec2<f32>(-w/2.0,-h2), vec2<f32>(0.0,-h2), vec2<f32>(wh,-h));
  let t3 = smud::sd_triangle(p_in, vec2<f32>(w, -h), vec2<f32>(0.0,-h2), vec2<f32>(wh,-h));
  let t4 = smud::sd_triangle(p_in, vec2<f32>(-w/2.0,h2), vec2<f32>(0.0,h2), vec2<f32>(wh,h));
  let t5 = smud::sd_triangle(p_in, vec2<f32>(w, h), vec2<f32>(0.0,h2), vec2<f32>(wh,h));

  let up = smud::op_union(t4, t5);
  let mid = smud::op_union(inner, t1);
  let down = smud::op_union(t2, t3);

  return smud::op_subtract(smud::op_union(smud::op_union(up,down), inner), outer);
}