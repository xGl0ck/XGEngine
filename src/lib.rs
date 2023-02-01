use event_bus::EventBus;
use raw_window_handle::RawWindowHandle;

mod core;
mod events;

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

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
