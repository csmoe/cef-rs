use crate::handler::*;
use cef_sys::{cef_client_t, cef_frame_t, cef_process_id_t, cef_process_message_t};

use crate::{rc::RcImpl, Browser};

/// Handle browser-instance-specific callbacks
///
/// See [cef_client_t] for more documentation.
pub trait Client: Sized {
    /// See [cef_client_t::get_audio_handler]
    fn get_audio_handler(&self) -> Option<AudioHandler> {
        None
    }

    /// See [cef_client_t::get_render_handler]
    fn get_command_handler(&self) -> Option<CommandHandler> {
        None
    }

    /// See [cef_client_t::get_request_handler]
    fn get_context_menu_handler(&self) -> Option<ContextMenuHandler> {
        None
    }

    /// See [cef_client_t::get_request_handler]
    fn get_dialog_handler(&self) -> Option<DialogHandler> {
        None
    }

    /// See [cef_client_t::get_display_handler]
    fn get_display_handler(&self) -> Option<DisplayHandler> {
        None
    }

    /// See [cef_client_t::get_download_handler]
    fn get_download_handler(&self) -> Option<DownloadHandler> {
        None
    }

    /// See [cef_client_t::get_drag_handler]
    fn get_drag_handler(&self) -> Option<DragHandler> {
        None
    }

    /// See [cef_client_t::get_find_handler]
    fn get_find_handler(&self) -> Option<FindHandler> {
        None
    }

    /// See [cef_client_t::get_focus_handler]
    fn get_focus_handler(&self) -> Option<FocusHandler> {
        None
    }

    /// See [cef_client_t::get_frame_handler]
    fn get_frame_handler(&self) -> Option<FrameHandler> {
        None
    }

    /// See [cef_client_t::get_permission_handler]
    fn get_permission_handler(&self) -> Option<PermissionHandler> {
        None
    }

    /// See [cef_client_t::get_jsdialog_handler]
    fn get_jsdialog_handler(&self) -> Option<JsDialogHandler> {
        None
    }

    /// See [cef_client_t::get_keyboard_handler]
    fn get_keyboard_handler(&self) -> Option<KeyboardHandler> {
        None
    }

    /// See [cef_client_t::get_life_span_handler]
    fn get_life_span_handler(&self) -> Option<BrowerLifeSpanHandler> {
        None
    }

    /// See [cef_client_t::get_load_handler]
    fn get_load_handler(&self) -> Option<LoadHandler> {
        None
    }

    /// See [cef_client_t::get_print_handler]
    fn get_print_handler(&self) -> Option<PrintHandler> {
        None
    }

    /// See [cef_client_t::get_request_handler]
    fn get_request_handler(&self) -> Option<RequestHandler> {
        None
    }

    /// See [cef_client_t::get_render_handler]
    fn get_render_handler(&self) -> Option<RenderHandler> {
        None
    }

    /// See [cef_client_t::on_process_message_received]
    fn on_process_message_received(
        &self,
        _browser: Browser,
        _frame: cef_frame_t,
        _source_process: cef_process_id_t,
        _message: cef_process_message_t,
    ) -> bool {
        false
    }

    #[doc(hidden)]
    fn into_raw(self) -> *mut cef_client_t {
        let mut object: cef_client_t = unsafe { std::mem::zeroed() };
        object.get_drag_handler = Some(get_drag_handler::<Self>);
        object.get_find_handler = Some(get_find_handler::<Self>);
        object.get_load_handler = Some(get_load_handler::<Self>);
        object.get_audio_handler = Some(get_audio_handler::<Self>);
        object.get_focus_handler = Some(get_focus_handler::<Self>);
        object.get_frame_handler = Some(get_frame_handler::<Self>);
        object.get_print_handler = Some(get_print_handler::<Self>);
        object.get_dialog_handler = Some(get_dialog_handler::<Self>);
        object.get_render_handler = Some(get_render_handler::<Self>);
        object.get_command_handler = Some(get_command_handler::<Self>);
        object.get_display_handler = Some(get_display_handler::<Self>);
        object.get_request_handler = Some(get_request_handler::<Self>);
        object.get_download_handler = Some(get_download_handler::<Self>);
        object.get_permission_handler = Some(get_permission_handler::<Self>);
        object.get_context_menu_handler = Some(get_context_menu_handler::<Self>);

        RcImpl::new(object, self).cast()
    }
}

#[derive(Default)]
pub struct ClientBuilder<
    Render: RenderCallback,
    Request: RequestCallback,
    LifeSpan: BrowserLifeSpanCallback,
    KeyBoard: KeyboardCallback,
    JsDiag: JsDialogCallback,
    Permission: PermissionCallback,
    Frame: FrameCallback,
    Audio: AudioCallback,
    Command: CommandCallback,
    ContextMenu: ContextMenuCallback,
    Dialog: DialogCallback,
    Display: DisplayCallback,
    Download: DownloadCallback,
    Drag: DragCallback,
    Find: FindCallback,
    Focus: FocusCallback,
> {
    render: Option<Render>,
    request: Option<Request>,
    life_span: Option<LifeSpan>,
    key_board: Option<KeyBoard>,
    js_diag: Option<JsDiag>,
    permission: Option<Permission>,
    frame: Option<Frame>,
    audio: Option<Audio>,
    command: Option<Command>,
    context_menu: Option<ContextMenu>,
    dialog: Option<Dialog>,
    display: Option<Display>,
    download: Option<Download>,
    drag: Option<Drag>,
    find: Option<Find>,
    focus: Option<Focus>,
}

impl<
        Render: RenderCallback,
        Request: RequestCallback,
        LifeSpan: BrowserLifeSpanCallback,
        KeyBoard: KeyboardCallback,
        JsDiag: JsDialogCallback,
        Permission: PermissionCallback,
        Frame: FrameCallback,
        Audio: AudioCallback,
        Command: CommandCallback,
        ContextMenu: ContextMenuCallback,
        Dialog: DialogCallback,
        Display: DisplayCallback,
        Download: DownloadCallback,
        Drag: DragCallback,
        Find: FindCallback,
        Focus: FocusCallback,
    >
    ClientBuilder<
        Render,
        Request,
        LifeSpan,
        KeyBoard,
        JsDiag,
        Permission,
        Frame,
        Audio,
        Command,
        ContextMenu,
        Dialog,
        Display,
        Download,
        Drag,
        Find,
        Focus,
    >
{
    pub fn with_render_callback(mut self, render: Render) -> Self {
        self.render = Some(render);
        self
    }

    pub fn with_request_callback(mut self, request: Request) -> Self {
        self.request = Some(request);
        self
    }

    pub fn with_life_span_callback(mut self, life_span: LifeSpan) -> Self {
        self.life_span = Some(life_span);
        self
    }

    pub fn with_key_board_callback(mut self, key_board: KeyBoard) -> Self {
        self.key_board = Some(key_board);
        self
    }

    pub fn with_js_diag_callback(mut self, js_diag: JsDiag) -> Self {
        self.js_diag = Some(js_diag);
        self
    }

    pub fn with_permission_callback(mut self, permission: Permission) -> Self {
        self.permission = Some(permission);
        self
    }

    pub fn with_frame_callback(mut self, frame: Frame) -> Self {
        self.frame = Some(frame);
        self
    }

    pub fn with_audio_callback(mut self, audio: Audio) -> Self {
        self.audio = Some(audio);
        self
    }

    pub fn with_command_callback(mut self, command: Command) -> Self {
        self.command = Some(command);
        self
    }

    pub fn with_context_menu_callback(mut self, context_menu: ContextMenu) -> Self {
        self.context_menu = Some(context_menu);
        self
    }

    pub fn with_dialog_callback(mut self, dialog: Dialog) -> Self {
        self.dialog = Some(dialog);
        self
    }

    pub fn with_display_callback(mut self, display: Display) -> Self {
        self.display = Some(display);
        self
    }

    pub fn with_download_callback(mut self, download: Download) -> Self {
        self.download = Some(download);
        self
    }

    pub fn with_drag_callback(mut self, drag: Drag) -> Self {
        self.drag = Some(drag);
        self
    }

    pub fn with_find_callback(mut self, find: Find) -> Self {
        self.find = Some(find);
        self
    }

    pub fn with_focus_callback(mut self, focus: Focus) -> Self {
        self.focus = Some(focus);
        self
    }
}

pub(crate) unsafe extern "C" fn get_audio_handler<I: Client>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_audio_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_audio_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_render_handler<I: Client>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_render_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_render_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_request_handler<I: Client>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_request_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_request_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_life_span_handler<I: Client>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_life_span_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_life_span_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_keyboard_handler<I: Client>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_keyboard_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_keyboard_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_js_dialog_handler<I: Client>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_jsdialog_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_jsdialog_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_permission_handler<I: Client>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_permission_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_permission_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_frame_handler<I: Client>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_frame_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_frame_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_command_handler<I: Client>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_command_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_command_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_context_menu_handler<I: Client>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_context_menu_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_context_menu_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_dialog_handler<I: Client>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_dialog_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_dialog_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_display_handler<I: Client>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_display_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_display_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_download_handler<I: Client>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_download_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_download_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_drag_handler<I: Client>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_drag_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_drag_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_find_handler<I: Client>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_find_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_find_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_focus_handler<I: Client>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_focus_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_focus_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_load_handler<I: Client>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_load_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_load_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}

pub(crate) unsafe extern "C" fn get_print_handler<I: Client>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_print_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_print_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}
