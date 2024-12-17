use cef::{CefApp, CefArgs, CefClient, CefSettings};

#[derive(Debug, Clone, Copy)]
struct Application;

impl CefApp for Application {
    type RenderProcess = ();
    type BrowserProcess = ();
}

#[derive(Debug, Copy, Clone)]
struct DemoClient;

impl CefClient for DemoClient {
    type Render = ();
    type LifeSpan = ();
}

fn main() {
    let mut args = CefArgs::new(std::env::args());
    let app = Application;
    let settings = CefSettings::new();
    cef::execute_process(&mut args, Some(app)).unwrap();
    cef::initialize(&mut args, &settings, Some(app)).unwrap();

    cef::run_message_loop();

    cef::shutdown();
}
