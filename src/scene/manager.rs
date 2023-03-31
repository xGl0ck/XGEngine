use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use event_bus::{dispatch_event, Event, EventResult, subscribe_event};
use crate::events::ActionEvent;
use crate::scene::scene::Scene;

pub struct SceneManager {
    pub scene_map: Arc<Mutex<Box<HashMap<String, Rc<Scene>>>>>
}

impl SceneManager {

    pub fn new() -> Self {

        let default_scene = Scene::new(String::from("default"));

        let mut scene_map: Box<HashMap<String, Rc<Scene>>> = Box::new(HashMap::new());

        scene_map.insert(String::from(&default_scene.name.clone()), Rc::new(default_scene));

        Self {
            scene_map: Arc::new(Mutex::new(scene_map))
        }
    }

    pub fn add_scene(&mut self, scene: Scene) {

        let mut scene_map = match self.scene_map.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };

        scene_map.insert(String::from(&scene.name), Rc::new(scene));

    }

    pub fn get_scene(&self, name: String) -> Option<Rc<Scene>> {

        let scene_map = match self.scene_map.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };

        let scene: Option<&Rc<Scene>> = scene_map.get(name.as_str());

        if scene.is_none() {
            panic!("Scene instance does not exist")
        }

        scene.cloned()
    }

    pub fn render_scene(&self, name: String) -> std::io::Result<(EventResult)> {

        let scene_map = match self.scene_map.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };

        let scene: Option<&Rc<Scene>> = scene_map.get(name.as_str());

        if scene.is_none() {
            panic!("Scene instance does not exist")
        }

        let mut event = ChangeSceneEvent {
            scene: scene.unwrap().clone(),
            cancelled: false,
            reason: None
        };

        Ok(dispatch_event!("engine", &mut event))

    }

    fn has_scene(&self, name: String) -> bool {
        let scene_map = match self.scene_map.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };

        scene_map.contains_key(name.as_str())
    }

}

pub struct ChangeSceneEvent {
    pub scene: Rc<Scene>,
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


