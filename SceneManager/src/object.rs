use bgfx_rs::bgfx::Texture;

pub struct PosVertex {
    x: f32,
    y: f32,
    z: f32,
    normal_rgba: u32,
    tangent: u32,
    texture_u: i16,
    texture_v: i16
}

pub struct SceneObject {
    vertices: Vec<PosVertex>,
    indices: Vec<u16>,
    texture_color: Texture,
    texture_normal: Texture
}

impl SceneObject {

    pub fn new(vertices: Vec<PosVertex>, indices: Vec<u16>, texture_color: Texture, texture_normal: Texture) -> Self {
        Self {
            vertices, indices, texture_color, texture_normal
        }
    }

}