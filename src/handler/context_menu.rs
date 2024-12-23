use crate::{prelude::*, CefBrowser, CefFrame, CefMenuModel};

/// See [cef_context_menu_handler_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct ContextMenuHandler(cef_context_menu_handler_t);

pub trait ContextMenuCallback {}

/// See [cef_context_menu_handler_t] for more docs.
pub trait CefContextMenuHandler: Sized {
    fn on_before_context_menu(
        &self,
        browser: CefBrowser,
        frame: CefFrame,
        params: CefContextMenuParams,
        model: CefMenuModel,
    ) {
    }

    #[doc(hidden)]
    fn into_raw(self) -> *mut cef_context_menu_handler_t {
        extern "C" fn on_before_context_menu<I: CefContextMenuHandler>(
            self_: *mut _cef_context_menu_handler_t,
            browser: *mut _cef_browser_t,
            frame: *mut _cef_frame_t,
            params: *mut _cef_context_menu_params_t,
            model: *mut _cef_menu_model_t,
        ) {
            let object: &crate::rc::RcImpl<_, I> = crate::rc::RcImpl::get(self_);

            object.interface.on_before_context_menu(
                CefBrowser::from(browser),
                CefFrame::from(frame),
                CefContextMenuParams::from(params),
                CefMenuModel::from(model),
            );
        }

        let mut object: cef_context_menu_handler_t = unsafe { std::mem::zeroed() };
        object.on_before_context_menu = Some(on_before_context_menu::<Self>);

        crate::rc::RcImpl::new(object, self).cast()
    }
}

/// See [cef_context_menu_params_t] for more docs.
#[derive(Debug, Clone)]
#[wrapper]
pub struct CefContextMenuParams(cef_context_menu_params_t);
