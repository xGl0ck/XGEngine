use std::cell::Ref;
use std::collections::HashMap;
use std::sync::{Mutex, MutexGuard};
use glam::{IVec2};
use uuid::Uuid;
use crate::scene::object::SceneObject;

pub struct Chunk {
    pub coordinates: IVec2,
    pub objects: Vec<SceneObject>
}

impl Chunk {

    pub fn new(coordinates: IVec2) -> Self {
        Self {
            coordinates, objects: Vec::new()
        }
    }

    pub fn add_object(&mut self, object: SceneObject) -> usize {

        let index: usize = self.objects.len();

        self.objects.push(object);

        index
    }

}