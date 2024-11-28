use crate::wrapper;
use cef_sys::cef_frame_t;
use cef_wrapper_macro::wrapper_methods;
use crate::CefString;

wrapper! {
    #[doc = "See [cef_frame_t] for more details."]
    #[derive(Debug, Clone)]
    pub struct Frame(cef_frame_t);
}

impl Frame {
    wrapper_methods!(
        /// See [cef_frame_t::is_valid]
        fn is_valid(&self) -> bool;

        /// See [cef_frame_t::undo]
        fn undo(&self);

        /// See [cef_frame_t::redo]
        fn redo(&self);

        /// See [cef_frame_t::cut]
        fn cut(&self);

        /// See [cef_frame_t::copy]
        fn copy(&self);

        /// See [cef_frame_t::paste]
        fn paste(&self);

        /// See [cef_frame_t::del]
        fn del(&self);

        /// See [cef_frame_t::select_all]
        fn select_all(&self);

        /// See [cef_frame_t::view_source]
        fn view_source(&self);

        /// See [cef_frame_t::get_source]
        fn get_source(&self, visitor: crate::StringVisitor);

        /// See [cef_frame_t::get_text]
        fn get_text(&self, visitor: crate::StringVisitor);

        /// See [cef_frame_t::load_request]
        fn load_request(&self, request: crate::Request);

        /// See [cef_frame_t::load_url]
        fn load_url(&self, url: CefString) {
        self.0.load_url.map(|f| unsafe {
            f(self.0.get_this(), std::ptr::from_ref(&CefString::as_raw()))
        })
    }

        /// See [cef_frame_t::execute_java_script]
        fn execute_java_script(&self, code: &str, script_url: &str, start_line: i32);

        /// See [cef_frame_t::is_main]
        fn is_main(&self) -> bool;

        /// See [cef_frame_t::is_focused]
        fn is_focused(&self) -> bool;

        /// See [cef_frame_t::get_name]
        fn get_name(&self) -> Option<String>;

        /// See [cef_frame_t::get_identifier]
        fn get_identifier(&self) -> Option<String>;

        /// See [cef_frame_t::get_parent]
        fn get_parent(&self) -> crate::Frame;

        /// See [cef_frame_t::get_url]
        fn get_url(&self) -> CefString;

        /// See [cef_frame_t::get_browser]
        fn get_browser(&self) -> crate::Browser;

        // See [cef_frame_t::get_v8context]
        //fn get_v8context(&self) -> crate::V8Context;

        /// See [cef_frame_t::visit_dom]
        fn visit_dom(&self, visitor: crate::DOMVisitor);

        /// See [cef_frame_t::create_urlrequest]
        fn create_urlrequest(
            &self,
            request: crate::Request,
            client: crate::URLRequestClient,
        ) -> Option<crate::URLRequest>;

        /// See [cef_frame_t::send_process_message]
        fn send_process_message(
            &self,
            target_process: cef_process_id_t,
            message: crate::ProcessMessage,
        );
    );
}
