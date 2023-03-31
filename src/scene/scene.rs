use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard};
use glam::{IVec2, Vec2, Vec3};
use crate::renderer::renderer::RenderView;
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
    pub name: String,
    chunk_map: HashMap<IVec2, Chunk>,
    chunk_corners: Vec<ChunkCorners>,
    camera: RenderView
}

impl Scene {

    pub fn new(name: String, camera: RenderView) -> Self {
        Self {
            name, chunk_map: HashMap::new(), chunk_corners: Vec::new(), camera
        }
    }

    pub fn get_chunk(&self, coordinates: Vec2) -> Option<&Chunk> {

        for corner in self.chunk_corners.iter() {

            if corner.check_range(coordinates) {

                let coordinates: &IVec2 = &corner.chunk;

                let chunk: Option<&Chunk> = self.chunk_map.get(coordinates);

                return chunk;
            }

        }

        None
    }

    pub fn add_chunk(&mut self, chunk: Chunk, begin: Vec2, end: Vec2) {

        let corners = ChunkCorners {
            begin, end, chunk: chunk.coordinates
        };

        self.chunk_map.insert(chunk.coordinates.clone(), chunk);
        self.chunk_corners.push(corners);
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