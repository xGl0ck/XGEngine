
pub enum InteractType {
    KEYBOARD(u32), MOUSE(u8, f64, f64)
}

pub struct InitEvent(InteractType);

pub struct ShutdownEvent();

pub struct InteractEvent();
