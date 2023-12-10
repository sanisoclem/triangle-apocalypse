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

/// Entry point for the fragment shader
@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let speed = 3.0;
    let grid_mult = 150.;
    let y_wave_size = 2500.; // multiplier of grid wave size to make them meet
    let y_wave_speed = 1300.; // match grid wave speed
    let edge_size = 1.;

    let t_1 = sin(globals.time * speed ) * 0.5 + 0.5;
    let t_2 = cos(globals.time * speed)  * 0.5 + 0.5;

    let y1 = pow(fract((in.world_position.y - (globals.time * y_wave_speed)) /y_wave_size),4.);

    let red = vec3<f32>(20.0, 0.0, 0.0);
    let blue = vec3<f32>(0.0, 0.0, 20.0);
    let mixed = mix(blue, red, y1 );

    let scaled_uv = in.uv * grid_mult;

    let bg_grid = pristine_grid(scaled_uv,  0.2, 1.0);
    let f_grid = pristine_grid(scaled_uv,  0.2, 1.0);

    var is_edge = 0.0;
    if scaled_uv.y <= edge_size {
        is_edge = 1.0;
    } else {
        is_edge = 0.0;
    };

    let edge_alpha = is_edge * (1. - pow(fract(( globals.time * y_wave_speed) /y_wave_size),4.));

    let bg_squares = vec4<f32>(mixed * (1. - bg_grid), max(y1, edge_alpha));
    return bg_squares;
}