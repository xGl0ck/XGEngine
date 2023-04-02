use std::rc::Rc;
use event_bus::EventBus;
use glfw::{FAIL_ON_ERRORS, Glfw};
use glfw::Key::N;
use log::info;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use crate::renderer::renderer::{Renderer, RenderPerspective};
use crate::scene::manager::{ChangeSceneEvent, SceneManager};

mod core;
mod events;
mod environment;

mod messaging {
//    pub mod controller;
//    pub mod event;
//    pub mod message;
}

mod renderer {
    pub mod controller;
    pub mod renderer;
    pub mod events;
}

mod scene {
    pub mod chunk;
    pub mod manager;
    pub mod object;
    pub mod scene;
}


static mut RENDERER: Option<Box<dyn Renderer>> = None;

static mut ENGINE_ENVIRONMENT: Option<environment::EngineEnvironment> = None;


fn set_renderer(renderer: Box<dyn Renderer>) {

    unsafe {
        RENDERER = Some(renderer);
    }

}

fn change_scene_handler(event: &mut ChangeSceneEvent) {

    unsafe {

        if RENDERER.is_none() {
            panic!("Cannot change event when RENDERER is not initialized");
        }

        info!("Changing scene");

        RENDERER.as_mut().unwrap().set_scene(Rc::clone(&event.scene));

    }
}

pub fn init(renderer: Box<dyn Renderer>) {

    let mut event_bus = EventBus::new("engine");

    event_bus.subscribe_event::<ChangeSceneEvent>(change_scene_handler);

    set_renderer(renderer);



}

pub fn do_frame() {

    unsafe {

        if RENDERER.is_none() {
            panic!("Cannot do frame when RENDERER is not initialized");
        }

        RENDERER.as_mut().unwrap().do_render_cycle();

    }

}

pub fn run_windowed(width: u32, height: u32, title: &str, window_mode: glfw::WindowMode, disable_cursor: bool, fps: i32) {

    let mut glfw = glfw::init(FAIL_ON_ERRORS).unwrap();

    let (mut window, events) = glfw.create_window(width, height, title, window_mode).expect("Failed to create GLFW window.");

    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);

    if disable_cursor {
        window.set_cursor_mode(glfw::CursorMode::Disabled);
    }

    glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

    let mut raw_window_handle = window.raw_window_handle();


}

#[cfg(test)]
mod tests {
    use super::*;
}
