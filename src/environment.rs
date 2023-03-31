use std::rc::Rc;
use event_bus::EventResult;
use log::error;
use crate::scene::manager::SceneManager;
use crate::scene::scene::Scene;

pub struct EngineEnvironment {
    pub scene_manager: SceneManager,
    pub current_scene: Rc<Scene>
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

    fn create_scene(&mut self, name: String) {

        let scene = Scene::new(name);

        self.scene_manager.add_scene(scene);

    }

    fn get_scene(&self, name: String) -> std::io::Result<Rc<Scene>> {

        let scene = self.scene_manager.get_scene(name);

        if scene.is_none() {
            panic!("Scene instance does not exist")
        }

        match scene {
            Some(scene) => Ok(scene),
            None => {
                error!("Scene instance does not exist");
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Scene instance does not exist"))
            }
        }

    }

    fn render_scene(&self, name: String) -> std::io::Result<(EventResult)> {

        self.scene_manager.render_scene(name)

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
        assert_eq!(scene.unwrap().name, "default");
    }

    fn event_sub(event: &mut ChangeSceneEvent) {
        info!("Event received: {:?}", event.scene.name);

        println!("Event received: {:?}", event.scene.name);
    }

    #[test]
    fn test_render_scene() {

        // init event bus
        let engine_event_bus = EventBus::new("engine");

        info!("Engine event bus initialized");

        subscribe_event!("engine", event_sub);

        let environment = EngineEnvironment::new();
        let result = environment.render_scene(String::from("default"));
        assert_eq!(result.is_ok(), true);
    }

}