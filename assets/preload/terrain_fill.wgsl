#define_import_path smud::outline

#import smud


fn fill(d: f32, color: vec4<f32>) -> vec4<f32> {
    var a = smud::sd_fill_alpha_fwidth(d);

    if d < -1.0 {
      a = 1.0;
    } else {
      a = 0.0;
    }
    let blue = vec3<f32>(0.3, 0.0, 0.0);
    return vec4<f32>(blue * pow(clamp(1. / -d, 0.0, 1.),2.),  color.a * a);
}
