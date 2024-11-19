use cef_sys::cef_command_handler_t;

crate::wrapper! {
    #[doc = "See [cef_command_handler_t] for more docs."]
    #[derive(Debug,Clone)]
    pub struct CommandHandler(cef_command_handler_t);
}

pub trait CommandCallback {}
