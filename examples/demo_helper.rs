use std::ptr::null_mut;

use cef::{args::CefArgs, client::CefClient, CefApp, Settings, ViewDelegate, WindowDelegate};

#[derive(Debug, Clone, Copy)]
struct Application;

impl CefApp for Application {}

#[derive(Debug, Copy, Clone)]
struct DemoClient;

impl CefClient for DemoClient {}

fn main() {
    let mut args = CefArgs::new(std::env::args());
    let app = Application;
    let settings = Settings::new();
    cef::execute_process(&mut args, Some(app)).unwrap();
    cef::initialize(&mut args, &settings, Some(app)).unwrap();

    cef::run_message_loop();

    cef::shutdown();
}
