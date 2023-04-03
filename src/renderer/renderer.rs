use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use bgfx_rs::bgfx;
use bgfx_rs::bgfx::{ClearFlags, Init, PlatformData, ResetArgs, ResetFlags, SetViewClearArgs};
use glam::{Mat4, Vec3};
use log::{error, info, log, trace};
use raw_window_handle::RawWindowHandle;
use crate::scene::scene::Scene;

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
            width, height, fov, near, far
        }
    }

}

pub struct RenderView {
    pub eye: Vec3,
    pub at: Vec3,
    pub up: Vec3
}

impl RenderView {

    // constructor
    pub fn new(eye: Vec3, at: Vec3, up: Vec3) -> Self {
        Self {
            eye, at, up
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

    fn init(&mut self);
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
    view: Arc<Mutex<RenderView>>
}

impl BgfxRenderer {

    // constructor
    pub fn new(width: u32, height: u32, surface: Rc<RefCell<RawWindowHandle>>, debug: bool, perspective: RenderPerspective, view: RenderView) -> Self {
        Self {
            resolution: RenderResolution::new(width, height),
            old_resolution: RenderResolution::new(0, 0),
            surface,
            debug: Arc::new(Mutex::new(debug)),
            scene: None,
            debug_data: None,
            perspective: Arc::new(Mutex::new(perspective)),
            view: Arc::new(Mutex::new(view))
        }
    }

}

impl Renderer for BgfxRenderer {

    fn init(&mut self) {

        info!("Initializing BgfxRenderer");

        let mut init = Init::new();
        init.type_r = bgfx::RendererType::Count;
        init.resolution.width = self.width;
        init.resolution.height = self.height;
        init.resolution.reset = bgfx::ResetFlags::VSYNC.bits();

        let mut platform_data = PlatformData::new();

        // get platform data from raw windows handle

        match self.surface.borrow() {
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
        bgfx::init(&init);
    }

    fn do_render_cycle(&mut self) {

        let mut debug = self.debug.lock().expect("Failed to lock debug mutex");
        let mut perspective = self.perspective.lock().expect("Failed to lock perspective mutex");
        let mut view = self.view.lock().expect("Failed to lock view mutex");

        if !self.resolution.eq(&self.old_resolution) {
            self.old_resolution.from(&self.resolution);
            bgfx::reset(self.resolution.width, self.resolution.height, ResetArgs::default());
        }

        if *debug {
            bgfx::set_debug(bgfx::DebugFlags::TEXT.bits());
        } else {
            bgfx::set_debug(bgfx::DebugFlags::NONE.bits());
        }

        bgfx::set_view_clear(
            0,
            ClearFlags::COLOR.bits() | ClearFlags::DEPTH.bits(),
            SetViewClearArgs {
                rgba: 0x103030ff,
                ..Default::default()
            },
        );

        bgfx::set_view_rect(0, 0, 0, self.width as u16, self.height as u16);
        bgfx::touch(0);

        let mut view_matrix = Mat4::look_at(view.eye, view.at, view.up);
        let mut proj_matrix = Mat4::perspective(perspective.fov, perspective.width as f32 / perspective.height as f32, perspective.near, perspective.far);
        let mut view_proj_matrix = proj_matrix * view_matrix;

        bgfx::set_view_transform(0, view_matrix.as_ptr(), proj_matrix.as_ptr());
        bgfx::set_view_transform(1, view_matrix.as_ptr(), proj_matrix.as_ptr());

        if self.scene.is_none() {
            error!("Scene is not initialized");
            return;
        }

        let binding = self.scene.clone().unwrap();
        let scene_guard = binding.lock().expect("Failed to lock scene mutex");
        let scene = scene_guard.clone();

        let mut scene_guard = scene.borrow_mut();
        scene_guard.render(&view_proj_matrix);

    }

    fn shutdown(&mut self) {
        info!("Shutting down BgfxRenderer");
    }

    fn set_scene(&mut self, scene: Rc<RefCell<Scene>>) {

        if self.scene.is_none() {
            error!("Scene is not initialized");
            return;
        }

        let binding = self.scene.clone().unwrap();

        let mut scene_guard = binding.lock().expect("Failed to lock scene mutex");
        *scene_guard = scene;

    }

    fn set_debug_data(&mut self, data: TextDebugData) {

        let mut debug = self.debug.lock().expect("Failed to lock debug mutex");
        *debug = debug_data;

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

