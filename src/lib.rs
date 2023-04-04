use std::cell::RefCell;
use std::rc::Rc;
use event_bus::{EventBus, subscribe_event};
use glam::Vec3;
use glfw::{FAIL_ON_ERRORS, Glfw};
use glfw::Key::{B, N};
use log::info;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use crate::environment::EngineEnvironment;
use crate::events::{Action, ActionEvent};
use crate::renderer::renderer::{BgfxRenderer, Renderer, RenderPerspective, RenderView};
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

pub struct Engine {
    renderer: Box<dyn Renderer>,
    environment: EngineEnvironment
}

static mut ENGINE: Option<Engine> = None;

impl Engine {

    // constructor
    pub fn new(renderer: Box<dyn Renderer>, environment: EngineEnvironment) -> Self {
        Self {
            renderer, environment
        }
    }

    pub fn init(&mut self) {
        self.renderer.init();
    }

    pub fn do_frame(&mut self) {
        self.renderer.do_render_frame();
    }

    pub fn get_environment(&self) -> &EngineEnvironment {
        &self.environment
    }

    fn update_resolution(&mut self, width: u32, height: u32) {
        self.renderer.update_surface_resolution(width, height);
    }

}



fn create_engine(renderer: Box<dyn Renderer>) {

    unsafe {

        let environment = EngineEnvironment::new();

        ENGINE = Some(Engine::new(renderer, environment));

    }

}

fn change_scene_handler(event: &mut ChangeSceneEvent) {

    unsafe {

        if ENGINE.is_none() {
            panic!("Cannot change event when RENDERER is not initialized");
        }

        info!("Changing scene");

        ENGINE.as_mut().unwrap().renderer.set_scene(Rc::clone(&event.scene));

    }
}

fn action_event_handler(event: &mut ActionEvent) {

    match event.action {
        Action::ChangeScene(ref scene) => {

            unsafe {

                ENGINE.as_mut().unwrap().environment.render_scene(scene.clone()).expect("TODO: panic message");

            }

        },

        Action::UpdateResolution(width, height) => {
            unsafe {
                ENGINE.as_mut().unwrap().update_resolution(width, height);
            }
        }

        _ => {}
    }

}

pub fn init(renderer: Box<dyn Renderer>) {

    let mut event_bus = EventBus::new("engine");

    subscribe_event!("engine", change_scene_handler);

    event_bus.subscribe_event::<ChangeSceneEvent>(change_scene_handler);

}

pub fn do_frame() {

    unsafe {

        if ENGINE.as_mut().unwrap().is_none() {
            panic!("Cannot do frame when ENGINE is not initialized");
        }

        ENGINE.as_mut().unwrap().renderer.do_render_cycle();

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

    let mut raw_window_handle = Rc::new(RefCell::new(window.raw_window_handle()));

    let render_perspective = RenderPerspective::new(
        width,
        height,
        90.0,
        0.2,
        150.0
    );

    let mut renderer = Box::new(BgfxRenderer::new(
        width,
        height,
        Rc::clone(&raw_window_handle),
        false,
        render_perspective
    ));

    init(renderer);



    while !window.should_close() {

        unsafe {
            ENGINE.as_mut().unwrap().renderer.do_render_cycle();
        }

    }

    unsafe {
        let renderer = RENDERER.unwrap().as_mut();

        renderer.clean_up();
        renderer.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
