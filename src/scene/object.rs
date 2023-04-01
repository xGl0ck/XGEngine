use bgfx_rs::bgfx::Texture;
use glam::Vec3;
use image::DynamicImage;
use uuid::Uuid;

pub struct ColoredVertex {
    pub coordinates: Vec3,
    pub color_rgba: u32
}

pub struct ImageTexturedVertex {
    pub coordinates: Vec3,
    pub texture_u: i16,
    pub texture_v: i16
}

pub struct TgaTexturedVertex {
    pub coordinates: Vec3,
    pub normal_rgba: u32,
    pub tangent: u32,
    pub texture_u: i16,
    pub texture_v: i16
}

pub enum ObjectTypes {
    Colored,
    ImageTextured,
    TgaTextured
}

pub struct Shaders {
    vertex: Vec<u8>,
    pixel: Vec<u8>
}


pub trait SceneObject {
    fn get_type(&self) -> ObjectTypes;
    fn as_any(&self) -> &dyn std::any::Any;
}

pub struct ColoredSceneObject {
    pub vertices: Vec<ColoredVertex>,
    pub indices: Vec<u16>
}

pub struct ImageTexturedSceneObject {
    pub vertices: Vec<ImageTexturedVertex>,
    pub indices: Vec<u16>,
    pub texture: DynamicImage
}

pub struct TgaTexturedSceneObject {
    pub vertices: Vec<TgaTexturedVertex>,
    pub indices: Vec<u16>,
    pub texture_color: DynamicImage,
    pub texture_normal: DynamicImage
}

// Implementations of new() with parameters for all SceneObject implementations
impl ColoredSceneObject {
    pub fn new(vertices: Vec<ColoredVertex>, indices: Vec<u16>) -> Self {
        Self {
            vertices, indices
        }
    }
}

impl ImageTexturedSceneObject {
    pub fn new(vertices: Vec<ImageTexturedVertex>, indices: Vec<u16>, texture: DynamicImage) -> Self {
        Self {
            vertices, indices, texture
        }
    }
}

impl TgaTexturedSceneObject {
    pub fn new(vertices: Vec<TgaTexturedVertex>, indices: Vec<u16>, texture_color: DynamicImage, texture_normal: DynamicImage) -> Self {
        Self {
            vertices, indices, texture_color, texture_normal
        }
    }
}

// SceneObject implementation for ColoredSceneObject
impl SceneObject for ColoredSceneObject {

    fn get_type(&self) -> ObjectTypes {
        ObjectTypes::Colored
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

// SceneObject implementation for ImageTexturedSceneObject
impl SceneObject for ImageTexturedSceneObject {

    fn get_type(&self) -> ObjectTypes {
        ObjectTypes::ImageTextured
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

// SceneObject implementation for TgaTexturedSceneObject
impl SceneObject for TgaTexturedSceneObject {

    fn get_type(&self) -> ObjectTypes {
        ObjectTypes::TgaTextured
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

}

#[cfg(test)]
mod tests {
    use std::any::Any;
    use super::*;

    // as_any() test for all SceneObject implementations
    #[test]
    fn as_any() {
        let colored_object = ColoredSceneObject {
            vertices: Vec::new(),
            indices: Vec::new()
        };

        let image_textured_object = ImageTexturedSceneObject {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture: DynamicImage::new_rgb8(50, 50)
        };

        let tga_textured_object = TgaTexturedSceneObject {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture_color: DynamicImage::new_rgb8(50, 50),
            texture_normal: DynamicImage::new_rgb8(50, 50)
        };

        assert!(colored_object.as_any().is::<ColoredSceneObject>());
        assert!(image_textured_object.as_any().is::<ImageTexturedSceneObject>());
        assert!(tga_textured_object.as_any().is::<TgaTexturedSceneObject>());

        // cast test

        let colored_object_any = colored_object.as_any();
        let colored_object_casted = colored_object_any.downcast_ref::<ColoredSceneObject>().unwrap();
        assert_eq!(colored_object_casted.type_id(), colored_object.type_id());

        let image_textured_object_any = image_textured_object.as_any();
        let image_textured_object_casted = image_textured_object_any.downcast_ref::<ImageTexturedSceneObject>().unwrap();
        assert_eq!(image_textured_object_casted.type_id(), image_textured_object.type_id());

        let tga_textured_object_any = tga_textured_object.as_any();
        let tga_textured_object_casted = tga_textured_object_any.downcast_ref::<TgaTexturedSceneObject>().unwrap();
        assert_eq!(tga_textured_object_casted.type_id(), tga_textured_object.type_id());

    }
}