use core::events::{Action, ActionEvent, InteractEvent, InteractType};
use core::renderer::renderer::RenderPerspective;
use core::scene::chunk::Chunk;
use core::scene::object::{ColoredSceneObject, ColoredVertex};
use core::shader::BgfxShaderContainer;
use core::windowed::Windowed;
use event_bus::{dispatch_event, subscribe_event};
use glam::{IVec2, Vec2, Vec3};

static mut SURFACE: Option<Windowed> = None;

fn on_key(event: &mut InteractEvent) {
    match event.interact {
        InteractType::Keyboard(glfw::Key::Escape) => unsafe {
            SURFACE.as_mut().unwrap().close_window();
        },

        InteractType::Mouse() => {
            let current_scene = core::current_scene();

            let scene = current_scene.unwrap();

            let mut scene_object = scene.borrow_mut();

            let data = &event.data;

            if data.delta.0 < 0.0 {
                scene_object.camera.at.x += 0.1;
            } else if data.delta.0 > 0.0 {
                scene_object.camera.at.x -= 0.1;
            }

            if data.delta.1 < 0.0 {
                scene_object.camera.at.y += 0.1;
            } else if data.delta.1 > 0.0 {
                scene_object.camera.at.y -= 0.1;
            }
        }

        InteractType::Keyboard(glfw::Key::W) => {
            let current_scene = core::current_scene();

            let scene = current_scene.unwrap();

            let mut scene_object = scene.borrow_mut();

            scene_object.camera.move_eye(0.1);
        }

        InteractType::Keyboard(glfw::Key::S) => {
            let current_scene = core::current_scene();

            let scene = current_scene.unwrap();

            let mut scene_object = scene.borrow_mut();

            scene_object.camera.move_eye_back(0.1);
        }

        InteractType::Keyboard(glfw::Key::T) => {
            let current_scene = core::current_scene();

            let scene = current_scene.unwrap();

            let mut scene_object = scene.borrow_mut();

            if scene_object.name == String::from("next") {
                return;
            }

            let mut event = ActionEvent::new(Action::ChangeScene(String::from("next")));

            dispatch_event!("engine", &mut event);
        }

        InteractType::Keyboard(glfw::Key::G) => {
            let current_scene = core::current_scene();

            let scene = current_scene.unwrap();

            let mut scene_object = scene.borrow_mut();

            if scene_object.name == String::from("default") {
                return;
            }

            let mut event = ActionEvent::new(Action::ChangeScene(String::from("default")));

            dispatch_event!("engine", &mut event);
        }

        _ => {}
    }
}

#[cfg(target_os = "macos")]
static PIXEL_SHADER: &'static [u8] = include_bytes!("../resources/shaders/metal/fs_cubes.bin");
#[cfg(target_os = "macos")]
static VERTEX_SHADER: &'static [u8] = include_bytes!("../resources/shaders/metal/vs_cubes.bin");

#[cfg(target_os = "linux")]
static PIXEL_SHADER: &'static [u8] = include_bytes!("../resources/shaders/opengl/fs_cubes.bin");
#[cfg(target_os = "linux")]
static VERTEX_SHADER: &'static [u8] = include_bytes!("../resources/shaders/opengl/vs_cubes.bin");

fn main() {
    const SKY_COLOR: u32 = 0x00B5DDFF;

    let mut windowed = Windowed::new(1920, 1080, "obj-scene", true, 60, SKY_COLOR);
    windowed.add_key_handler(glfw::Key::Escape, glfw::Action::Press);
    windowed.add_key_handler(glfw::Key::W, glfw::Action::Press);
    windowed.add_key_handler(glfw::Key::S, glfw::Action::Press);
    windowed.add_key_handler(glfw::Key::T, glfw::Action::Press);
    windowed.add_key_handler(glfw::Key::G, glfw::Action::Press);

    fn init_objects() {
        let mut chunk: Chunk = Chunk::new(IVec2::new(0, 0));

        let basic_object_vert: Box<[ColoredVertex]> = Box::new([
            ColoredVertex {
                coordinates: Vec3::new(-1.0, 1.0, 1.0),
                color_rgba: 0xff000000,
            },
            ColoredVertex {
                coordinates: Vec3::new(1.0, 1.0, 1.0),
                color_rgba: 0xff0000ff,
            },
            ColoredVertex {
                coordinates: Vec3::new(-1.0, -1.0, 1.0),
                color_rgba: 0xff00ff00,
            },
            ColoredVertex {
                coordinates: Vec3::new(1.0, -1.0, 1.0),
                color_rgba: 0xffff0000,
            },
            ColoredVertex {
                coordinates: Vec3::new(-1.0, 1.0, -1.0),
                color_rgba: 0xffffff00,
            },
            ColoredVertex {
                coordinates: Vec3::new(1.0, 1.0, -1.0),
                color_rgba: 0xffffffff,
            },
            ColoredVertex {
                coordinates: Vec3::new(-1.0, -1.0, -1.0),
                color_rgba: 0xff000000,
            },
            ColoredVertex {
                coordinates: Vec3::new(1.0, -1.0, -1.0),
                color_rgba: 0xff0000ff,
            },
        ]);

        // indices for a cube
        let basic_object_idx: Box<[u16]> = Box::new([
            0, 1, 2, // 0
            1, 3, 2, 4, 6, 5, // 2
            5, 6, 7, 0, 2, 4, // 4
            4, 2, 6, 1, 5, 3, // 6
            5, 7, 3, 0, 4, 1, // 8
            4, 5, 1, 2, 3, 6, // 10
            6, 3, 7,
        ]);

        let basic_object_vert_l: Box<[ColoredVertex]> = Box::new([
            ColoredVertex {
                coordinates: Vec3::new(-2.0, 2.0, 2.0),
                color_rgba: 0xff000000,
            },
            ColoredVertex {
                coordinates: Vec3::new(2.0, 2.0, 2.0),
                color_rgba: 0xff0000ff,
            },
            ColoredVertex {
                coordinates: Vec3::new(-2.0, -2.0, 2.0),
                color_rgba: 0xff00ff00,
            },
            ColoredVertex {
                coordinates: Vec3::new(2.0, -2.0, 2.0),
                color_rgba: 0xffff0000,
            },
            ColoredVertex {
                coordinates: Vec3::new(-2.0, 2.0, -2.0),
                color_rgba: 0xffffff00,
            },
            ColoredVertex {
                coordinates: Vec3::new(2.0, 2.0, -2.0),
                color_rgba: 0xffffffff,
            },
            ColoredVertex {
                coordinates: Vec3::new(-2.0, -2.0, -2.0),
                color_rgba: 0xff000000,
            },
            ColoredVertex {
                coordinates: Vec3::new(2.0, -2.0, -2.0),
                color_rgba: 0xff0000ff,
            },
        ]);

        // indices for a cube
        let basic_object_idx_l: Box<[u16]> = Box::new([
            0, 1, 2, // 0
            1, 3, 2, 4, 6, 5, // 2
            5, 6, 7, 0, 2, 4, // 4
            4, 2, 6, 1, 5, 3, // 6
            5, 7, 3, 0, 4, 1, // 8
            4, 5, 1, 2, 3, 6, // 10
            6, 3, 7,
        ]);

        // create bgfx shader container
        let shader_container = BgfxShaderContainer::new(
            Vec::from(PIXEL_SHADER),
            Vec::from(VERTEX_SHADER),
            core::shader::BgfxShaderVertexType::COLORED,
        );

        let id = core::add_shader(Box::new(shader_container));

        // create colored scene object
        let mut scene_object = ColoredSceneObject::new(
            basic_object_vert,
            basic_object_idx,
            core::get_shader(id).unwrap(),
            Vec3::new(3.0, 0.0, 0.0),
        );

        let mut scene_object_l = ColoredSceneObject::new(
            basic_object_vert_l,
            basic_object_idx_l,
            core::get_shader(id).unwrap(),
            Vec3::new(7.0, 0.0, 0.0),
        );

        chunk.add_object(Box::new(scene_object));
        chunk.add_object(Box::new(scene_object_l));

        let scene_binding = core::current_scene().unwrap();

        let mut current_scene = scene_binding.borrow_mut();

        // add chunk to current scene using crate::current_scene();
        current_scene.add_chunk(chunk, Vec2::new(-50.0, -50.0), Vec2::new(50.0, 50.0));

        current_scene.camera.set_eye(Vec3::new(-5.0, 0.0, -5.0));
        current_scene.camera.set_at(Vec3::new(0.0, 0.0, 0.0));
        current_scene.camera.set_up(Vec3::new(0.0, 0.5, 0.0));

        core::create_scene(String::from("next"));

        let mut scene = core::get_scene(String::from("next"));

        if scene.is_err() {
            panic!("Cannot get next scene");
        }

        const QUI_OBJ_COLOR_A: u32 = 0xF906A500;
        const QUI_OBJ_COLOR_B: u32 = 0xA61281FF;

        let basic_object_vert_n: Box<[ColoredVertex]> = Box::new([
            // bottom
            ColoredVertex {
                coordinates: Vec3::new(0.0, -1.0, 0.0),
                color_rgba: QUI_OBJ_COLOR_B,
            },
            ColoredVertex {
                coordinates: Vec3::new(1.0, 0.0, 0.0),
                color_rgba: QUI_OBJ_COLOR_A,
            },
            ColoredVertex {
                coordinates: Vec3::new(1.0, 1.0, 0.0),
                color_rgba: QUI_OBJ_COLOR_B,
            },
            ColoredVertex {
                coordinates: Vec3::new(0.0, 2.0, 0.0),
                color_rgba: QUI_OBJ_COLOR_A,
            },
            ColoredVertex {
                coordinates: Vec3::new(-1.0, 1.0, 0.0),
                color_rgba: QUI_OBJ_COLOR_B,
            },
            ColoredVertex {
                coordinates: Vec3::new(-1.0, 0.0, 0.0),
                color_rgba: QUI_OBJ_COLOR_A,
            },
            // top
            ColoredVertex {
                coordinates: Vec3::new(0.0, -1.0, 2.0),
                color_rgba: QUI_OBJ_COLOR_B,
            },
            ColoredVertex {
                coordinates: Vec3::new(1.0, 0.0, 2.0),
                color_rgba: QUI_OBJ_COLOR_A,
            },
            ColoredVertex {
                coordinates: Vec3::new(1.0, 1.0, 2.0),
                color_rgba: QUI_OBJ_COLOR_B,
            },
            ColoredVertex {
                coordinates: Vec3::new(0.0, 2.0, 2.0),
                color_rgba: QUI_OBJ_COLOR_A,
            },
            ColoredVertex {
                coordinates: Vec3::new(-1.0, 1.0, 2.0),
                color_rgba: QUI_OBJ_COLOR_B,
            },
            ColoredVertex {
                coordinates: Vec3::new(-1.0, 0.0, 2.0),
                color_rgba: QUI_OBJ_COLOR_A,
            },
        ]);

        // indices for a cube
        let basic_object_idx_n: Box<[u16]> = Box::new([
            // bottom
            0, 1, 5, // 0
            1, 2, 5, // 1
            2, 4, 5, // 2
            2, 3, 4, // 3
            // top
            6, 7, 11, // 0
            7, 8, 11, // 1
            8, 10, 11, // 2
            8, 9, 10, // 3
            // sides
            0, 6, 7, // 1
            7, 1, 0, // 2
            1, 7, 8, // 3
            8, 2, 1, // 4
            2, 8, 9, // 5
            9, 3, 2, // 6
            3, 9, 10, // 7
            10, 4, 3, // 8
            4, 10, 11, // 9
            11, 5, 4, // 10
            5, 11, 6, // 11
            6, 0, 5,
        ]);

        // create colored scene object
        let mut scene_object_n = ColoredSceneObject::new(
            basic_object_vert_n,
            basic_object_idx_n,
            core::get_shader(id).unwrap(),
            Vec3::new(3.0, 0.0, 0.0),
        );

        let mut scene_binding = scene.unwrap();

        let mut scene_reference = scene_binding.borrow_mut();

        let mut chunk = Chunk::new(IVec2::new(0, 0));

        chunk.add_object(Box::new(scene_object_n));

        scene_reference.add_chunk(chunk, Vec2::new(-50.0, -50.0), Vec2::new(50.0, 50.0));

        scene_reference.camera.set_eye(Vec3::new(-5.0, 0.0, -5.0));
        scene_reference.camera.set_at(Vec3::new(0.0, 0.0, 0.0));
        scene_reference.camera.set_up(Vec3::new(0.0, 0.5, 0.0));

        subscribe_event!("engine", on_key);

        core::set_debug(false);
    }

    let default_perspective = RenderPerspective::new(1920, 1080, 60.0, 0.2, 150.0);

    unsafe {
        SURFACE = Some(windowed);
        SURFACE
            .as_mut()
            .unwrap()
            .run(default_perspective, &init_objects);
    }
}
