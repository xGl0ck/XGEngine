use crate::events::{Action, ActionEvent, InteractEvent, InteractType};
use crate::renderer::renderer::{BgfxRenderer, RenderPerspective, Renderer};
use crate::ENGINE;
use event_bus::dispatch_event;
use glfw::FAIL_ON_ERRORS;
use raw_window_handle::HasRawWindowHandle;
use std::cell::RefCell;
use std::rc::Rc;

pub struct WindowedKeyHandler {
    key: glfw::Key,
    action: glfw::Action,
}

pub struct Windowed {
    width: u32,
    height: u32,
    title: String,
    disable_cursor: bool,
    fps: i32,
    key_handlers: Vec<WindowedKeyHandler>,
    window: Option<glfw::Window>,
}

impl Windowed {
    // constructor
    pub fn new(width: u32, height: u32, title: &str, disable_cursor: bool, fps: i32) -> Self {
        Self {
            width,
            height,
            title: title.to_string(),
            disable_cursor,
            fps,
            key_handlers: Vec::new(),
            window: None,
        }
    }

    // adds key handler
    pub fn add_key_handler(&mut self, key: glfw::Key, action: glfw::Action) {
        self.key_handlers.push(WindowedKeyHandler { key, action });
    }

    // closes window
    pub fn close_window(&mut self) {
        self.window.as_mut().unwrap().set_should_close(true);
    }

    // creates window, create renderer and run
    pub fn run(&mut self, default_perspective: RenderPerspective, before_cycle: &dyn Fn()) {
        let mut glfw = glfw::init(FAIL_ON_ERRORS).unwrap();

        let (mut window, events) = glfw
            .create_window(
                self.width,
                self.height,
                &self.title,
                glfw::WindowMode::Windowed,
            )
            .expect("Failed to create GLFW window.");

        glfw.window_hint(glfw::WindowHint::ClientApi(glfw::ClientApiHint::NoApi));
        window.set_key_polling(true);

        // set window
        self.window = Some(window);

        // unwrap window
        let window = self.window.as_mut().unwrap();

        //window.set_cursor_pos_polling(true);

        if self.disable_cursor {
            window.set_cursor_mode(glfw::CursorMode::Disabled);
        }

        let mut raw_window_handle = Rc::new(RefCell::new(window.raw_window_handle()));

        let mut renderer = Box::new(BgfxRenderer::new(
            self.width,
            self.height,
            Rc::clone(&raw_window_handle),
            false,
            default_perspective,
        ));

        crate::create_engine(renderer);

        crate::init();

        before_cycle();

        let mut old = (0, 0);

        let mut cursor_old: (f64, f64) = (0.0, 0.0);

        while !window.should_close() {
            glfw.poll_events();

            let current_res = window.get_framebuffer_size();

            if current_res != old {
                let mut event = ActionEvent::new(Action::UpdateResolution(
                    current_res.0 as u32,
                    current_res.1 as u32,
                ));

                dispatch_event!("engine", &mut event);

                old = current_res;
            }

            // get cursor position
            let cursor = window.get_cursor_pos();

            // calculate delta
            let delta = (cursor.0 - cursor_old.0, cursor.1 - cursor_old.1);

            cursor_old = cursor;

            if delta.0 != 0.0 || delta.1 != 0.0 {
                let mut event = InteractEvent::new(InteractType::Mouse());

                event.data.delta = delta.clone();
                event.data.cursor = cursor.clone();

                dispatch_event!("engine", &mut event);
            }

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
                        let mut event =
                            ActionEvent::new(Action::UpdateResolution(width as u32, height as u32));

                        dispatch_event!("engine", &mut event);
                    }
                    _ => {}
                }
            }

            crate::do_frame();

            // spleep in order to limit fps
            std::thread::sleep(std::time::Duration::from_millis((1000 / self.fps) as u64));
        }

        unsafe {
            let renderer = &mut ENGINE.as_mut().unwrap().renderer;

            renderer.clean_up();
            renderer.shutdown()
        }
    }
}
