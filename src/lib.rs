use std::rc::Rc;
use event_bus::EventBus;
use glfw::Key::N;
use log::info;
use raw_window_handle::RawWindowHandle;
use crate::renderer::renderer::Renderer;
use crate::scene::manager::ChangeSceneEvent;

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

        RENDERER.unwrap().as_mut().set_scene(Rc::clone(&event.scene));

    }

}



#[cfg(test)]
mod tests {
    use super::*;
}
