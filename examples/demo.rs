use cef::{
    args::Args, client::Client, string::CefString, App, BrowserSettings, BrowserView,
    PanelDelegate, Settings, ViewDelegate, WindowDelegate,
};

#[derive(Debug, Clone, Copy)]
struct Application;

impl App for Application {}

#[derive(Debug, Copy, Clone)]
struct DemoClient;

impl Client for DemoClient {}

#[derive(Debug)]
struct DemoWindow {
    browser_view: BrowserView,
}

impl ViewDelegate for DemoWindow {
    fn on_child_view_changed(&self, _view: cef::View, _added: bool, _child: cef::View) {
        // view.as_panel().map(|x| x.as_window().map(|w| w.close()));
    }
}
impl PanelDelegate for DemoWindow {}
impl WindowDelegate for DemoWindow {
    fn on_window_created(&self, window: cef::Window) {
        window
            .get_panel()
            .add_child_view(self.browser_view.get_view());
        window.show();
    }

    fn on_window_destroyed(&self, _window: cef::Window) {
        cef::quit_message_loop();
    }
}

fn main() {
    let cef_path = std::path::PathBuf::from(std::env::var("CEF_PATH").unwrap())
        .canonicalize()
        .unwrap();
    let mut args = Args::new(std::env::args());
    let app = Application;
    let mut settings = Settings::new();
    settings.resources_dir_path = cef_path.join("Resources").into();
    //settings.locales_dir_path = cef_path.join("Resources").into();
    //settings.framework_dir_path = cef_path.join("Release").into();
    cef::execute_process(&mut args, Some(app)).unwrap();
    cef::initialize(&mut args, &settings, Some(app)).unwrap();

    let window_info = cef::WindowInfo::new();
    let browser_settings = BrowserSettings::new();
    let client = DemoClient;
    let url = CefString::new("https://www.google.com");

    let browser_view =
        cef::create_browser_view(Some(client), &url, browser_settings.clone()).unwrap();
    let delegate = DemoWindow { browser_view };

    cef::create_top_level_window(delegate);
    //cef::create_browser(window_info, Some(client), url, browser_settings);

    cef::run_message_loop();

    cef::shutdown();
}
