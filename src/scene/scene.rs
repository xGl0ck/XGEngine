use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::scene::chunk::Chunk;

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct ChunkCoordinate {
    x: i32,
    y: i32
}

pub struct ChunkCorners {
    begin: ChunkCoordinate,
    end: ChunkCoordinate,
    chunk: ChunkCoordinate
}

impl ChunkCorners {

    fn check_range(&self, x: i32, y: i32) -> bool {
        x >= self.begin.x && y >= self.begin.y && x <= self.end.x && y <= self.end.y
    }

}

pub struct Scene {
    name: String,
    chunk_map: Arc<Mutex<HashMap<ChunkCoordinate, Chunk>>>,
    chunk_corners: Mutex<Vec<ChunkCorners>>
}

impl Scene {

    fn new(name: String) -> Self {
        Self {
            name, chunk_map: Arc::new(Mutex::new(HashMap::new())), chunk_corners: Mutex::new(Vec::new())
        }
    }

    fn get_chunk(&self, x: i32, y: i32) -> Option<&Chunk> {

        let mut corners = match self.chunk_corners.lock() {
            Ok(data) => data,
            Err(poisoned) => poisoned.into_inner()
        };

        for corner in corners.iter() {
            if corner.check_range(x, y) {

                let mut map = match self.chunk_map.lock() {
                    Ok(data) => data,
                    Err(poisoned) => poisoned.into_inner()
                };

                return map.get(&corner.chunk.clone());
            }
        }

        None
    }

    fn add_chunk(&mut self, chunk: Chunk, begin: ChunkCoordinate, end: ChunkCoordinate) -> Option<Chunk> {

        let coordinate = ChunkCoordinate { x: chunk.x, y: chunk.y };

        let mut chunk_map = match self.chunk_map.lock() {
            Ok(map) => map,
            Err(poisoned) => poisoned.into_inner()
        };

        let corners = ChunkCorners {
            begin, end, chunk: coordinate.clone()
        };

        chunk_map.insert(coordinate, chunk)
    }

}