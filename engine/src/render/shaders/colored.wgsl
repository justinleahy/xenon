struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) translation: vec2<f32>,
    @location(2) scale: vec2<f32>,
    @location(3) color: vec4<f32>,
    @location(4) uv: vec2<f32>,
}

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
    @location(1) uv: vec2<f32>,
}

@group(0) @binding(0)
var sprite_texture: texture_2d<f32>;

@group(0) @binding(1)
var sprite_sampler: sampler;

@vertex
fn vs_main(input: VertexInput) -> VertexOutput {
    var output: VertexOutput;

    let position = input.position * input.scale + input.translation;

    output.position = vec4<f32>(position, 0.0, 1.0);
    output.color = input.color;
    output.uv = input.uv;

    return output;
}

@fragment
fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
    let sampled = textureSample(sprite_texture, sprite_sampler, input.uv);
    return sampled * input.color;
}
