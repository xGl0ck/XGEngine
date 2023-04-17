use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use event_bus::{dispatch_event, Event, EventResult, subscribe_event};
use glam::Vec3;
use crate::events::ActionEvent;
use crate::renderer::renderer::RenderView;
use crate::scene::scene::Scene;

pub struct SceneManager {
    pub scene_map: Arc<Mutex<Box<HashMap<String, Rc<RefCell<Scene>>>>>>
}

impl SceneManager {

    pub fn new() -> Self {

        let default_scene = Scene::new(String::from("default"), RenderView::new(Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,0.0,0.0)));

        let mut scene_map: Box<HashMap<String, Rc<RefCell<Scene>>>> = Box::new(HashMap::new());

        scene_map.insert(String::from(&default_scene.name.clone()), Rc::new(RefCell::new(default_scene)));

        Self {
            scene_map: Arc::new(Mutex::new(scene_map))
        }
    }

    pub fn add_scene(&mut self, scene: Scene) {

        let mut scene_map = match self.scene_map.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };

        scene_map.insert(String::from(&scene.name), Rc::new(RefCell::new(scene)));

    }

    pub fn get_scene(&self, name: String) -> std::io::Result<Rc<RefCell<Scene>>> {

        let scene_map = match self.scene_map.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };

        let scene: Option<&Rc<RefCell<Scene>>> = scene_map.get(name.as_str());

        match scene {
            Some(scene) => Ok(Rc::clone(&scene)),
            None => {
                Err(std::io::Error::new(std::io::ErrorKind::Other, "Scene instance does not exist"))
            }
        }

    }

    pub fn render_scene(&self, name: String) -> std::io::Result<(EventResult)> {

        let scene_map = match self.scene_map.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };

        let scene: Option<&Rc<RefCell<Scene>>> = scene_map.get(name.as_str());

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
    pub scene: Rc<RefCell<Scene>>,
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
    use std::cell::{Cell, RefCell};
    use std::ops::Deref;
    use std::rc::Rc;
    use event_bus::{Event, EventBus, subscribe_event};
    use event_bus::EventResult::EvCancelled;
    use glam::{IVec2, Vec2, Vec3};
    use crate::renderer::renderer::RenderView;
    use crate::scene::chunk::Chunk;
    use crate::scene::manager::{ChangeSceneEvent, SceneManager};
    use crate::scene::scene::Scene;

    static mut RENDERER: Cell<Option<RendererSim>> = Cell::new(None);

    struct RendererSim {
        current_scene: Rc<RefCell<Scene>>
    }

    impl RendererSim {

        fn new(default: Rc<RefCell<Scene>>) -> Self {
            Self {
                current_scene: Rc::clone(&default)
            }
        }

        fn change_scene(&mut self, scene: Rc<RefCell<Scene>>) {
            self.current_scene = Rc::clone(&scene);
        }

        fn render(&self) {

            let chunk = self.current_scene.borrow().get_chunk(Vec2::new(0.0, 0.0));

            println!("{}", chunk.is_ok());

        }

    }

    unsafe fn set_renderer(renderer: RendererSim) {
        RENDERER.set(Some(renderer));
    }


    fn test_handler(event: &mut ChangeSceneEvent) {

        unsafe {

            let scene = Rc::clone(&event.scene);

            RENDERER.get_mut().as_mut().unwrap().change_scene(scene);

        }

        event.set_cancelled(true, Option::from(String::from("test reason")));
    }

    #[test]
    fn render_scene_test() {

        let mut test_bus = EventBus::new("engine");

        subscribe_event!("engine", test_handler);

        let mut mamager = SceneManager::new();

        let mut scene = Scene::new(String::from("test"), RenderView::new(Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,0.0,0.0)));

        mamager.add_scene(scene);

        unsafe {

            set_renderer(RendererSim::new(mamager.get_scene(String::from("test")).unwrap()))

        }

        let mut result = match mamager.render_scene(String::from("test")) {
            Ok(res) => res,
            Err(err) => panic!("{}", err)
        };

        unsafe {
            RENDERER.get_mut().as_ref().unwrap().render();
        };

        let chunk = Chunk::new(IVec2::new(0,0));

        mamager.get_scene(String::from("test")).unwrap().borrow_mut().add_chunk(chunk, Vec2::new(-5.0, -5.0), Vec2::new(5.0, 5.0));

        unsafe {

            RENDERER.get_mut().as_mut().unwrap().render();

        }

        assert_eq!(result, EvCancelled(String::from("test reason")));

    }

}


