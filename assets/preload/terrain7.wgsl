
#define_import_path smud::terrain

#import smud

fn track(p_in: vec2<f32>, w: f32, h: f32) -> f32 {
  let wh = w/2.0;
  let a = 3.0 * wh;
  let angle = atan(3. * (w/2.)/h);
  let h2 = w/(2.0* tan(angle));
  let inner = smud::sd_box(p_in + vec2<f32>(w/4.0, 0.0), vec2<f32>(w/4.0,h2));

  let t1 = smud::sd_triangle(p_in, vec2<f32>(-w/2.0,h2), vec2<f32>(-w/2.0,-h2), vec2<f32>(-w,0.));
  let t2 = smud::sd_triangle(p_in, vec2<f32>(-w/2.0,-h2), vec2<f32>(0.0,-h2), vec2<f32>(wh,-h));
  let t3 = smud::sd_triangle(p_in, vec2<f32>(w, -h), vec2<f32>(0.0,-h2), vec2<f32>(wh,-h));
  let t4 = smud::sd_triangle(p_in, vec2<f32>(-w/2.0,h2), vec2<f32>(0.0,h2), vec2<f32>(wh,h));
  let t5 = smud::sd_triangle(p_in, vec2<f32>(w, h), vec2<f32>(0.0,h2), vec2<f32>(wh,h));

  let up = smud::op_union(t4, t5);
  let mid = smud::op_union(inner, t1);
  let down = smud::op_union(t2, t3);

  return smud::op_union(smud::op_union(up,down), inner);
}

fn maze(p_in: vec2<f32>, p: f32) -> f32 {
  let h = p * 20.;
  let w = p * 20.;

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

  return
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
}

fn sdf(p_in: vec2<f32>) -> f32 {
  let p = 200.;
  let w = p * 20.;
  let mh = p * 20.;
  let th = 10000.0;
  let h = th + mh + mh;
  let border = 3000.;

  let outer = smud::sd_box(p_in, vec2<f32>(w + border, h + border));
  let flipped_m_coords = p_in - vec2<f32>(0.0, th + mh / 2.0);

  let m1 = maze(vec2<f32>(p_in.x * -1.0, (p_in.y * -1.0) + (th + mh / 1.0)), p);
  let m2 = maze(p_in - vec2<f32>(0.0, -th - (mh / 1.0)), p);
  let t1 = track(p_in, w, th);

  let inner = smud::op_union(smud::op_union(m1,m2), t1);
  return smud::op_subtract(inner, outer);
}