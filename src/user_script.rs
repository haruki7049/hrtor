use crate::CommandStatus;

pub mod lua;

pub trait UserScript {
    fn init(&self);

    /// Handle request from user/
    ///
    /// # Returns
    /// - `Some(CommandStatus)` if the request is handled
    /// - `None` if the request is not handled
    fn request_handle(&self, request: &String) -> Option<CommandStatus>;
}
