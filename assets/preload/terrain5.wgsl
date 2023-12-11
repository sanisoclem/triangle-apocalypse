#define_import_path smud::terrain

#import smud

fn sdf(p_in: vec2<f32>) -> f32 {
  let p = 150.;
  let h = p * 20.;
  let w = p * 20.;
  let border = 3000.;
  let outer = smud::sd_box(p_in, vec2<f32>(w + border, h + border));

  let b1 = smud::sd_box(p_in - vec2<f32>(p * 15., p * -14.0), vec2<f32>(p * 5., p * 6.));
  let b2 = smud::sd_box(p_in - vec2<f32>(p * 15., p * -6.0), vec2<f32>(p * 3., p * 2.0));
  let b3 = smud::sd_box(p_in - vec2<f32>(p * 2.0 , p * -10.0), vec2<f32>(p * 8., p * 2.0));
  let b4 = smud::sd_box(p_in - vec2<f32>(p * 2.0 , p * -18.0), vec2<f32>(p * 8., p * 2.0));
  let b5 = smud::sd_box(p_in - vec2<f32>(p * -4.0 , p * -14.0), vec2<f32>(p * 2., p * 2.0));
  let b6 = smud::sd_box(p_in - vec2<f32>(p * 8.0, p * -5.0), vec2<f32>(p * 2., p * 3.0));
  let b7 = smud::sd_box(p_in - vec2<f32>(p * 0.0, p * 0.0), vec2<f32>(p * 20., p * 2.0));
  let b8 = smud::sd_box(p_in - vec2<f32>(p * -12.0, p * -11.0), vec2<f32>(p * 2., p * 9.0));
  let b9 = smud::sd_box(p_in - vec2<f32>(p * -18.0, p * -11.0), vec2<f32>(p * 2., p * 9.0));
  let b10 = smud::sd_box(p_in - vec2<f32>(p * -15.0, p * -18.0), vec2<f32>(p * 1., p * 2.0));
  let b11 = smud::sd_box(p_in - vec2<f32>(p * -16.0, p * 8.0), vec2<f32>(p * 4., p * 2.0));
  let b12 = smud::sd_box(p_in - vec2<f32>(p * -16.0, p * 4.0), vec2<f32>(p * 2., p * 2.0));
  let b13 = smud::sd_box(p_in - vec2<f32>(p * -14.0, p * 12.0), vec2<f32>(p * 2., p * 2.0));

  let b14 = smud::sd_box(p_in - vec2<f32>(p * -8., p * 5.0), vec2<f32>(p * 2., p * 3.0));
  let b15 = smud::sd_box(p_in - vec2<f32>(p * -4.0, p * 10.0), vec2<f32>(p * 6., p * 2.0));
  let b16 = smud::sd_box(p_in - vec2<f32>(p * 3.0, p * 6.0), vec2<f32>(p * 5., p * 2.0));
  let b17 = smud::sd_box(p_in - vec2<f32>(p * 6.0, p * 11.0), vec2<f32>(p * 2., p * 3.0));

  let b18 = smud::sd_box(p_in - vec2<f32>(p * 18.0, p * 8.0), vec2<f32>(p * 2., p * 6.0));
  let b19 = smud::sd_box(p_in - vec2<f32>(p * 12.0, p * 6.0), vec2<f32>(p * 2., p * 4.0));
  let b20 = smud::sd_box(p_in - vec2<f32>(p * 15.0, p * 8.0), vec2<f32>(p * 1., p * 2.0));

  let b21 = smud::sd_box(p_in - vec2<f32>(p * 0.0, p * 17.0), vec2<f32>(p * 20., p * 3.0));

  let inner =
    smud::op_union(b2,
    smud::op_union(b3,
    smud::op_union(b4,
    smud::op_union(b5,
    smud::op_union(b6,
    smud::op_union(b7,
    smud::op_union(b8,
    smud::op_union(b9,
    smud::op_union(b10,
    smud::op_union(b11,
    smud::op_union(b12,
    smud::op_union(b13,
    smud::op_union(b14,
    smud::op_union(b15,
    smud::op_union(b16,
    smud::op_union(b17,
    smud::op_union(b18,
    smud::op_union(b19,
    smud::op_union(b20,
    smud::op_union(b21,
      b1))))))))))))))))))));

  let out = smud::op_subtract(inner, outer);


  return out;
}