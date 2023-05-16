@fragment
fn fragment(
    @builtin(position) coord: vec4<f32>,
    @location(0) world_position: vec4<f32>,
    @location(1) normals: vec3<f32>,
    @location(2) uv: vec2<f32>
    ) -> @location(0) vec4<f32> {
    return world_position;
    // return vec4<f32>(normals.x, normals.y, normals.z, 0.0);
    // return vec4<f32>(uv.x, uv.y, 0.0, 0.0);
}