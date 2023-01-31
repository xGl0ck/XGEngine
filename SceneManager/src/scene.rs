use std::collections::HashMap;
use std::sync::Mutex;
use crate::chunk::Chunk;

pub struct ChunkCoordinate {
    x: i32,
    y: i32
}

pub struct Scene {
    name: String,
    chunk_map: Mutex<HashMap<ChunkCoordinate, Chunk>>
}

impl Scene {

    fn new(name: String) -> Self {
        Self {
            name, chunk_map: Mutex::new(HashMap::new())
        }
    }

    fn add_chunk(&mut self, chunk: Chunk) {

        let coordinate = ChunkCoordinate { x: chunk.x, y: chunk.y };

        let chunk_map = match self.chunk_map.lock() {
            Ok(map) => map,
            Err(poisoned) => poisoned.into_inner()
        };

        // TODO:: Add chunk logic

    }

}