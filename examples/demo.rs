use cef::{
    CefApp, CefArgs, CefBrowser, CefBrowserSettings, CefBrowserView, CefClient, CefSettings,
    CefString, PanelDelegate, ViewDelegate, WindowDelegate,
};

#[derive(Debug, Clone, Copy)]
struct Application;

impl CefApp for Application {
    type BrowserProcess = ();
    type RenderProcess = ();
}

#[derive(Debug, Copy, Clone)]
struct DemoClient;

impl CefClient for DemoClient {
    type LifeSpan = ();
    type Render = ();
}

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
    fn on_window_created(&self, mut window: cef::CefWindow) {
        window.get_panel().add_child_view(self.browser_view.view());
        window.show();
    }

    fn on_window_destroyed(&self, _window: cef::CefWindow) {
        cef::quit_message_loop();
    }
}

fn main() {
    //let cef_path = std::path::PathBuf::from(std::env::var("CEF_PATH").unwrap())
    //    .canonicalize()
    //    .unwrap();
    let mut args = CefArgs::new(std::env::args());
    dbg!(&args);
    let app = Application;
    let mut settings = CefSettings::new();
    settings.root_cache_path = CefString::from("/tmp/demo").into();
    settings.no_sandbox = true;
    cef::execute_process(&mut args, Some(app)).unwrap();
    cef::initialize(&mut args, &settings, Some(app)).unwrap();

    let window_info = cef::CefWindowInfo::new();
    let browser_settings = CefBrowserSettings::new();
    let client = DemoClient;
    let url = CefString::new("https://www.google.com");

    let _brower_view =
        CefBrowserView::create(Some(client), &url, browser_settings.clone()).unwrap();
    //let delegate = DemoWindow { browser_view };

    //cef::create_top_level_window(delegate);
    CefBrowser::create(window_info, Some(client), url, browser_settings).unwrap();

    cef::run_message_loop();

    cef::shutdown();
}
