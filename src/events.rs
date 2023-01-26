use event_bus::Event;

pub enum InteractType {
    KEYBOARD(glfw::Key),
    MOUSE(glfw::MouseButton, i32, i32)
}

pub struct InitEvent {
    cancelled: bool,
    reason: Option<String>
}

pub struct ShutdownEvent {
    cancelled: bool,
    reason: Option<String>
}

pub struct InteractEvent {
    interact: InteractType,
    cancelled: bool,
    reason: Option<String>
}

impl Event for InteractEvent {
    fn cancellable(&self) -> bool {
        true
    }

    fn cancelled(&self) -> bool {
        self.cancelled
    }

    fn get_cancelled_reason(&self) -> Option<String> {
        self.reason.clone()
    }

    fn set_cancelled(&mut self, _cancel: bool, reason: Option<String>) {
        self.cancelled = _cancel;
        self.reason = reason;
    }
}

impl Event for ShutdownEvent {
    fn cancellable(&self) -> bool {
        true
    }

    fn cancelled(&self) -> bool {
        self.cancelled
    }

    fn get_cancelled_reason(&self) -> Option<String> {
        self.reason.clone()
    }

    fn set_cancelled(&mut self, _cancel: bool, reason: Option<String>) {
        self.cancelled = _cancel;
        self.reason = reason;
    }
}

impl Event for InitEvent {
    fn cancellable(&self) -> bool {
        true
    }

    fn cancelled(&self) -> bool {
        self.cancelled
    }

    fn get_cancelled_reason(&self) -> Option<String> {
        self.reason.clone()
    }

    fn set_cancelled(&mut self, _cancel: bool, reason: Option<String>) {
        self.cancelled = _cancel;
        self.reason = reason;
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use event_bus::{subscribe_event, dispatch_event, EventBus, Event, EventResult};
    use event_bus::EventResult::{EvCancelled, EvPassed};
    use crate::events::InteractType::KEYBOARD;

    fn test_sub(event: &mut InteractEvent) {
        println!("Event called");
    }

    fn test_sub2(event: &mut InteractEvent) {
        println!("Event 2 called")
    }

    fn test_sub_init(event: &mut InitEvent) {
        println!("Event inti called");
    }

    fn test_sub_cancelled(event: &mut InitEvent) {
        println!("Event init cancel called");
        event.cancel(Option::from("Event init cancelled".to_string()));
    }

    #[test]
    fn event_test() {

        let mut test_bus = EventBus::new("test");

        subscribe_event!("test", test_sub);
        subscribe_event!("test", test_sub2);
        subscribe_event!("test", test_sub_init);
        subscribe_event!("test", test_sub_cancelled);

        let mut event = InteractEvent {
            interact: KEYBOARD(glfw::Key::B),
            cancelled: false,
            reason: None
        };

        let mut init_event = InitEvent {
            cancelled: false,
            reason: None
        };

        let result_interact: EventResult = dispatch_event!("test", &mut event);

        assert_eq!(result_interact, EvPassed);

        println!("calling other");

        let result_init = dispatch_event!("test", &mut init_event);

        assert_eq!(result_init, EvCancelled("Event init cancelled".to_string()))

    }

}