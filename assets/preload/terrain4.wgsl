#define_import_path smud::terrain

#import smud

fn sdf(p_in: vec2<f32>) -> f32 {
  let w = 4000.;
  let h = w * 2.0 + 1000.;
  let wp = 500.;
  let border = 3000.;
  let hw = (h - 2.0 * w)/2. + 200. ;
  let outer = smud::sd_box(p_in, vec2<f32>(w + border, h + border));
  let inner = smud::sd_box(abs(p_in) + vec2<f32>(0., -h + hw), vec2<f32>(w, hw));
  let mid_box = smud::sd_box(p_in, vec2<f32>(wp, wp));

  let s1p = vec2<f32>(0.0, w);
  let s1a = smud::sd_circle(p_in + s1p, w);
  let s1b = smud::sd_circle(p_in + s1p, w - wp);
  let s1c = smud::sd_circle(p_in + s1p, w + wp);
  // let s1 = smud::op_union(smud::op_subtract(s1a,s1c), s1b);

  let s2p = vec2<f32>(0.0, -w);
  let s2a = smud::sd_circle(p_in + s2p, w);
  let s2b = smud::sd_circle(p_in + s2p, w - wp);
  let s2c = smud::sd_circle(p_in + s2p, w + wp);
  // let s2 = smud::op_union(smud::op_subtract(s2a,s2c), s2b);

  let sa = smud::op_union(s1a, s2a);
  let sb = smud::op_union(s1b, s2b);
  let sc = smud::op_union(s1c, s2c);

  let s = smud::op_union(smud::op_union(mid_box, smud::op_subtract(sb,sa)), inner);
  let s_back = smud::op_subtract(s, outer);


  return s_back;
}