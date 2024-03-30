use event_bus::{dispatch_event, subscribe_event};
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
        InteractType::Keyboard(glfw::Key::Escape) => unsafe {
            SURFACE.as_mut().unwrap().close_window();
        },

        InteractType::Mouse() => {
            let current_scene = XGEngine::current_scene();

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
            let current_scene = XGEngine::current_scene();

            let scene = current_scene.unwrap();

            let mut scene_object = scene.borrow_mut();

            scene_object.camera.move_eye(0.1);
        }

        InteractType::Keyboard(glfw::Key::S) => {
            let current_scene = XGEngine::current_scene();

            let scene = current_scene.unwrap();

            let mut scene_object = scene.borrow_mut();

            scene_object.camera.move_eye_back(0.1);
        }

        InteractType::Keyboard(glfw::Key::T) => {
            let current_scene = XGEngine::current_scene();

            let scene = current_scene.unwrap();

            let mut scene_object = scene.borrow_mut();

            if scene_object.name == String::from("next") {
                return;
            }

            let mut event = ActionEvent::new(Action::ChangeScene(String::from("next")));

            dispatch_event!("engine", &mut event);
        }

        InteractType::Keyboard(glfw::Key::G) => {
            let current_scene = XGEngine::current_scene();

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

fn main() {
    let mut windowed = Windowed::new(1920, 1080, "Test", true, 60);
    windowed.add_key_handler(glfw::Key::Escape, glfw::Action::Press);
    windowed.add_key_handler(glfw::Key::W, glfw::Action::Press);
    windowed.add_key_handler(glfw::Key::S, glfw::Action::Press);
    windowed.add_key_handler(glfw::Key::T, glfw::Action::Press);
    windowed.add_key_handler(glfw::Key::G, glfw::Action::Press);

    fn init_objects() {
        let mut chunk: Chunk = Chunk::new(IVec2::new(0, 0));

        let basic_object_vert: Box<[ColoredVertex]> = Box::new([
            ColoredVertex {
                coordinates: Vec3::new(-0.5, -0.866, -0.5),
                color_rgba: 0xff000000,
            }, // Bottom-left-back
            ColoredVertex {
                coordinates: Vec3::new(0.5, -0.866, -0.5),
                color_rgba: 0xff0000ff,
            }, // Bottom-right-back
            ColoredVertex {
                coordinates: Vec3::new(1.0, 0.0, 0.0),
                color_rgba: 0xff00ff00,
            }, // Right
            ColoredVertex {
                coordinates: Vec3::new(0.5, 0.866, -0.5),
                color_rgba: 0xffff0000,
            }, // Top-right-back
            ColoredVertex {
                coordinates: Vec3::new(-0.5, 0.866, -0.5),
                color_rgba: 0xffffff00,
            }, // Top-left-back
            ColoredVertex {
                coordinates: Vec3::new(-1.0, 0.0, 0.0),
                color_rgba: 0xffffffff,
            }, // Left
            ColoredVertex {
                coordinates: Vec3::new(-0.5, -0.866, 0.5),
                color_rgba: 0xff000000,
            }, // Bottom-left-front
            ColoredVertex {
                coordinates: Vec3::new(0.5, -0.866, 0.5),
                color_rgba: 0xff0000ff,
            }, // Bottom-right-front
            ColoredVertex {
                coordinates: Vec3::new(1.0, 0.0, 0.0),
                color_rgba: 0xff00ff00,
            }, // Right
            ColoredVertex {
                coordinates: Vec3::new(0.5, 0.866, 0.5),
                color_rgba: 0xffff0000,
            }, // Top-right-front
            ColoredVertex {
                coordinates: Vec3::new(-0.5, 0.866, 0.5),
                color_rgba: 0xffffff00,
            }, // Top-left-front
            ColoredVertex {
                coordinates: Vec3::new(-1.0, 0.0, 0.0),
                color_rgba: 0xffffffff,
            }, // Leftft
        ]);

        // indices for a cube
        let basic_object_idx: Box<[u16]> = Box::new([
            0, 1, 6, 6, 1, 7, // Side faces
            1, 2, 7, 7, 2, 8, 2, 3, 8, 8, 3, 9, 3, 4, 9, 9, 4, 10, 4, 5, 10, 10, 5, 11,
            // Top face
            4, 3, 11, 11, 3, 9,
        ]);

        let basic_object_vert_l: Box<[ColoredVertex]> = Box::new([
            ColoredVertex {
                coordinates: Vec3::new(0.0, 0.0, 0.0),
                color_rgba: 0xff000000,
            },
            ColoredVertex {
                coordinates: Vec3::new(0.0, 0.0, 2.0),
                color_rgba: 0xff0000ff,
            },
            ColoredVertex {
                coordinates: Vec3::new(2.0, 0.0, 2.0),
                color_rgba: 0xff00ff00,
            },
            ColoredVertex {
                coordinates: Vec3::new(2.0, 0.0, 0.0),
                color_rgba: 0xffff0000,
            },
            ColoredVertex {
                coordinates: Vec3::new(0.0, 2.0, 0.0),
                color_rgba: 0xffffff00,
            },
            ColoredVertex {
                coordinates: Vec3::new(0.0, 2.0, 2.0),
                color_rgba: 0xffffffff,
            },
            ColoredVertex {
                coordinates: Vec3::new(2.0, 2.0, 2.0),
                color_rgba: 0xff000000,
            },
            ColoredVertex {
                coordinates: Vec3::new(2.0, 2.0, 0.0),
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
            Vec::from(include_bytes!("../resources/fs_cubes.bin")),
            Vec::from(include_bytes!("../resources/vs_cubes.bin")),
        );

        let id = XGEngine::add_shader(Box::new(shader_container));

        // create colored scene object
        let mut scene_object = ColoredSceneObject::new(
            basic_object_vert,
            basic_object_idx,
            XGEngine::get_shader(id).unwrap(),
            Vec3::new(5.0, 0.0, 0.0),
        );

        let mut scene_object_l = ColoredSceneObject::new(
            basic_object_vert_l,
            basic_object_idx_l,
            XGEngine::get_shader(id).unwrap(),
            Vec3::new(7.0, 0.0, 0.0),
        );

        chunk.add_object(Box::new(scene_object));
        chunk.add_object(Box::new(scene_object_l));

        let scene_binding = XGEngine::current_scene().unwrap();

        let mut current_scene = scene_binding.borrow_mut();

        // add chunk to current scene using crate::current_scene();
        current_scene.add_chunk(chunk, Vec2::new(-50.0, -50.0), Vec2::new(50.0, 50.0));

        current_scene.camera.set_eye(Vec3::new(-5.0, 0.0, -5.0));
        current_scene.camera.set_at(Vec3::new(0.0, 0.0, 0.0));
        current_scene.camera.set_up(Vec3::new(0.0, 0.5, 0.0));

        XGEngine::create_scene(String::from("next"));

        let mut scene = XGEngine::get_scene(String::from("next"));

        if scene.is_err() {
            panic!("Cannot get next scene");
        }

        let basic_object_vert_n: Box<[ColoredVertex]> = Box::new([
            ColoredVertex {
                coordinates: Vec3::new(0.5, 0.0, 0.0),
                color_rgba: 0xff000000,
            }, // Bottom center
            ColoredVertex {
                coordinates: Vec3::new(1.0, 0.0, 0.0),
                color_rgba: 0xff0000ff,
            }, // Bottom right
            ColoredVertex {
                coordinates: Vec3::new(0.75, 0.0, -0.866),
                color_rgba: 0xff00ff00,
            }, // Bottom right-top
            ColoredVertex {
                coordinates: Vec3::new(0.25, 0.0, -0.866),
                color_rgba: 0xffff0000,
            }, // Bottom left-top
            ColoredVertex {
                coordinates: Vec3::new(0.0, 0.0, 0.0),
                color_rgba: 0xffffff00,
            }, // Bottom left
            ColoredVertex {
                coordinates: Vec3::new(0.25, 0.0, 0.866),
                color_rgba: 0xffffffff,
            }, // Top left-bottom
            ColoredVertex {
                coordinates: Vec3::new(0.75, 0.0, 0.866),
                color_rgba: 0xff000000,
            }, // Top right-bottom
        ]);

        // indices for a cube
        let basic_object_idx_n: Box<[u16]> = Box::new([
            0, 1, 2, // Bottom face
            1, 3, 2, // Bottom-right face
            2, 3, 4, // Bottom-left face
            2, 4, 5, // Top-left face
            2, 5, 6, // Top face
            1, 6, 7, // Top-right face
            1, 7, 3, // Top-bottom face
        ]);

        let basic_object_vert_l_n: Box<[ColoredVertex]> = Box::new([
            ColoredVertex {
                coordinates: Vec3::new(0.0, 0.0, 0.0),
                color_rgba: 0xff000000,
            },
            ColoredVertex {
                coordinates: Vec3::new(0.0, 0.0, 2.0),
                color_rgba: 0xff0000ff,
            },
            ColoredVertex {
                coordinates: Vec3::new(2.0, 0.0, 2.0),
                color_rgba: 0xff00ff00,
            },
            ColoredVertex {
                coordinates: Vec3::new(2.0, 0.0, 0.0),
                color_rgba: 0xffff0000,
            },
            ColoredVertex {
                coordinates: Vec3::new(0.0, 2.0, 0.0),
                color_rgba: 0xffffff00,
            },
            ColoredVertex {
                coordinates: Vec3::new(0.0, 2.0, 2.0),
                color_rgba: 0xffffffff,
            },
            ColoredVertex {
                coordinates: Vec3::new(2.0, 2.0, 2.0),
                color_rgba: 0xff000000,
            },
            ColoredVertex {
                coordinates: Vec3::new(2.0, 2.0, 0.0),
                color_rgba: 0xff0000ff,
            },
        ]);

        // indices for a cube
        let basic_object_idx_l_n: Box<[u16]> = Box::new([
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
            Vec::from(include_bytes!("../resources/fs_cubes.bin")),
            Vec::from(include_bytes!("../resources/vs_cubes.bin")),
        );

        let id = XGEngine::add_shader(Box::new(shader_container));

        // create colored scene object
        let mut scene_object_n = ColoredSceneObject::new(
            basic_object_vert_n,
            basic_object_idx_n,
            XGEngine::get_shader(id).unwrap(),
            Vec3::new(7.0, 0.0, 0.0),
        );

        let mut scene_object_l_n = ColoredSceneObject::new(
            basic_object_vert_l_n,
            basic_object_idx_l_n,
            XGEngine::get_shader(id).unwrap(),
            Vec3::new(4.0, 0.0, 0.0),
        );

        let mut scene_binding = scene.unwrap();

        let mut scene_reference = scene_binding.borrow_mut();

        let mut chunk = Chunk::new(IVec2::new(0, 0));

        chunk.add_object(Box::new(scene_object_n));
        chunk.add_object(Box::new(scene_object_l_n));

        scene_reference.add_chunk(chunk, Vec2::new(-50.0, -50.0), Vec2::new(50.0, 50.0));

        scene_reference.camera.set_eye(Vec3::new(-5.0, 0.0, -5.0));
        scene_reference.camera.set_at(Vec3::new(0.0, 0.0, 0.0));
        scene_reference.camera.set_up(Vec3::new(0.0, 0.5, 0.0));

        subscribe_event!("engine", on_key);

        XGEngine::set_debug(false);
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
