use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use event_bus::EventResult;
use glam::Vec3;
use log::error;
use crate::renderer::renderer::{Renderer, RenderPerspective, RenderView};
use crate::scene::manager::SceneManager;
use crate::scene::scene::Scene;

pub struct EngineEnvironment {
    pub scene_manager: SceneManager,
    pub current_scene: Rc<RefCell<Scene>>,
}

impl EngineEnvironment {

    pub fn new() -> Self {

        let mut scene_manager = SceneManager::new();

        let default_scene = scene_manager.get_scene(String::from("default")).unwrap();

        Self {
            scene_manager,
            current_scene: default_scene
        }
    }

    pub fn create_scene(&mut self, name: String) {

        let scene = Scene::new(name, RenderView::new(Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,0.0,0.0)));

        self.scene_manager.add_scene(scene);

    }

    pub fn get_scene(&self, name: String) -> std::io::Result<Rc<RefCell<Scene>>> {

        let scene = self.scene_manager.get_scene(name);

        match scene {
            Ok(scene) => Ok(Rc::clone(&scene)),
            Err(e) => {
                error!("Scene instance does not exist");
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Scene instance does not exist"))
            }
        }

    }

    pub fn render_scene(&mut self, name: String) -> std::io::Result<(EventResult)> {

        let result = self.scene_manager.render_scene(name.clone());

        if result.is_ok() {
            self.current_scene = self.get_scene(name.clone()).unwrap();
        }

        result
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use event_bus::{EventBus, subscribe_event};
    use log::info;
    use crate::scene::manager::ChangeSceneEvent;
    use super::*;

    #[test]
    fn test_create_scene() {
        let mut environment = EngineEnvironment::new();
        environment.create_scene(String::from("test"));
        assert_eq!(environment.scene_manager.scene_map.lock().unwrap().len(), 2);
    }

    #[test]
    fn test_get_scene() {
        let environment = EngineEnvironment::new();
        let scene = environment.get_scene(String::from("default"));
        assert_eq!(scene.unwrap().borrow().name, "default");
    }

    fn event_sub(event: &mut ChangeSceneEvent) {
        info!("Event received: {:?}", event.scene.borrow().name);
    }

    #[test]
    fn test_render_scene() {

        // init event bus
        let engine_event_bus = EventBus::new("engine");

        info!("Engine event bus initialized");

        subscribe_event!("engine", event_sub);

        let mut environment = EngineEnvironment::new();
        let result = environment.render_scene(String::from("default"));
        assert_eq!(result.is_ok(), true);
    }

}