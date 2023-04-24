use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use bgfx_rs::bgfx;
use bgfx_rs::bgfx::{AddArgs, Attrib, AttribType, BufferFlags, ClearFlags, Init, Memory, PlatformData, Program, ResetArgs, ResetFlags, SetViewClearArgs, StateCullFlags, StateDepthTestFlags, StateWriteFlags, SubmitArgs, VertexLayoutBuilder};
use bgfx_rs::bgfx::RendererType::{Count, Metal};
use glam::{Mat4, Vec3};
use log::{error, info, log, trace};
use raw_window_handle::RawWindowHandle;
use crate::scene::object::{ColoredSceneObject, ObjectTypes};
use crate::scene::scene::Scene;
use crate::shader::{BgfxShaderContainer, ShaderContainer};

pub struct DebugLine {
    key: String,
    value: String
}

impl DebugLine {

    // constructor
    pub fn new(key: String, value: String) -> Self {
        Self {
            key, value
        }
    }

}

pub struct TextDebugData {
    lines: Vec<DebugLine>
}

impl TextDebugData {

    // constructor
    pub fn new() -> Self {
        Self {
            lines: Vec::new()
        }
    }

    pub fn add_line(&mut self, line: DebugLine) {
        self.lines.push(line);
    }

}

pub struct RenderPerspective {
    pub width: u32,
    pub height: u32,
    pub fov: f32,
    pub near: f32,
    pub far: f32
}

impl RenderPerspective {

    // constructor
    pub fn new(width: u32, height: u32, fov: f32, near: f32, far: f32) -> Self {
        Self {
            width,
            height,
            fov: fov * (std::f32::consts::PI / 180.0),
            near,
            far
        }
    }

}

pub struct RenderView {
    pub eye: Vec3,
    pub at: Vec3,
    pub up: Vec3
}

pub enum MoveDirection {
    FORWARD, BACKWARDS, LEFT, RIGHT
}

impl RenderView {

    // constructor
    pub fn new(eye: Vec3, at: Vec3, up: Vec3) -> Self {
        Self {
            eye, at, up
        }
    }

    pub fn set_eye(&mut self, eye: Vec3) {
        self.eye = eye;
    }

    pub fn set_at(&mut self, at: Vec3) {
        self.at = at;
    }

    pub fn set_up(&mut self, up: Vec3) {
        self.up = up;
    }

    // calculates normal direction from at and eye
    pub fn get_normal(&self) -> Vec3 {
        (self.at - self.eye).normalize()
    }

    // moves eye in normal direction
    pub fn move_eye(&mut self, distance: f32, direction: MoveDirection) {

        match direction {
            MoveDirection::FORWARD => self.eye += self.get_normal() * distance,
            MoveDirection::BACKWARDS => self.eye -= self.get_normal() * distance,

            // move eye left or right
            MoveDirection::LEFT => {
                let normal = self.get_normal();
                let left = Vec3::new(normal.z, normal.y, -normal.x);
                self.eye += left * distance;
            },

            MoveDirection::RIGHT => {
                let normal = self.get_normal();
                let right = Vec3::new(-normal.z, normal.y, normal.x);
                self.eye += right * distance;
            },
        }

    }

}

pub struct RenderResolution {
    pub width: u32,
    pub height: u32
}

impl RenderResolution {

    // constructor
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width, height
        }
    }

    fn update(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }

    fn from(&mut self, other: &Self) {
        self.width = other.width.clone();
        self.height = other.height.clone();
    }

}

impl PartialEq<Self> for RenderResolution {

    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}

impl Eq for RenderResolution {}


pub trait Renderer {

    // initializes all resources required for rendering
    fn init(&mut self);

    // do one cycle which does all action in native render framework required for object render
    fn do_render_cycle(&mut self);

    fn shutdown(&mut self);
    fn set_scene(&mut self, scene: Rc<RefCell<Scene>>);
    fn set_debug_data(&mut self, data: TextDebugData);
    fn do_debug(&mut self, debug: bool);
    fn clean_up(&mut self);
    fn update_surface_resolution(&mut self, width: u32, height: u32);
    fn update_perspective(&mut self, perspective: RenderPerspective);

}

pub struct BgfxRenderer {
    resolution: RenderResolution,
    old_resolution: RenderResolution,
    surface: Rc<RefCell<RawWindowHandle>>,
    debug: Arc<Mutex<bool>>,
    scene: Option<Arc<Mutex<Rc<RefCell<Scene>>>>>,
    debug_data: Option<TextDebugData>,
    perspective: Arc<Mutex<RenderPerspective>>,
    shaders: HashMap<ObjectTypes, Program>
}

impl BgfxRenderer {

    // constructor
    pub fn new(width: u32, height: u32, surface: Rc<RefCell<RawWindowHandle>>, debug: bool, perspective: RenderPerspective) -> Self {
        Self {
            resolution: RenderResolution::new(width, height),
            old_resolution: RenderResolution::new(0, 0),
            surface,
            debug: Arc::new(Mutex::new(debug)),
            scene: None,
            debug_data: None,
            perspective: Arc::new(Mutex::new(perspective)),
            shaders: HashMap::new()
        }
    }

}

impl Renderer for BgfxRenderer {

    fn init(&mut self) {

        info!("Initializing BgfxRenderer");

        let mut init = Init::new();
        init.type_r = Count;
        init.resolution.width = self.resolution.width;
        init.resolution.height = self.resolution.height;
        init.resolution.reset = ResetFlags::NONE.bits();

        let mut platform_data = PlatformData::new();

        // get platform data from raw windows handle

        match self.surface.borrow().deref() {
            RawWindowHandle::Win32(handle) => {
                platform_data.nwh = handle.hwnd
            },
            RawWindowHandle::AppKit(handle) => {
                platform_data.nwh = handle.ns_window
            },
            RawWindowHandle::Xlib(handle) => {
                platform_data.nwh = handle.window as *mut std::ffi::c_void;
            },
            RawWindowHandle::Wayland(handle) => {
                platform_data.ndt = handle.surface
            },
            _ => {
                error!("Unsupported platform");
                return;
            }
        }

        init.platform_data = platform_data;

        if !bgfx::init(&init) {
            panic!("failed to init bgfx");
        }

        bgfx::set_debug(bgfx::DebugFlags::NONE.bits());
        self.clean_up();
    }

    fn do_render_cycle(&mut self) {

        let mut debug = self.debug.lock().expect("Failed to lock debug mutex");
        let mut perspective = self.perspective.lock().expect("Failed to lock perspective mutex");

        if !self.resolution.eq(&self.old_resolution) {
            self.old_resolution.from(&self.resolution);
            bgfx::reset(self.resolution.width, self.resolution.height, ResetArgs::default());
        }

        bgfx::dbg_text_clear(bgfx::DbgTextClearArgs::default());
        bgfx::set_view_rect(0, 0, 0, self.resolution.width.clone() as u16, self.resolution.height.clone() as u16);

        if self.scene.is_none() {
            error!("Scene is not initialized");
            return;
        }

        let scene = match &self.scene {
            Some(scene) => scene,
            None => {
                error!("Scene is not initialized");
                return;
            }
        };

        let scene_guard = scene.lock().expect("Failed to lock scene mutex");

        let scene_reference = scene_guard.borrow();

        let mut view_matrix = Mat4::look_at_lh(scene_reference.camera.eye.clone(), scene_reference.camera.at.clone(), scene_reference.camera.up.clone());
        let mut proj_matrix = Mat4::perspective_lh(perspective.fov, perspective.width as f32 / perspective.height as f32, perspective.near, perspective.far);

        bgfx::set_view_transform(0, &view_matrix.to_cols_array(), &proj_matrix.to_cols_array());

        let chunk = match scene_reference.get_current_chunk() {
            Ok(chunk) => chunk,
            Err(e) => {
                error!("Failed to get current chunk: {}", e);
                return;
            }
        };

        for object in chunk.objects.borrow_mut().iter_mut() {

            match object.get_type() {

                ObjectTypes::Colored => {

                    let mut colored = object.as_any_mut().downcast_mut::<ColoredSceneObject>().unwrap();

                    let vertex_buffer = unsafe {

                        let layout = VertexLayoutBuilder::new();

                        layout
                            .begin(Metal)
                            .add(Attrib::Position, 3, AttribType::Float, AddArgs::default())
                            .add(Attrib::Color0, 4, AttribType::Uint8, AddArgs { normalized: true, as_int: false })
                            .end();

                        let memory = Memory::reference(&colored.vertices);
                        bgfx::create_vertex_buffer(&memory, &layout, BufferFlags::empty().bits())
                    };

                    let index_buffer = unsafe {
                        let memory = Memory::reference(&colored.indices);
                        bgfx::create_index_buffer(&memory, BufferFlags::empty().bits())
                    };

                    let state = (StateWriteFlags::R
                        | StateWriteFlags::G
                        | StateWriteFlags::B
                        | StateWriteFlags::A
                        | StateWriteFlags::Z)
                        .bits()
                        | StateDepthTestFlags::LESS.bits()
                        | StateCullFlags::CW.bits();

                    let transform = Mat4::from_translation(colored.coordinates.clone());

                    bgfx::set_transform(&transform.to_cols_array(), 1);
                    bgfx::set_vertex_buffer(0, &vertex_buffer, 0, std::u32::MAX);
                    bgfx::set_index_buffer(&index_buffer, 0, std::u32::MAX);

                    bgfx::set_state(state, 0);

                    let mut shaders_reference = Rc::clone(&colored.shaders);

                    let mut shaders_deref = shaders_reference.deref().borrow_mut();

                    let shaders = shaders_deref.as_any_mut().downcast_mut::<BgfxShaderContainer>().unwrap();

                    if !shaders.loaded() {
                        shaders.load();
                    }

                    let program = Rc::clone(&shaders.program.clone().unwrap());

                    bgfx::submit(0, program.as_ref(), SubmitArgs::default());
                }

                _ => {}

            }

        }

        if *debug {

            let debug_data = self.debug_data.as_ref().unwrap();

            for i in 0..debug_data.lines.len() {
                let line = debug_data.lines.get(i).unwrap();

                bgfx::dbg_text(0, i as u16, 0x0f, format!("{}: {}", line.key, line.value).as_str());

            }

        }

        bgfx::touch(0);
        bgfx::frame(false);

    }

    fn shutdown(&mut self) {
        info!("Shutting down BgfxRenderer");
        bgfx::shutdown();
    }

    fn set_scene(&mut self, scene: Rc<RefCell<Scene>>) {

        if self.scene.is_none() {

            self.scene = Some(Arc::new(Mutex::new(Rc::clone(&scene))));
            return;
        }

        let binding = self.scene.clone().unwrap();

        let mut scene_guard = binding.lock().expect("Failed to lock scene mutex");
        *scene_guard = scene;

    }

    fn set_debug_data(&mut self, data: TextDebugData) {

        self.debug_data = Some(data);
    }

    fn do_debug(&mut self, debug: bool) {

        let mut debug_guard = self.debug.lock().expect("Failed to lock debug mutex");
        *debug_guard = debug;

        if debug {
            info!("Debugging enabled");
            bgfx::set_debug(bgfx::DebugFlags::TEXT.bits());
        } else {
            info!("Debugging disabled");
            bgfx::set_debug(bgfx::DebugFlags::NONE.bits());
        }

    }

    fn clean_up(&mut self) {
        info!("Cleaning up BgfxRenderer");
        bgfx::set_view_clear(
            0,
            ClearFlags::COLOR.bits() | ClearFlags::DEPTH.bits(),
            SetViewClearArgs {
                rgba: 0x103030ff,
                ..Default::default()
            },
        );
    }

    fn update_surface_resolution(&mut self, width: u32, height: u32) {
        self.old_resolution.from(&self.resolution);
        self.resolution.update(width, height);
    }

    fn update_perspective(&mut self, perspective: RenderPerspective) {

        let mut perspective_guard = self.perspective.lock().expect("Failed to lock perspective mutex");
        *perspective_guard = perspective;

    }
}

