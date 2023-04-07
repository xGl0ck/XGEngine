use glam::{IVec2, Vec2, Vec3};
use XGEngine::renderer::renderer::RenderPerspective;
use XGEngine::scene::chunk::Chunk;
use XGEngine::scene::object::{ColoredSceneObject, ColoredVertex};
use XGEngine::shader::BgfxShaderContainer;
use XGEngine::windowed::Windowed;

fn main() {


    let mut windowed = Windowed::new(800, 600, "Test", false, 60);
    windowed.add_key_handler(glfw::Key::Escape, glfw::Action::Press);

    fn init_objects() {

        let mut chunk: Chunk = Chunk::new(IVec2::new(0,0));

        // define vertex buffer for cube using ColoredVertex
        let vertex_buffer: [ColoredVertex; 8] = [
            ColoredVertex { coordinates: Vec3::new(0.0, 0.0, 0.0), color_rgba: 0xff000000 },
            ColoredVertex { coordinates: Vec3::new(0.0, 0.0, 1.0), color_rgba: 0xff0000ff },
            ColoredVertex { coordinates: Vec3::new(1.0, 0.0, 1.0), color_rgba: 0xff00ff00 },
            ColoredVertex { coordinates: Vec3::new(1.0, 0.0, 0.0), color_rgba: 0xffff0000 },
            ColoredVertex { coordinates: Vec3::new(0.0, 1.0, 0.0), color_rgba: 0xffffff00 },
            ColoredVertex { coordinates: Vec3::new(0.0, 1.0, 1.0), color_rgba: 0xffffffff },
            ColoredVertex { coordinates: Vec3::new(1.0, 1.0, 1.0), color_rgba: 0xff000000 },
            ColoredVertex { coordinates: Vec3::new(1.0, 1.0, 0.0), color_rgba: 0xff0000ff },
        ];

        // define index buffer for cube
        let index_buffer: [u16; 36] = [
            0, 1, 2, 0, 2, 3, // bottom
            4, 6, 5, 4, 7, 6, // top
            0, 7, 4, 0, 3, 7, // left
            1, 5, 6, 1, 6, 2, // right
            0, 5, 1, 0, 4, 5, // front
            3, 2, 6, 3, 6, 7, // back
        ];

        // create bgfx shader container
        let shader_container = BgfxShaderContainer::new(
            std::fs::read("resources/shaders/metal/fs_cubes.bin").unwrap(),
            std::fs::read("resources/shaders/metal/vs_cubes.bin").unwrap()
        );

        let id = XGEngine::add_shader(Box::new(shader_container));

        // create colored scene object
        let mut scene_object = ColoredSceneObject::new(
            Box::new(vertex_buffer),
            Box::new(index_buffer),
            XGEngine::get_shader(id).unwrap(),
            Vec3::new(0.0, 0.0, 0.0)
        );

        chunk.add_object(Box::new(scene_object));

        // add chunk to current scene using crate::current_scene();
        XGEngine::current_scene().unwrap().borrow_mut().add_chunk(chunk, Vec2::new(-50.0, -50.0), Vec2::new(50.0, 50.0));

    }

    windowed.run(RenderPerspective::new(1920, 1080, 100.0, 150.0, 0.2), &init_objects);
}
