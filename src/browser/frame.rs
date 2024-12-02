use crate::prelude::*;

#[doc = "See [cef_frame_t] for more details."]
#[derive(Debug, Clone)]
#[wrapper]
pub struct Frame(cef_frame_t);

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
        /// fn get_source(&self, visitor: crate::StringVisitor);

        /// See [cef_frame_t::get_text]
        ///fn get_text(&self, visitor: crate::StringVisitor);

        /// See [cef_frame_t::load_request]
        fn load_request(&self, request: crate::net::Request);

        /// See [cef_frame_t::load_url]
        fn load_url(&self, url: &str) {
            self.0.load_url.map(|f| unsafe {
                f(
                    self.0.get_this(),
                    std::ptr::from_ref(&<_ as Into<CefString>>::into(url).as_raw()),
                )
            })
        }

        /// See [cef_frame_t::execute_java_script]
        fn execute_java_script(&self, code: &str, script_url: &str, start_line: i32) {
            self.0.execute_java_script.map(|f| unsafe {
                f(
                    self.0.get_this(),
                    std::ptr::from_ref(&<_ as Into<CefString>>::into(code).as_raw()),
                    std::ptr::from_ref(&<_ as Into<CefString>>::into(script_url).as_raw()),
                    start_line,
                )
            })
        }

        /// See [cef_frame_t::is_main]
        fn is_main(&self) -> bool;

        /// See [cef_frame_t::is_focused]
        fn is_focused(&self) -> bool;

        /// See [cef_frame_t::get_name]
        fn get_name(&self) -> CefString {
            self.0
                .get_name
                .and_then(|f| unsafe { CefString::from_raw(f(self.0.get_this())) })
        }

        /// See [cef_frame_t::get_identifier]
        fn get_identifier(&self) -> CefString {
            self.0
                .get_identifier
                .and_then(|f| unsafe { CefString::from_raw(f(self.0.get_this())) })
        }

        /// See [cef_frame_t::get_parent]
        fn get_parent(&self) -> Frame {
            self.0.get_parent.and_then(|f| unsafe {
                let f = f(self.0.get_this());
                if f.is_null() {
                    None
                } else {
                    Some(Frame::from_raw(f))
                }
            })
        }

        /// See [cef_frame_t::get_url]
        fn get_url(&self) -> CefString {
            self.0
                .get_url
                .and_then(|f| unsafe { CefString::from_raw(f(self.0.get_this())) })
        }

        /// See [cef_frame_t::get_browser]
        fn get_browser(&self) -> crate::Browser {
            self.0.get_browser.and_then(|f| unsafe {
                let f = f(self.0.get_this());
                if f.is_null() {
                    None
                } else {
                    Some(crate::Browser::from_raw(f))
                }
            })
        }

        // See [cef_frame_t::get_v8context]
        //fn get_v8context(&self) -> crate::V8Context;

        /// See [cef_frame_t::visit_dom]
        // fn visit_dom(&self, visitor: crate::DOMVisitor);

        /// See [cef_frame_t::create_urlrequest]
        //fn create_urlrequest(
        //    &self,
        //    request: crate::Request,
        //    client: crate::URLRequestClient,
        //) -> Option<crate::URLRequest>;

        /// See [cef_frame_t::send_process_message]
        fn send_process_message(
            &self,
            target_process: crate::ProcessId,
            message: crate::ProcessMessage,
        ) {
            self.0
                .send_process_message
                .map(|f| unsafe { f(self.0.get_this(), target_process, message.into_raw()) })
        }
    );
}
