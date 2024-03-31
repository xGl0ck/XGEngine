use crate::shader::ShaderContainer;
use bgfx_rs::bgfx::Texture;
use glam::Vec3;
use image::DynamicImage;
use std::any::Any;
use std::cell::RefCell;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use uuid::Uuid;

pub struct ColoredVertex {
    pub coordinates: Vec3,
    pub color_rgba: u32,
}

pub struct ImageTexturedVertex {
    pub coordinates: Vec3,
    pub texture_u: i16,
    pub texture_v: i16,
}

pub struct TgaTexturedVertex {
    pub coordinates: Vec3,
    pub normal_rgba: u32,
    pub tangent: u32,
    pub texture_u: i16,
    pub texture_v: i16,
}

pub enum ObjectTypes {
    Colored,
    ImageTextured,
    TgaTextured,
}

pub struct Shaders {
    vertex: Vec<u8>,
    pixel: Vec<u8>,
}

pub trait SceneObject {
    fn get_type(&self) -> ObjectTypes;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct ColoredSceneObject {
    pub vertices: Box<[ColoredVertex]>,
    pub indices: Box<[u16]>,
    pub shaders: Rc<RefCell<Box<dyn ShaderContainer>>>,
    pub coordinates: Vec3,
}

pub struct ImageTexturedSceneObject {
    pub vertices: Box<[ImageTexturedVertex]>,
    pub indices: Box<[u16]>,
    pub texture: DynamicImage,
    pub shaders: Rc<RefCell<Box<dyn ShaderContainer>>>,
    pub coordinates: Vec3,
}

pub struct TgaTexturedSceneObject {
    pub vertices: Box<[TgaTexturedVertex]>,
    pub indices: Box<[u16]>,
    pub texture_color: DynamicImage,
    pub texture_normal: DynamicImage,
    pub shaders: Rc<RefCell<Box<dyn ShaderContainer>>>,
    pub coordinates: Vec3,
}

// Implementations of new() with parameters for all SceneObject implementations
impl ColoredSceneObject {
    pub fn new(
        vertices: Box<[ColoredVertex]>,
        indices: Box<[u16]>,
        shaders: Rc<RefCell<Box<dyn ShaderContainer>>>,
        coordinates: Vec3,
    ) -> Self {
        Self {
            vertices,
            indices,
            shaders,
            coordinates,
        }
    }
}

impl ImageTexturedSceneObject {
    pub fn new(
        vertices: Box<[ImageTexturedVertex]>,
        indices: Box<[u16]>,
        texture: DynamicImage,
        shaders: Rc<RefCell<Box<dyn ShaderContainer>>>,
        coordinates: Vec3,
    ) -> Self {
        Self {
            vertices,
            indices,
            texture,
            shaders,
            coordinates,
        }
    }
}

impl TgaTexturedSceneObject {
    pub fn new(
        vertices: Box<[TgaTexturedVertex]>,
        indices: Box<[u16]>,
        texture_color: DynamicImage,
        texture_normal: DynamicImage,
        shaders: Rc<RefCell<Box<dyn ShaderContainer>>>,
        coordinates: Vec3,
    ) -> Self {
        Self {
            vertices,
            indices,
            texture_color,
            texture_normal,
            shaders,
            coordinates,
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

    fn as_any_mut(&mut self) -> &mut dyn Any {
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

    fn as_any_mut(&mut self) -> &mut dyn Any {
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

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct TestShaderContainer {}

impl ShaderContainer for TestShaderContainer {
    fn loaded(&self) -> bool {
        false
    }

    fn load(&mut self) {
        println!("TestShaderContainer::load()");
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glfw::Key::V;
    use std::any::Any;

    // as_any() test for all SceneObject implementations
    #[test]
    fn as_any() {
        let colored_object = ColoredSceneObject {
            vertices: Box::new([]),
            indices: Box::new([]),
            shaders: Rc::new(RefCell::new(Box::new(TestShaderContainer {}))),
            coordinates: Vec3::new(0.0, 0.0, 0.0),
        };

        let image_textured_object = ImageTexturedSceneObject {
            vertices: Box::new([]),
            indices: Box::new([]),
            texture: DynamicImage::new_rgb8(50, 50),
            shaders: Rc::new(RefCell::new(Box::new(TestShaderContainer {}))),
            coordinates: Vec3::new(0.0, 0.0, 0.0),
        };

        let tga_textured_object = TgaTexturedSceneObject {
            vertices: Box::new([]),
            indices: Box::new([]),
            texture_color: DynamicImage::new_rgb8(50, 50),
            texture_normal: DynamicImage::new_rgb8(50, 50),
            shaders: Rc::new(RefCell::new(Box::new(TestShaderContainer {}))),
            coordinates: Vec3::new(0.0, 0.0, 0.0),
        };

        assert!(colored_object.as_any().is::<ColoredSceneObject>());
        assert!(image_textured_object
            .as_any()
            .is::<ImageTexturedSceneObject>());
        assert!(tga_textured_object.as_any().is::<TgaTexturedSceneObject>());

        // cast test

        let colored_object_any = colored_object.as_any();
        let colored_object_casted = colored_object_any
            .downcast_ref::<ColoredSceneObject>()
            .unwrap();
        assert_eq!(colored_object_casted.type_id(), colored_object.type_id());

        let image_textured_object_any = image_textured_object.as_any();
        let image_textured_object_casted = image_textured_object_any
            .downcast_ref::<ImageTexturedSceneObject>()
            .unwrap();
        assert_eq!(
            image_textured_object_casted.type_id(),
            image_textured_object.type_id()
        );

        let tga_textured_object_any = tga_textured_object.as_any();
        let tga_textured_object_casted = tga_textured_object_any
            .downcast_ref::<TgaTexturedSceneObject>()
            .unwrap();
        assert_eq!(
            tga_textured_object_casted.type_id(),
            tga_textured_object.type_id()
        );
    }
}
