#define_import_path smud::bevy

#import smud

fn sdf(p_in: vec2<f32>) -> f32 {
  let outer = smud::sd_box(p_in, vec2<f32>(3000., 10000.));
  let inner = smud::sd_box(p_in, vec2<f32>(2000., 9000.));
  let m1 = smud::sd_circle(p_in - vec2<f32>(300.,200.), 200.);
  let m2 = smud::sd_circle(p_in - vec2<f32>(-200.,300.), 150.);
  let m3 = smud::sd_circle(p_in - vec2<f32>(300.,-300.), 125.);
  let m4 = smud::sd_circle(p_in - vec2<f32>(100.,1500.), 50.);

  let inner2 = max(max(max(max(inner, -m1), -m2), -m3), -m4);

  return max(outer, -inner2);
}