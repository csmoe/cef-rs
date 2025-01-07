#[cfg(target_os = "macos")]
use cef::LibraryLoader;
use cef::{CefApp, CefArgs, CefSettings};

#[derive(Debug, Clone, Copy)]
struct Application;

impl CefApp for Application {
    type RenderProcess = ();
    type BrowserProcess = ();
}

fn main() {
    #[cfg(target_os = "macos")]
    {
        let loader = LibraryLoader::new(&std::env::current_exe().unwrap(), true);
        loader.load().unwrap();
    }

    let mut args = CefArgs::new(std::env::args());
    let app = Application;
    let settings = CefSettings::new();
    cef::execute_process(&mut args, Some(app.into())).unwrap();
    cef::initialize(&mut args, &settings, Some(app.into())).unwrap();

    cef::run_message_loop();

    cef::shutdown();
}
