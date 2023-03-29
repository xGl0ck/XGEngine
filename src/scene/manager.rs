use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use event_bus::{dispatch_event, Event, EventResult, subscribe_event};
use crate::events::ActionEvent;
use crate::scene::scene::Scene;

pub struct SceneManager {
    pub scene_map: Arc<Mutex<Box<HashMap<String, Scene>>>>
}

impl SceneManager {

    fn new() -> Self {
        Self {
            scene_map: Arc::new(Mutex::new(Box::new(HashMap::new())))
        }
    }

    fn add_scene(&mut self, scene: Scene) {

        let mut scene_map = match self.scene_map.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };

        scene_map.insert(String::from(&scene.name), scene);

    }

    fn render_scene(&self, name: String) -> std::io::Result<(EventResult)> {

        let scene_map = match self.scene_map.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };

        let scene: Option<&Scene> = scene_map.get(name.as_str());

        if scene.is_none() {
            panic!("Scene instance does not exist")
        }

        let mut event = ChangeSceneEvent {
            scene: scene.unwrap(),
            cancelled: false,
            reason: None
        };

        Ok(dispatch_event!("engine", &mut event))

    }

}

pub struct ChangeSceneEvent {
    scene: *const Scene,
    cancelled: bool,
    reason: Option<String>
}

impl Event for ChangeSceneEvent {

    fn cancellable(&self) -> bool {
        true
    }

    fn cancelled(&self) -> bool {
        self.cancelled
    }

    fn get_cancelled_reason(&self) -> Option<String> {
        self.reason.clone()
    }

    fn set_cancelled(&mut self, _cancel: bool, reason: Option<String>) {
        self.cancelled = _cancel;
        self.reason = reason
    }

}

#[cfg(test)]
mod tests {
    use event_bus::{Event, EventBus, subscribe_event};
    use event_bus::EventResult::EvCancelled;
    use crate::scene::manager::{ChangeSceneEvent, SceneManager};
    use crate::scene::scene::Scene;

    fn test_handler(event: &mut ChangeSceneEvent) {
        event.set_cancelled(true, Option::from(String::from("test reason")));
    }

    #[test]
    fn render_scene_test() {

        let mut test_bus = EventBus::new("engine");

        subscribe_event!("engine", test_handler);

        let mut mamager = SceneManager::new();

        let mut scene = Scene::new(String::from("test"));

        mamager.add_scene(scene);

        let mut result = match mamager.render_scene(String::from("test")) {
            Ok(res) => res,
            Err(err) => panic!("{}", err)
        };

        assert_eq!(result, EvCancelled(String::from("test reason")));

    }

}


