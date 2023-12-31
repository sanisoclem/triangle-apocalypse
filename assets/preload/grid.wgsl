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
    let speed = 1.0;

    let t_1 = sin(globals.time * speed ) * 0.5 + 0.5;
    let t_2 = cos(globals.time * speed);
    let t_3 = cos(globals.time * speed / 10.0);

    let distance_to_center = distance(in.uv, vec2<f32>(0.5)) *1.4;

    let red = vec3<f32>(10.0, 0.0, 0.0);
    let green = vec3<f32>(0.0, 10.0, 0.0);
    let orange = vec3<f32>(20.0, 0.0, 20.0);
    let blue = vec3<f32>(0.0, 0.0, 10.0);
    let white = vec3<f32>(1.0, 1.0, 1.0);
    let mixed = green * t_1; //white * t_1;

    let warp_mode = 1.0; //t_1;

    let small_grid = pristine_grid(in.uv * 750. * 2. * warp_mode,  0.01 , 1.0);
    let big_grid = pristine_grid(in.uv * 750.* .5 * warp_mode,  0.01 , 1.0);
    let faint_grid = pristine_grid(in.uv * 750.* 2. * warp_mode,  0.001 , 1.0)* blue * 0.2;
    let vertical_lines = pristine_grid(in.uv * 750.* warp_mode  *0.5,  0.003, 0.0);



    var x1 = -cos(in.world_position.x / 1000.) + (0.5* -cos(in.world_position.x / 200.) *cos(in.world_position.x / 130.) * cos(in.world_position.x / 70.) ); //sin(floor((in.world_position.x) /100.));
    var y1 = pow(fract((in.world_position.y + (x1 * 10000.) - (globals.time * 1000.)) /10000.), 20.);
    var y2 = pow(fract((in.world_position.y - (globals.time * 1300.)) /10000.),20.);


    var f = 0.0;
    if y2 >= 0.995 {
        f = 1.0;
    }

    let color = max(max(faint_grid, vertical_lines * orange * y1), max(f, max(small_grid, big_grid)) * y2 * blue* 0.5);
    return vec4<f32>(color, 1.);
}