use event_bus::Event;

pub struct RefreshEvent {
    cancelled: bool,
    reason: Option<String>
}

impl Event for RefreshEvent {

    fn cancellable(&self) -> bool {
        true
    }

    fn cancel(&mut self, reason: Option<String>) {
        self.cancelled = true;
        self.reason = reason;
    }

}

pub struct ClearEvent {
    cancelled: bool,
    reason: Option<String>
}

impl Event for ClearEvent {

    fn cancellable(&self) -> bool {
        true
    }

    fn cancel(&mut self, reason: Option<String>) {
        self.cancelled = true;
        self.reason = reason
    }

}