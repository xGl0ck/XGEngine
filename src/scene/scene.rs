use crate::renderer::renderer::RenderView;
use crate::scene::chunk::Chunk;
use glam::{IVec2, Vec2, Vec3};
use glfw::Key::O;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard};

pub struct ChunkCorners {
    begin: Vec2,
    end: Vec2,
    chunk: IVec2,
}

impl ChunkCorners {
    fn check_range(&self, coordinates: Vec2) -> bool {
        coordinates.x >= self.begin.x
            && coordinates.y >= self.begin.y
            && coordinates.x <= self.end.x
            && coordinates.y <= self.end.y
    }
}

struct RgbaAttachment {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

pub struct Scene {
    pub name: String,
    chunk_map: HashMap<IVec2, Rc<Chunk>>,
    chunk_corners: Vec<ChunkCorners>,
    pub camera: RenderView,
    pub color_attechment: RgbaAttachment,
}

impl Scene {
    pub fn new(name: String, camera: RenderView, rgba: RgbaAttachment) -> Self {
        Self {
            name,
            chunk_map: HashMap::new(),
            chunk_corners: Vec::new(),
            camera,
            color_attechment: rgba,
        }
    }

    pub fn get_current_chunk(&self) -> std::io::Result<Rc<Chunk>> {
        let coordinates = Vec2::new(self.camera.at.x, self.camera.at.z);

        return self.get_chunk(coordinates);
    }

    pub fn get_chunk(&self, coordinates: Vec2) -> std::io::Result<Rc<Chunk>> {
        for corner in self.chunk_corners.iter() {
            if corner.check_range(coordinates) {
                let coordinates: &IVec2 = &corner.chunk;

                let chunk: Option<&Rc<Chunk>> = self.chunk_map.get(coordinates);

                if chunk.is_none() {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        "Chunk does not exist",
                    ));
                }

                return Ok(Rc::clone(chunk.unwrap()));
            }
        }

        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Chunk does not exist",
        ))
    }

    pub fn add_chunk(&mut self, chunk: Chunk, begin: Vec2, end: Vec2) {
        let corners = ChunkCorners {
            begin,
            end,
            chunk: chunk.coordinates,
        };

        self.chunk_map
            .insert(chunk.coordinates.clone(), Rc::new(chunk));
        self.chunk_corners.push(corners);
    }
}

#[cfg(test)]
mod tests {
    use crate::renderer::renderer::RenderView;
    use crate::scene::chunk::Chunk;
    use crate::scene::scene::Scene;
    use glam::{IVec2, Vec2, Vec3};

    #[test]
    fn chunk_test() {
        let mut scene = Scene::new(
            String::from("test"),
            RenderView::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(0.0, 0.0, 0.0),
            ),
        );

        let mut test_chunk = Chunk::new(IVec2::new(0, 0));

        scene.add_chunk(test_chunk, Vec2::new(0.0, 0.0), Vec2::new(150.0, 150.0));

        assert_eq!(scene.get_chunk(Vec2::new(50.0, 50.0)).is_ok(), true);
        assert_eq!(scene.get_chunk(Vec2::new(200.0, 200.0)).is_err(), true);
    }
}
