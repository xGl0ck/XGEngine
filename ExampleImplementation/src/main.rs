use event_bus::subscribe_event;
use glam::{IVec2, Vec2, Vec3};
use XGEngine::events::{Action, ActionEvent, InteractEvent, InteractType};
use XGEngine::renderer::renderer::RenderPerspective;
use XGEngine::scene::chunk::Chunk;
use XGEngine::scene::object::{ColoredSceneObject, ColoredVertex};
use XGEngine::shader::BgfxShaderContainer;
use XGEngine::windowed::Windowed;

static mut SURFACE: Option<Windowed> = None;

fn on_key(event: &mut InteractEvent) {

    match event.interact {

        InteractType::Keyboard(glfw::Key::Escape) => {
            unsafe {
                SURFACE.as_mut().unwrap().close_window();
            }
        }

        _ => {}
    }

}

fn main() {

    let mut windowed = Windowed::new(1920, 1080, "Test", false, 60);
    windowed.add_key_handler(glfw::Key::Escape, glfw::Action::Press);

    fn init_objects() {

        let mut chunk: Chunk = Chunk::new(IVec2::new(0,0));

        let basic_object_vert: Box<[ColoredVertex]> = Box::new(
            [
                ColoredVertex { coordinates: Vec3::new(0.0, 0.0, 0.0), color_rgba: 0xff000000 },
                ColoredVertex { coordinates: Vec3::new(0.0, 0.0, 1.0), color_rgba: 0xff0000ff },
                ColoredVertex { coordinates: Vec3::new(1.0, 0.0, 1.0), color_rgba: 0xff00ff00 },
                ColoredVertex { coordinates: Vec3::new(1.0, 0.0, 0.0), color_rgba: 0xffff0000 },
                ColoredVertex { coordinates: Vec3::new(0.0, 1.0, 0.0), color_rgba: 0xffffff00 },
                ColoredVertex { coordinates: Vec3::new(0.0, 1.0, 1.0), color_rgba: 0xffffffff },
                ColoredVertex { coordinates: Vec3::new(1.0, 1.0, 1.0), color_rgba: 0xff000000 },
                ColoredVertex { coordinates: Vec3::new(1.0, 1.0, 0.0), color_rgba: 0xff0000ff },
            ]
        );

        // indices for a cube
        let basic_object_idx: Box<[u16]> = Box::new(
            [
                0, 1, 2, // 0
                1, 3, 2,
                4, 6, 5, // 2
                5, 6, 7,
                0, 2, 4, // 4
                4, 2, 6,
                1, 5, 3, // 6
                5, 7, 3,
                0, 4, 1, // 8
                4, 5, 1,
                2, 3, 6, // 10
                6, 3, 7,
            ]
        );

        // create bgfx shader container
        let shader_container = BgfxShaderContainer::new(
            std::fs::read("resources/shaders/metal/fs_cubes.bin").unwrap(),
            std::fs::read("resources/shaders/metal/vs_cubes.bin").unwrap()
        );

        let id = XGEngine::add_shader(Box::new(shader_container));

        // create colored scene object
        let mut scene_object = ColoredSceneObject::new(
            basic_object_vert,
            basic_object_idx,
            XGEngine::get_shader(id).unwrap(),
            Vec3::new(5.0, 0.0, 0.0)
        );

        chunk.add_object(Box::new(scene_object));

        let scene_binding = XGEngine::current_scene().unwrap();

        let mut current_scene = scene_binding.borrow_mut();

        // add chunk to current scene using crate::current_scene();
        current_scene.add_chunk(chunk, Vec2::new(-50.0, -50.0), Vec2::new(50.0, 50.0));

        current_scene.camera.set_eye(Vec3::new(-5.0, 0.0, -5.0));
        current_scene.camera.set_at(Vec3::new(0.0, 0.0, 0.0));
        current_scene.camera.set_up(Vec3::new(0.0, 0.5, 0.0));

        XGEngine::set_debug(false);

    }

    subscribe_event!("engine", on_key);

    unsafe {
        SURFACE = Some(windowed);
        SURFACE.as_mut().unwrap().run(RenderPerspective::new(1920, 1080, 60.0, 0.2, 150.0), &init_objects);
    }
}
