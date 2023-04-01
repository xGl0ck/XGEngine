use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use bgfx_rs::bgfx;
use bgfx_rs::bgfx::{Init, PlatformData, ResetArgs};
use glam::Vec3;
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

pub struct DebugData {
    lines: Vec<DebugLine>
}

impl DebugData {

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

pub trait Renderer {

    fn init(&mut self);
    fn do_render_cycle(&mut self);
    fn shutdown(&mut self);
    fn set_scene(&mut self, scene: Rc<RefCell<Scene>>);
    fn set_debug_data(&mut self, debug_data: bool, data: DebugData);
    fn clean_up(&mut self);
    fn update_surface_resolution(&mut self, width: u32, height: u32);
    fn update_perspective(&mut self, perspective: RenderPerspective);

}

pub struct BgfxRenderer {
    width: u32,
    height: u32,
    old_size: (i32, i32),
    surface: RawWindowHandle,
    debug: Arc<Mutex<bool>>,
    scene: Option<Arc<Mutex<Rc<RefCell<Scene>>>>>,
    debug_data: Option<DebugData>,
    perspective: Arc<Mutex<RenderPerspective>>,
    view: Arc<Mutex<RenderView>>
}

impl BgfxRenderer {

    // constructor
    pub fn new(width: u32, height: u32, surface: RawWindowHandle, debug: bool, perspective: RenderPerspective, view: RenderView) -> Self {
        Self {
            width,
            height,
            surface,
            debug: Arc::new(Mutex::new(debug)),
            scene: None,
            debug_data: None,
            old_size: (0, 0),
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
        match self.surface {
            RawWindowHandle::Win32(handle) => {
                platform_data.nwh = handle.hwnd as *mut std::ffi::c_void;
            },
            RawWindowHandle::AppKit(handle) => {
                platform_data.nwh = handle.ns_view as *mut std::ffi::c_void;
            },
            RawWindowHandle::Xlib(handle) => {
                platform_data.nwh = handle.window as *mut std::ffi::c_void;
            },
            RawWindowHandle::Wayland(handle) => {
                platform_data.nwh = handle.surface as *mut std::ffi::c_void;
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

        info!("Rendering BgfxRenderer");



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

    fn set_debug_data(&mut self, debug_data: bool, data: DebugData) {

        let binding = self.scene.clone().unwrap();

        let mut debug = self.debug.lock().expect("Failed to lock debug mutex");
        *debug = debug_data;

        self.debug_data = Some(data);
    }

    fn clean_up(&mut self) {
        info!("Cleaning up BgfxRenderer");
    }

    fn update_surface_resolution(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        bgfx::reset(self.width as _, self.height as _, ResetArgs::default());
    }

    fn update_perspective(&mut self, perspective: RenderPerspective) {

        let mut perspective_guard = self.perspective.lock().expect("Failed to lock perspective mutex");
        *perspective_guard = perspective;

    }
}

