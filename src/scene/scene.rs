use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use glam::{IVec2, Vec2};
use crate::scene::chunk::Chunk;

pub struct ChunkCorners {
    begin: Vec2,
    end: Vec2,
    chunk: IVec2
}

impl ChunkCorners {

    fn check_range(&self, coordinates: Vec2) -> bool {
        coordinates.x >= self.begin.x &&
            coordinates.y >= self.begin.y &&
            coordinates.x <= self.end.x &&
            coordinates.y <= self.end.y
    }

}

pub struct Scene {
    name: String,
    chunk_map: Arc<Mutex<HashMap<IVec2, Chunk>>>,
    chunk_corners: Arc<Mutex<Vec<ChunkCorners>>>
}

impl Scene {

    fn new(name: String) -> Self {
        Self {
            name, chunk_map: Arc::new(Mutex::new(HashMap::new())), chunk_corners: Arc::new(Mutex::new(Vec::new()))
        }
    }

    fn get_chunk(&self, coordinates: Vec2) -> Option<&Chunk> {

        let mut corners = match self.chunk_corners.lock() {
            Ok(data) => data,
            Err(poisoned) => poisoned.into_inner()
        };

        for corner in corners.iter() {

            if corner.check_range(coordinates) {

                let mut map = match self.chunk_map.lock() {
                    Ok(data) => data,
                    Err(poisoned) => poisoned.into_inner()
                };

                let coordinates: &IVec2 = &corner.chunk;

                let chunk: Option<&Chunk> = map.get(coordinates);

                return chunk;
            }

        }

        None
    }

    fn add_chunk(&mut self, chunk: Chunk, begin: Vec2, end: Vec2) {

        let mut chunk_map: MutexGuard<HashMap<IVec2, Chunk>> = match self.chunk_map.lock() {
            Ok(map) => map,
            Err(poisoned) => poisoned.into_inner()
        };

        let mut corners_vec: MutexGuard<Vec<ChunkCorners>> = match self.chunk_corners.lock() {
            Ok(map) => map,
            Err(poisoned) => poisoned.into_inner()
        };

        let corners = ChunkCorners {
            begin, end, chunk: chunk.coordinates
        };

        chunk_map.insert(chunk.coordinates.clone(), chunk);
        corners_vec.push(corners);
    }

}

#[cfg(test)]
mod tests {
    use glam::{IVec2, Vec2};
    use crate::scene::chunk::Chunk;
    use crate::scene::scene::Scene;

    #[test]
    fn chunk_test() {

        let mut scene = Scene::new(String::from("test"));

        let mut test_chunk = Chunk::new(IVec2::new(0, 0));

        scene.add_chunk(test_chunk, Vec2::new(0.0, 0.0), Vec2::new(150.0, 150.0));

        assert_eq!(scene.get_chunk(Vec2::new(50.0, 50.0)).is_some(), true);
        assert_eq!(scene.get_chunk(Vec2::new(200.0, 200.0)).is_none(), true);

    }

}