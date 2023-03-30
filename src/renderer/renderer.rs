use std::rc::Rc;
use std::sync::{Arc, Mutex};
use bgfx_rs::bgfx;
use bgfx_rs::bgfx::{Init, PlatformData};
use log::{error, log};
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

pub trait Renderer {

    fn init(&mut self);
    fn do_render_cycle(&mut self);
    fn shutdown(&mut self);
    fn set_scene(&mut self, scene: Rc<Arc<Mutex<Scene>>>);
    fn set_debug_data(&mut self, debug_data: bool, data: DebugData);
    fn clean_up(&mut self);

}

pub struct BgfxRenderer {
    width: u32,
    height: u32,
    surface: RawWindowHandle,
    debug: Arc<Mutex<bool>>,
    scene: Option<Rc<Arc<Mutex<Scene>>>>,
    debug_data: Option<DebugData>
}

impl BgfxRenderer {

    // constructor
    pub fn new(width: u32, height: u32, surface: RawWindowHandle, debug: bool) -> Self {
        Self {
            width, height, surface, debug: Arc::new(Mutex::new(debug)), scene: None, debug_data: None
        }
    }

}

impl Renderer for BgfxRenderer {

    fn init(&mut self) {

        log!("Initializing BgfxRenderer");

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
        log!("Rendering BgfxRenderer");

    }

    fn shutdown(&mut self) {
        log!("Shutting down BgfxRenderer");
    }

    fn set_scene(&mut self, scene: Rc<Arc<Mutex<Scene>>>) {
        self.scene = Some(scene);
    }

    fn set_debug_data(&mut self, debug_data: bool, data: DebugData) {

        let mut debug = self.debug.lock().expect("Failed to lock debug mutex");
        *debug = debug_data;

        self.debug_data = Some(data);
    }

    fn clean_up(&mut self) {
        log!("Cleaning up BgfxRenderer");
    }

}

