use std::ptr::null_mut;

use cef::{
    args::Args, client::Client, App, BrowserView, IntoRaw, PanelDelegate, Settings, ViewDelegate,
    WindowDelegate,
};

#[derive(Debug, Clone, Copy)]
struct Application;

impl App for Application {}

#[derive(Debug, Copy, Clone)]
struct DemoClient;

impl Client for DemoClient {}

fn main() {
    let mut args = Args::new(std::env::args());
    let app = Application;
    let settings = Settings::new();
    cef::execute_process(&mut args, Some(app)).unwrap();
    cef::initialize(&mut args, &settings, Some(app)).unwrap();

    cef::run_message_loop();

    cef::shutdown();
}
