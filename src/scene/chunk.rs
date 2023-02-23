use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
use glam::{IVec2};
use uuid::Uuid;
use crate::scene::object::SceneObject;

pub struct Chunk {
    pub coordinates: IVec2,
    pub objects: Mutex<HashMap<Uuid, SceneObject>>
}

impl Chunk {

    pub fn new(coordinates: IVec2) -> Self {
        Self {
            coordinates, objects: Mutex::new(HashMap::new())
        }
    }

    pub fn add_object(&mut self, object: SceneObject) {

        let mut objects: MutexGuard<HashMap<Uuid, SceneObject>> = match self.objects.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };

        objects.insert(object.id.clone(), object);
    }

}