use crate::environment::EngineEnvironment;
use crate::events::{Action, ActionEvent, InteractEvent, InteractType};
use crate::renderer::renderer::{BgfxRenderer, RenderPerspective, RenderView, Renderer};
use crate::scene::manager::{ChangeSceneEvent, SceneManager};
use crate::scene::scene::Scene;
use crate::shader::{ShaderContainer, ShaderManager};
use event_bus::{dispatch_event, subscribe_event, EventBus};
use glam::Vec3;
use glfw::Key::{B, N, P};
use glfw::{Glfw, FAIL_ON_ERRORS};
use log::info;
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::cell::RefCell;
use std::rc::Rc;

mod core;
mod environment;
pub mod events;
pub mod shader;
pub mod windowed;

mod messaging {
    //    pub mod controller;
    //    pub mod event;
    //    pub mod message;
}

pub mod renderer {
    pub mod controller;
    pub mod events;
    pub mod renderer;
}

pub mod scene {
    pub mod chunk;
    pub mod manager;
    pub mod object;
    pub mod scene;
}

pub struct Engine {
    renderer: Box<dyn Renderer>,
    environment: EngineEnvironment,
    shader_manager: ShaderManager,
    bus: EventBus,
}

static mut ENGINE: Option<Engine> = None;

impl Engine {
    // constructor
    pub fn new(renderer: Box<dyn Renderer>, environment: EngineEnvironment) -> Self {
        Self {
            renderer,
            environment,
            shader_manager: ShaderManager::new(),
            bus: EventBus::new("engine"),
        }
    }

    pub fn init(&mut self) {
        self.renderer.init();
    }

    pub fn do_frame(&mut self) {
        self.renderer.do_render_cycle();
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

pub fn set_debug(debug: bool) {
    unsafe {
        if ENGINE.is_none() {
            panic!("Cannot debug when ENGINE is not initialized");
        }

        ENGINE.as_mut().unwrap().renderer.do_debug(debug);
    }
}

// create scene in engine environment
pub fn create_scene(name: String) {
    unsafe {
        if ENGINE.is_none() {
            panic!("Cannot create scene when ENGINE is not initialized");
        }

        ENGINE.as_mut().unwrap().environment.create_scene(name);
    }
}

// get scene
pub fn get_scene(name: String) -> std::io::Result<Rc<RefCell<Scene>>> {
    unsafe {
        if ENGINE.is_none() {
            panic!("Cannot get scene when ENGINE is not initialized");
        }

        ENGINE.as_mut().unwrap().environment.get_scene(name)
    }
}

// current scene
pub fn current_scene() -> std::io::Result<Rc<RefCell<Scene>>> {
    unsafe {
        if ENGINE.is_none() {
            panic!("Cannot get scene when ENGINE is not initialized");
        }

        Ok(Rc::clone(
            &ENGINE.as_mut().unwrap().environment.current_scene,
        ))
    }
}

// add shader
pub fn add_shader(shader: Box<dyn ShaderContainer>) -> i32 {
    unsafe {
        if ENGINE.is_none() {
            panic!("Cannot add shader when ENGINE is not initialized");
        }

        ENGINE.as_mut().unwrap().shader_manager.add_shader(shader)
    }
}

// get shader
pub fn get_shader(id: i32) -> std::io::Result<Rc<RefCell<Box<dyn ShaderContainer>>>> {
    unsafe {
        if ENGINE.is_none() {
            panic!("Cannot get shader when ENGINE is not initialized");
        }

        let shader = ENGINE.as_mut().unwrap().shader_manager.get_shader(id);

        if shader.is_none() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Shader not found",
            ));
        }

        Ok(shader.unwrap())
    }
}

fn change_scene_handler(event: &mut ChangeSceneEvent) {
    unsafe {
        if ENGINE.is_none() {
            panic!("Cannot change event when RENDERER is not initialized");
        }

        info!("Changing scene");

        ENGINE
            .as_mut()
            .unwrap()
            .renderer
            .set_scene(Rc::clone(&event.scene));
    }
}

fn action_event_handler(event: &mut ActionEvent) {
    match event.action {
        Action::ChangeScene(ref scene) => unsafe {
            ENGINE
                .as_mut()
                .unwrap()
                .environment
                .render_scene(scene.clone())
                .expect("TODO: panic message");
        },

        Action::UpdateResolution(width, height) => unsafe {
            println!("Updating resolution: {}, {}", width, height);

            ENGINE.as_mut().unwrap().update_resolution(width, height);
        },

        _ => {}
    }
}

pub fn init() {
    unsafe {
        ENGINE.as_mut().unwrap().init();
    }

    subscribe_event!("engine", change_scene_handler);
    subscribe_event!("engine", action_event_handler);

    unsafe {
        ENGINE
            .as_mut()
            .unwrap()
            .environment
            .scene_manager
            .render_scene(String::from("default"));
    }
}

pub fn do_frame() {
    unsafe {
        if ENGINE.as_mut().is_none() {
            panic!("Cannot do frame when ENGINE is not initialized");
        }

        ENGINE.as_mut().unwrap().renderer.do_render_cycle();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;
}
