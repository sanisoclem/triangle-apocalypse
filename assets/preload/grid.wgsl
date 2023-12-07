// Import the standard 2d mesh uniforms and set their bind groups
#import bevy_sprite::{mesh2d_functions, mesh2d_view_bindings::globals}

// The structure of the vertex buffer is as specified in `specialize()`
struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) uv: vec2<f32>
};

struct VertexOutput {
    // The vertex shader must set the on-screen position of the vertex
    @builtin(position) position: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) uv: vec2<f32>,
    @location(2) color: vec4<f32>,
};

fn pristine_grid(uv: vec2f, w: f32, vs: f32) -> f32 {
    var lineWidth: vec2f = vec2<f32>(w, w);
    var ddx: vec2f = dpdx(uv);
    var ddy: vec2f = dpdy(uv);
    var uvDeriv: vec2f = vec2(length(vec2(ddx.x, ddy.x)), length(vec2(ddx.y, ddy.y)));
    let invertLine: vec2<bool> = vec2<bool>(lineWidth.x > 0.5, lineWidth.y > 0.5);
    var targetWidth: vec2<f32>;
    if invertLine.x {
        targetWidth.x = 1.0 - lineWidth.x;
    } else {
        targetWidth.x = lineWidth.x;
    };
    if invertLine.y {
        targetWidth.y = 1.0 - lineWidth.y;
    } else {
        targetWidth.y = lineWidth.y;
    };
    let drawWidth: vec2f = clamp(targetWidth, uvDeriv, vec2(0.5));
    let lineAA: vec2f = uvDeriv * 1.5;
    var gridUV: vec2f = abs(fract(uv) * 2.0 - 1.0);
    if invertLine.x { gridUV.x = gridUV.x; } else { gridUV.x = 1.0 - gridUV.x; };
    if invertLine.y { gridUV.y = gridUV.y; } else { gridUV.y = 1.0 - gridUV.y; };
    var grid2: vec2f = smoothstep(drawWidth + lineAA, drawWidth - lineAA, gridUV);

    grid2 *= clamp(targetWidth / drawWidth, vec2(0.0), vec2(1.0));
    grid2 = mix(grid2, targetWidth, clamp(uvDeriv * 2.0 - 1.0, vec2(0.0), vec2(1.0)));
    if invertLine.x {
        grid2.x = 1.0 - grid2.x;
    };// else { grid2.x = grid2.x };
    if invertLine.y {
        grid2.y = 1.0 - grid2.y;
    }; // else { grid2.y = grid2.y };
    return mix(grid2.x, 1.0, grid2.y * vs);
}


/// Entry point for the vertex shader
@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    out.uv = vertex.uv;
    // Project the world position of the mesh into screen position
    let model = mesh2d_functions::get_model_matrix(vertex.instance_index);

    out.world_position = mesh2d_functions::mesh2d_position_local_to_world(
        model,
        vec4<f32>(vertex.position, 1.0)
    );
    out.position = mesh2d_functions::mesh2d_position_world_to_clip(out.world_position);
    out.color = vec4<f32>(5.89, 5.729, 5.776, 1.0);
    return out;
}

fn oklab_to_linear_srgb(c: vec3<f32>) -> vec3<f32> {
    let L = c.x;
    let a = c.y;
    let b = c.z;

    let l_ = L + 0.3963377774 * a + 0.2158037573 * b;
    let m_ = L - 0.1055613458 * a - 0.0638541728 * b;
    let s_ = L - 0.0894841775 * a - 1.2914855480 * b;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    return vec3<f32>(
        4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
    );
}

/// Entry point for the fragment shader
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let speed = 1.0;
    var rand = fract(in.position.x * 123.4112 + in.position.y *42.3185 + in.position.z * 4213.3212);

    let t_1 = sin(globals.time * speed ) * 0.5 + 0.5;
    let t_2 = cos(globals.time * speed);
    let t_3 = cos(globals.time * speed / 10.0);

    //let d_1 = fract(distance(in.uv, vec2<f32>(0.5)) *1.4 * 100.);
    let distance_to_center = distance(in.uv, vec2<f32>(0.5)) *1.4;

    // blending is done in a perceptual color space: https://bottosson.github.io/posts/oklab/
    let red = vec3<f32>(10.0, 0.0, 0.0);
    let green = vec3<f32>(0.0, 10.0, 0.0);
    let orange = vec3<f32>(20.0, 1.0, 0.0);
    let blue = vec3<f32>(0.0, 0.0, 10.0);
    let white = vec3<f32>(1.0, 1.0, 1.0);
    // let mixed = mix(mix(mix(red, blue, t_1), mix(green, white, t_2), t_3), green, distance_to_center);
    let mixed = green * t_1; //white * t_1;

    let warp_mode = 1.0; //t_1;

    let small_grid = pristine_grid(in.uv * 750. * 2. * warp_mode,  0.01 , 1.0);
    let big_grid = pristine_grid(in.uv * 750.* .25 * warp_mode,  0.01 , 1.0);
    let faint_grid = pristine_grid(in.uv * 750.* 2. * warp_mode,  0.001 , 1.0)* blue * 0.2;
    let vertical_lines = pristine_grid(in.uv * 750.* warp_mode * .125,  0.003, 0.0);


    var x1 = (cos(in.world_position.x / 1200.) *sin(in.world_position.x / 1500.)); //sin(floor((in.world_position.x) /100.));
    var y1 = pow(fract((in.world_position.y + (x1 * 10000.) - (globals.time * 1000.)) /10000.), 10.);
    var y2 = pow( fract((in.world_position.y - (globals.time * 1000.)) /10000.),20.);


    let color = max(max(faint_grid, vertical_lines * orange * y1), max(small_grid, big_grid) * y2 * blue* 0.5);
    // let color = vec3<f32>(y1);

    //return vec4<f32>(mix(g1,g2,1.0), 1.0);
    // return vec4<f32>(vec3<f32>(grid * y1) * mix(blue,green,t_2) , 1.);
    // return vec4<f32>(vec3<f32>(x1) * blue, 1.);
    return vec4<f32>(color, 1.);
}