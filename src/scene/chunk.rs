use std::sync::{Mutex, MutexGuard};
use crate::scene::object::SceneObject;

pub struct Chunk {
    pub x: i32,
    pub y: i32,
    pub objects: Mutex<Vec<SceneObject>>
}

impl Chunk {

    pub fn new(x: i32, y: i32) -> Self {
        Self {
            x, y, objects: Mutex::new(Vec::new())
        }
    }

    pub fn add_object(&mut self, object: SceneObject) {

        let mut objects: MutexGuard<Vec<SceneObject>> = match self.objects.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };

        objects.push(object);

    }

}