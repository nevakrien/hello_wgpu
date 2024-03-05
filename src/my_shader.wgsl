// Vertex shader

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>, // Pass color to fragment shader
};

@vertex
fn vs_main(
    @builtin(vertex_index) in_vertex_index: u32,
) -> VertexOutput {
    var out: VertexOutput;
    var x = f32(1 - i32(in_vertex_index)) * 0.5;
    var y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);

    x=(cos(x * 3.14159) + 1.0) * 0.5;
    y=(sin(y * 3.14159) + 1.0) * 0.5;

    //out.color=vec4<f32>(x*x, y*y, (x+y)/f32(2), 1.0);
    out.color=vec4<f32>(x, y, (x+y)/f32(2), 1.0);
    //let red_component =  f32(abs(x) > 0.499) ;
    //out.color = vec4<f32>(red_component, 1.0 - red_component, 0.5, 1.0);
    return out;
}

// Fragment shader
@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}
