#import bevy_pbr::forward_io::VertexOutput
#import "shaders/simplex_noise_3d.wgsl"::simplexNoise3

@group(1) @binding(0) var<uniform> color: vec4<f32>;
@group(1) @binding(1) var<uniform> time: f32;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    var border_width = 0.2;
    var c1 = vec4<f32>(0.0, 0.1, 0.5, 0.0);
    var c2 = vec4<f32>(0.0, 0.6, .7, 1.0);
    //var c2 = vec4<f32>(1.0);
    var n = simplexNoise3(vec3<f32>(mesh.world_position.xy * 1.0, time * 0.5));
    //n = (n / 2.0) + 0.5;
    //n = min(n, 1.0 - n);
    n = smoothstep(-0.4, -0.0, n) * smoothstep(-0.4, -0.0, -n);

    var border_up = smoothstep(border_width, 0.0, mesh.uv.y);
    var border_down = smoothstep(1.0 - border_width, 1.0, mesh.uv.y);
    var border = max(border_up, border_down);

    n = max(border, n);
    var c = mix(c1, c2, n);
    // return color;
    return vec4<f32>(c);
//    return vec4<f32>(t, t, t, 1.0);
}
