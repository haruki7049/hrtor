pub mod lua;

pub trait UserScript {
    fn init(&self) -> ();
}
