use crate::prelude::*;

#[doc = "See [cef_command_handler_t] for more docs."]
#[derive(Debug, Clone)]
#[wrapper]
pub struct CommandHandler(cef_command_handler_t);

pub trait CommandCallback {}
