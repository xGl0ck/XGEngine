use std::cell::RefCell;
use std::rc::Rc;
use event_bus::dispatch_event;
use glfw::FAIL_ON_ERRORS;
use raw_window_handle::HasRawWindowHandle;
use crate::ENGINE;
use crate::events::{Action, ActionEvent, InteractEvent, InteractType};
use crate::renderer::renderer::{BgfxRenderer, Renderer, RenderPerspective};

pub struct WindowedKeyHandler {
    key: glfw::Key,
    action: glfw::Action
}

pub struct Windowed {
    width: u32,
    height: u32,
    title: String,
    disable_cursor: bool,
    fps: i32,
    key_handlers: Vec<WindowedKeyHandler>,
    window: Option<glfw::Window>
}

impl Windowed {

    // constructor
    pub fn new(width: u32, height: u32, title: &str, disable_cursor: bool, fps: i32) -> Self {
        Self {
            width, height, title: title.to_string(), disable_cursor, fps,
            key_handlers: Vec::new(),
            window: None,
        }
    }

    // add key handler
    pub fn add_key_handler(&mut self, key: glfw::Key, action: glfw::Action) {
        self.key_handlers.push(WindowedKeyHandler { key, action });
    }

    // close window method
    pub fn close_window(&mut self) {
        self.window.as_mut().unwrap().set_should_close(true);
    }

    // create window, create renderer and run
    fn run(&mut self, default_perspective: RenderPerspective, before_cycle: &dyn Fn()) {

        let mut glfw = glfw::init(FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw.create_window(self.width, self.height, &self.title, glfw::WindowMode::Windowed).expect("Failed to create GLFW window.");

        // set window
        self.window = Some(window);

        // unwrap window
        let window = self.window.as_mut().unwrap();

        window.set_key_polling(true);
        window.set_cursor_pos_polling(true);

        if self.disable_cursor {
            window.set_cursor_mode(glfw::CursorMode::Disabled);
        }

        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));

        let mut raw_window_handle = Rc::new(RefCell::new(window.raw_window_handle()));

        let mut renderer = Box::new(BgfxRenderer::new(
            self.width,
            self.height,
            Rc::clone(&raw_window_handle),
            false,
            default_perspective
        ));

        crate::create_engine(renderer);

        crate::init();

        before_cycle();

        while !window.should_close() {

            glfw.poll_events();

            // handle key events
            for key_handler in self.key_handlers.iter() {
                if window.get_key(key_handler.key) == key_handler.action {
                    unsafe {

                        let mut event = InteractEvent::new(InteractType::Keyboard(key_handler.key));

                        dispatch_event!("engine", &mut event);
                    }
                }
            }

            for (_, event) in glfw::flush_messages(&events) {
                match event {
                    glfw::WindowEvent::FramebufferSize(width, height) => {

                        let mut event = ActionEvent::new(Action::UpdateResolution(width as u32, height as u32));

                        dispatch_event!("engine", &mut event);
                    },
                    _ => {}
                }
            }

            unsafe {
                ENGINE.as_mut().unwrap().renderer.do_render_cycle();
            }

        }

        unsafe {
            let renderer = &mut ENGINE.as_mut().unwrap().renderer;

            renderer.clean_up();
            renderer.shutdown()
        }

    }

}

// unit tests
#[cfg(test)]
mod tests {
    use glam::{IVec2, Vec2, Vec3};
    use crate::scene::chunk::Chunk;
    use crate::scene::object::{ColoredSceneObject, ColoredVertex};
    use crate::shader::BgfxShaderContainer;
    use super::*;

    #[test]
    fn test_windowed() {
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
                std::fs::read("resources/shaders/opengl/fs_cubes.bin").unwrap(),
                std::fs::read("resources/shaders/opengl/vs_cubes.bin").unwrap()
            );

            let id = crate::add_shader(Box::new(shader_container));

            // create colored scene object
            let mut scene_object = ColoredSceneObject::new(
                Box::new(vertex_buffer),
                Box::new(index_buffer),
                crate::get_shader(id).unwrap(),
                Vec3::new(0.0, 0.0, 0.0)
            );

            chunk.add_object(Box::new(scene_object));

            // add chunk to current scene using crate::current_scene();
            unsafe {
                crate::current_scene().unwrap().borrow_mut().add_chunk(chunk, Vec2::new(-50.0, -50.0), Vec2::new(50.0, 50.0));
            }

        }

        windowed.run(RenderPerspective::new(1920, 1080, 100.0, 150.0, 0.2), &init_objects);
    }
}