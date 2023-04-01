use event_bus::EventBus;
use raw_window_handle::RawWindowHandle;

mod core;
mod events;
mod environment;

mod messaging {
//    pub mod controller;
//    pub mod event;
//    pub mod message;
}

mod renderer {
    pub mod controller;
    pub mod renderer;
    pub mod events;
}

mod scene {
    pub mod chunk;
    pub mod manager;
    pub mod object;
    pub mod scene;
}

pub struct Engine {
    window_handle: RawWindowHandle
}

impl Engine {

    fn new(window_handle: RawWindowHandle) -> Self {
        Self {
            window_handle
        }
    }

    fn start() {

        let engine_event_bus = EventBus::new("engine");

        

    }

}

#[cfg(test)]
mod tests {
    use super::*;
}
