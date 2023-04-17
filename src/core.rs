use std::sync::{Arc, Mutex, MutexGuard};
use event_bus::dispatch_event;
use crate::events::InitEvent;

pub trait Initializer {

    fn init(&mut self) -> bool;

}

pub struct AppBoostrap {
    init_pipeline: Arc<Mutex<Vec<Box<dyn Initializer>>>>
}

impl AppBoostrap {

    pub fn new() -> Self {
        Self {
            init_pipeline: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub fn boostrap(&mut self) -> bool {

        let mut guard = match self.init_pipeline.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };

        for initializer in guard.iter_mut() {
            let result: bool = initializer.init();
            if !result {
                panic!("The error occurred in init pipeline")
            }
        }

        let mut event: InitEvent = InitEvent::new();

        dispatch_event!("engine", &mut event);

        true
    }

    fn add_initializer(&mut self, initializer: Box<dyn Initializer>) {

        let mut guard: MutexGuard<Vec<Box<dyn Initializer>>> = match self.init_pipeline.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner()
        };

        guard.push(initializer);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestInit {
        initialized: bool
    }
    struct TestInit2 {
        initialized: bool
    }

    impl Initializer for TestInit {

        fn init(&mut self) -> bool {
            println!("This is test init");
            self.initialized = true;
            true
        }
    }

    impl Initializer for TestInit2 {

        fn init(&mut self) -> bool {
            println!("This is test init2");
            self.initialized = true;
            true
        }
    }

    #[test]
    fn init_test() {

        let mut boostrap = AppBoostrap::new();

        let init = TestInit {
            initialized: false
        };
        let init_b = TestInit2 {
            initialized: false
        };

        boostrap.add_initializer(Box::new(init));
        boostrap.add_initializer(Box::new(init_b));

        let result = boostrap.boostrap();

        assert_eq!(true, result);
    }
}