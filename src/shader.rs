use bgfx_rs::bgfx;
use bgfx_rs::bgfx::{Memory, Program, Shader};
use std::any::Any;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub trait ShaderContainerLoadContext {
    fn as_any(&self) -> &dyn Any;
}

pub trait ShaderContainer {
    fn loaded(&self) -> bool;
    fn load(&mut self, context: Box<dyn ShaderContainerLoadContext>);
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
    pub program: Option<Rc<Program>>,
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
            program: None,
        }
    }
}

pub struct BgfxShaderContainerLoadContext {}

impl ShaderContainerLoadContext for BgfxShaderContainerLoadContext {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ShaderContainer for BgfxShaderContainer {
    fn loaded(&self) -> bool {
        self.loaded
    }

    fn load(&mut self, context: Box<dyn ShaderContainerLoadContext>) {
        self.pixel_mem = Option::from(unsafe { Memory::reference(&self.pixel_raw) });
        self.vertex_mem = Option::from(unsafe { Memory::reference(&self.vertex_raw) });

        // create shader with bgfx
        self.pixel = Option::from(unsafe { bgfx::create_shader(&self.pixel_mem.unwrap()) });
        self.vertex = Option::from(unsafe { bgfx::create_shader(&self.vertex_mem.unwrap()) });

        // create program with bgfx
        self.program = Some(Rc::new(unsafe {
            bgfx::create_program(
                &self.vertex.clone().unwrap(),
                &self.pixel.clone().unwrap(),
                true,
            )
        }));

        self.loaded = true;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub trait WgpuVertexLayout {
    fn desc(&self) -> wgpu::VertexBufferLayout<'static>;
}

pub struct WgpuShaderContainer {
    source_string: String,
    vertex_layout: Box<dyn WgpuVertexLayout>,
    shader_module: Option<wgpu::ShaderModule>,
    pipeline_layout: Option<wgpu::PipelineLayout>,
    render_pipeline: RefCell<Option<wgpu::RenderPipeline>>,
    texture_format: wgpu::TextureFormat,
    loaded: bool,
}

impl WgpuShaderContainer {
    pub fn new(
        source_string: String,
        layout: Box<dyn WgpuVertexLayout>,
        texture_format: wgpu::TextureFormat,
    ) -> Self {
        Self {
            source_string,
            vertex_layout: layout,
            shader_module: None,
            pipeline_layout: None,
            render_pipeline: RefCell::new(None),
            texture_format,
            loaded: false,
        }
    }

    pub fn get_pipeline_layout(&self) -> &RefCell<Option<wgpu::RenderPipeline>> {
        if !self.loaded {
            panic!("Shader not loaded");
        }

        &self.render_pipeline
    }
}

struct WgpuShaderLoadContext {
    device: Rc<wgpu::Device>,
}

impl ShaderContainerLoadContext for WgpuShaderLoadContext {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl ShaderContainer for WgpuShaderContainer {
    fn loaded(&self) -> bool {
        self.loaded
    }

    fn load(&mut self, context: Box<dyn ShaderContainerLoadContext>) {
        let shader_module = wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(self.source_string.clone().into()),
        };

        let context = context
            .as_any()
            .downcast_ref::<WgpuShaderLoadContext>()
            .unwrap();

        let device = context.device.clone();

        self.shader_module = Some(device.create_shader_module(shader_module));

        let pipeline_layout = wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        };

        self.pipeline_layout = Some(wgpu::Device::create_pipeline_layout(
            &device,
            &pipeline_layout,
        ));

        let texture_format = self.texture_format;

        let color_state = [Some(wgpu::ColorTargetState {
            format: texture_format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        })];

        let render_pipeline = wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&self.pipeline_layout.as_ref().unwrap()),
            vertex: wgpu::VertexState {
                module: &self.shader_module.as_ref().unwrap(),
                entry_point: "vs_main",
                buffers: &[self.vertex_layout.desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &self.shader_module.as_ref().unwrap(),
                entry_point: "fs_main",
                targets: &color_state,
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
                unclipped_depth: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        };

        *self.render_pipeline.borrow_mut() = Some(wgpu::Device::create_render_pipeline(
            &device,
            &render_pipeline,
        ));

        self.loaded = true;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

pub struct ShaderManager {
    pub shaders: HashMap<i32, Rc<RefCell<Box<dyn ShaderContainer>>>>,
}

impl ShaderManager {
    pub fn new() -> Self {
        Self {
            shaders: HashMap::new(),
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
            None => None,
        }
    }
}
