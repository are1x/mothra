struct VertexInput {
    @location(0) position: vec2<f32>
};

struct Uniforms {
    scale: vec2<f32>
};

@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(input: VertexInput) -> @builtin(position) vec4<f32> {
    // スケーリング後、-1.0 を引いてクリップ空間に変換
    let pos = input.position * uniforms.scale - vec2<f32>(1.0, 1.0);
    return vec4<f32>(pos, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(0.2, 0.8, 1.0, 1.0);
}
