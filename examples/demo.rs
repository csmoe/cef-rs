#[cfg(target_os = "macos")]
use cef::LibraryLoader;
use cef::{
    CefApp, CefArgs, CefBrowser, CefBrowserSettings, CefBrowserView, CefClient,
    CefContextMenuHandler, CefLifeSpanHandler, CefLoadHandler, CefSettings, CefString,
    PanelDelegate, ViewDelegate, WindowDelegate,
};
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
struct Application;

impl CefApp for Application {
    type BrowserProcess = ();
    type RenderProcess = ();
}

#[derive(Debug, Copy, Clone)]
struct DemoClient;

impl CefClient for DemoClient {
    type LifeSpan = LifeSpanHandler;
    type Render = ();
    type Load = LoadHandler;
    type ContextMenu = ContextMenu;
}

struct ContextMenu;
impl CefContextMenuHandler for ContextMenu {}

struct LifeSpanHandler;
impl CefLifeSpanHandler for LifeSpanHandler {}

struct LoadHandler;
impl CefLoadHandler for LoadHandler {}

#[derive(Debug)]
struct DemoWindow {
    browser_view: CefBrowserView,
}

impl ViewDelegate for DemoWindow {
    fn on_child_view_changed(&self, _view: cef::CefView, _added: bool, _child: cef::CefView) {
        // view.as_panel().map(|x| x.as_window().map(|w| w.close()));
    }
}
impl PanelDelegate for DemoWindow {}
impl WindowDelegate for DemoWindow {
    fn on_window_created(&self, window: cef::CefWindow) {}

    fn on_window_destroyed(&self, _window: cef::CefWindow) {
        cef::quit_message_loop();
    }
}

fn main() {
    #[cfg(target_os = "macos")]
    {
        let loader = LibraryLoader::new(&std::env::current_exe().unwrap(), false);
        loader.load().unwrap();
    }
    let mut args = CefArgs::new(std::env::args());
    let app = Application;
    let mut settings = CefSettings::new();
    settings.root_cache_path = CefString::from("/tmp/demo").into();
    cef::execute_process(&mut args, Some(Arc::new(app))).unwrap();
    cef::initialize(&mut args, &settings, Some(Arc::new(app))).unwrap();

    let window_info = cef::CefWindowInfo::new();
    let browser_settings = CefBrowserSettings::new();
    let client = DemoClient;
    let url = CefString::new("https://www.example.com");

    CefBrowser::create(window_info, Some(client), url, browser_settings, None).unwrap();

    cef::run_message_loop();

    cef::shutdown();
}
