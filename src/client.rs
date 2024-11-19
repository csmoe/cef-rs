use crate::handler::*;
use cef_sys::{
    cef_audio_handler_t, cef_client_t, cef_command_handler_t, cef_context_menu_handler_t,
    cef_dialog_handler_t, cef_display_handler_t, cef_download_handler_t, cef_drag_handler_t,
    cef_find_handler_t, cef_focus_handler_t, cef_frame_handler_t, cef_frame_t,
    cef_jsdialog_handler_t, cef_keyboard_handler_t, cef_life_span_handler_t, cef_load_handler_t,
    cef_permission_handler_t, cef_print_handler_t, cef_process_id_t, cef_process_message_t,
    cef_render_handler_t, cef_request_handler_t,
};

use crate::{rc::RcImpl, Browser};

/// Handle browser-instance-specific callbacks
///
/// See [cef_client_t] for more documentation.
pub trait Client: Sized {
    fn get_audio_handler(&self) -> Option<cef_audio_handler_t> {
        None
    }

    fn get_command_handler(&self) -> Option<cef_command_handler_t> {
        None
    }

    fn get_context_menu_handler(&self) -> Option<cef_context_menu_handler_t> {
        None
    }

    fn get_dialog_handler(&self) -> Option<cef_dialog_handler_t> {
        None
    }

    fn get_display_handler(&self) -> Option<cef_display_handler_t> {
        None
    }

    fn get_download_handler(&self) -> Option<cef_download_handler_t> {
        None
    }

    fn get_drag_handler(&self) -> Option<cef_drag_handler_t> {
        None
    }

    fn get_find_handler(&self) -> Option<cef_find_handler_t> {
        None
    }

    fn get_focus_handler(&self) -> Option<cef_focus_handler_t> {
        None
    }

    fn get_frame_handler(&self) -> Option<cef_frame_handler_t> {
        None
    }

    fn get_permission_handler(&self) -> Option<cef_permission_handler_t> {
        None
    }

    fn get_jsdialog_handler(&self) -> Option<cef_jsdialog_handler_t> {
        None
    }

    fn get_keyboard_handler(&self) -> Option<cef_keyboard_handler_t> {
        None
    }

    fn get_life_span_handler(&self) -> Option<cef_life_span_handler_t> {
        None
    }

    fn get_load_handler(&self) -> Option<cef_load_handler_t> {
        None
    }

    fn get_print_handler(&self) -> Option<cef_print_handler_t> {
        None
    }

    fn get_request_handler(&self) -> Option<cef_request_handler_t> {
        None
    }

    fn get_render_handler(&self) -> Option<cef_render_handler_t> {
        None
    }

    fn on_process_message_received(
        &self,
        _browser: Browser,
        _frame: cef_frame_t,
        _source_process: cef_process_id_t,
        _message: cef_process_message_t,
    ) -> bool {
        false
    }

    fn into_raw(self) -> *mut cef_client_t {
        let object: cef_client_t = unsafe { std::mem::zeroed() };

        RcImpl::new(object, self).cast()
    }
}

pub trait RenderCallback {}
pub trait RequestCallback {}
pub trait LifeSpanCallback {}
pub trait KeyBoardCallback {}
pub trait JsDiagCallback {}
pub trait PermissionCallback {}
pub trait FrameCallback {}
pub trait AudioCallback {}
pub trait CommandCallback {}
pub trait ContextMenuCallback {}
pub trait DialogCallback {}
pub trait DisplayCallback {}
pub trait DownloadCallback {}
pub trait DragCallback {}
pub trait FindCallback {}
pub trait FocusCallback {}

#[derive(Default)]
pub struct ClientBuilder<
    Render: RenderCallback,
    Request: RequestCallback,
    LifeSpan: LifeSpanCallback,
    KeyBoard: KeyBoardCallback,
    JsDiag: JsDiagCallback,
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
        LifeSpan: LifeSpanCallback,
        KeyBoard: KeyBoardCallback,
        JsDiag: JsDiagCallback,
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

    pub fn into_raw(self) -> *mut cef_sys::cef_client_t {
        let object: *mut cef_client_t = unsafe { std::mem::zeroed() };

        RcImpl::new(object, self).cast()
    }
}

/*
pub(crate) unsafe extern "C" fn get_audio_handler<I: Client>(
    self_: *mut cef_sys::cef_client_t,
) -> *mut cef_sys::cef_audio_handler_t {
    let obj: &mut RcImpl<_, I> = RcImpl::get(self_);
    obj.interface
        .get_audio_handler()
        .map(|h| h.into_raw())
        .unwrap_or(core::ptr::null_mut())
}
*/
