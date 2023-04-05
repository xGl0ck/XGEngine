use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use bgfx_rs::bgfx;
use bgfx_rs::bgfx::{Memory, Program, Shader};

pub trait ShaderContainer {

    fn loaded(&self) -> bool;
    fn load(&mut self);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

}

pub struct BgfxShaderContainer {
    loaded: bool,
    pixel_raw: Vec<u8>,
    vertex_raw: Vec<u8>,
    pixel_mem: Option<Memory>,
    vertex_mem: Option<Memory>,
    pixel: Option<Shader>,
    vertex: Option<Shader>,
    pub program: Option<Rc<Program>>
}

impl BgfxShaderContainer {

    pub fn new(pixel_raw: Vec<u8>, vertex_raw: Vec<u8>) -> Self {

        Self {
            loaded: false,
            pixel_raw,
            vertex_raw,
            pixel_mem: None,
            vertex_mem: None,
            pixel: None,
            vertex: None,
            program: None
        }
    }

}

impl ShaderContainer for BgfxShaderContainer {

    fn loaded(&self) -> bool {
        self.loaded
    }

    fn load(&mut self) {

        self.pixel_mem = Option::from(unsafe { Memory::reference(&self.pixel_raw) });
        self.vertex_mem = Option::from(unsafe { Memory::reference(&self.vertex_raw) });

        // create shader with bgfx
        self.pixel = Option::from(unsafe { bgfx::create_shader(&self.pixel_mem.unwrap()) });
        self.vertex = Option::from(unsafe { bgfx::create_shader(&self.vertex_mem.unwrap()) });

        // create program with bgfx
        self.program = Some(Rc::new(unsafe { bgfx::create_program(&self.vertex.clone().unwrap(), &self.pixel.clone().unwrap(), true) }));

    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct ShaderManager {
    pub shaders: HashMap<i32, Rc<RefCell<Box<dyn ShaderContainer>>>>
}

impl ShaderManager {

    pub fn new() -> Self {
        Self {
            shaders: HashMap::new()
        }
    }

    pub fn add_shader(&mut self, shader: Box<dyn ShaderContainer>) -> i32 {
        let index: i32 = self.shaders.len() as i32;
        self.shaders.insert(index, Rc::new(RefCell::new(shader)));
        index
    }

    pub fn get_shader(&self, index: i32) -> Option<Rc<RefCell<Box<dyn ShaderContainer>>>> {
        match self.shaders.get(&index) {
            Some(shader) => Some(Rc::clone(shader)),
            None => None
        }
    }

}