use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::sync::{Mutex, MutexGuard};
use glam::{IVec2};
use uuid::Uuid;
use crate::scene::object::{SceneObject};

pub struct Chunk {
    pub coordinates: IVec2,
    pub objects: RefCell<Vec<Box<dyn SceneObject>>>
}

impl Chunk {

    pub fn new(coordinates: IVec2) -> Self {
        Self {
            coordinates, objects: RefCell::new(Vec::new())
        }
    }

    pub fn add_object(&mut self, object: Box<dyn SceneObject>) -> usize {

        let index: usize = self.objects.borrow().len();

        self.objects.borrow_mut().push(object);

        index
    }

}

#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use glam::IVec2;
    use crate::scene::chunk::Chunk;

    #[test]
    fn test() {

        let mut chunk = Rc::new(RefCell::new(Chunk::new(IVec2::new(0, 0))));

        let mut reference = Rc::clone(&chunk);

        {
            let mut reference_mut = reference.borrow_mut();

            reference_mut.coordinates.x = 1;
        }

        println!("{}", chunk.borrow().coordinates.x);

        print!("");

    }

}