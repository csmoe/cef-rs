use crate::prelude::*;

#[doc = "See [cef_command_line_t] for more documentation."]
#[wrapper]
#[derive(Debug, Clone)]
pub struct CommandLine(cef_command_line_t);
