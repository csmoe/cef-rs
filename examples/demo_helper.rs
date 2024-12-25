use cef::{CefApp, CefArgs, CefClient, CefContextMenuHandler, CefSettings, LibraryLoader};

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
    type ContextMenu = ContextMenu;
}
struct ContextMenu;
impl CefContextMenuHandler for ContextMenu {}

fn main() {
    {
        let loader = LibraryLoader::new(&std::env::current_exe().unwrap(), true);
        loader.load().unwrap();
    }

    let mut args = CefArgs::new(std::env::args());
    let app = Application;
    let settings = CefSettings::new();
    cef::execute_process(&mut args, Some(app)).unwrap();
    cef::initialize(&mut args, &settings, Some(app)).unwrap();

    cef::run_message_loop();

    cef::shutdown();
}
