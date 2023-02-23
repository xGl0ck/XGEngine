use bgfx_rs::bgfx::Texture;
use glam::Vec3;
use uuid::Uuid;

pub struct PosVertex {
    pub coordinates: Vec3,
    pub normal_rgba: u32,
    pub tangent: u32,
    pub texture_u: i16,
    pub texture_v: i16
}

pub struct SceneObject {
    pub name: String,
    pub id: Uuid,
    pub vertices: Vec<PosVertex>,
    pub indices: Vec<u16>,
    pub texture_color: Texture,
    pub texture_normal: Texture
}

impl SceneObject {

    pub fn new(name: String, vertices: Vec<PosVertex>, indices: Vec<u16>, texture_color: Texture, texture_normal: Texture) -> Self {
        Self {
            name, id: Uuid::default() ,vertices, indices, texture_color, texture_normal
        }
    }

}