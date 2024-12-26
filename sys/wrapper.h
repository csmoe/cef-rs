#ifndef CEF_RUST_SYS_WRAPPER_H
#define CEF_RUST_SYS_WRAPPER_H

#ifdef __APPLE__
#include "include/wrapper/cef_library_loader.h"
#include "include/cef_sandbox_mac.h"
#endif

#ifdef _WIN32
#include "include/cef_sandbox_win.h"
#endif

#include "include/capi/cef_base_capi.h"
#include "include/capi/cef_app_capi.h"

#include "include/capi/cef_browser_capi.h"
#include "include/capi/cef_browser_process_handler_capi.h"

#include "include/capi/cef_client_capi.h"

#include "include/capi/cef_command_line_capi.h"
#include "include/capi/cef_command_handler_capi.h"

#include "include/capi/cef_request_capi.h"
#include "include/capi/cef_request_context_handler_capi.h"
#include "include/capi/cef_request_handler_capi.h"

#include "include/capi/cef_resource_bundle_capi.h"
#include "include/capi/cef_resource_bundle_handler_capi.h"
#include "include/capi/cef_response_capi.h"
#include "include/capi/cef_response_filter_capi.h"

#include "include/capi/cef_resource_handler_capi.h"
#include "include/capi/cef_resource_request_handler_capi.h"

// views
#include "include/capi/views/cef_browser_view_capi.h"
#include "include/capi/views/cef_browser_view_delegate_capi.h"
#include "include/capi/views/cef_window_capi.h"
#include "include/capi/views/cef_window_delegate_capi.h"

#include "include/capi/views/cef_button_capi.h"
#include "include/capi/views/cef_button_delegate_capi.h"
#include "include/capi/views/cef_label_button_capi.h"
#include "include/capi/views/cef_menu_button_capi.h"
#include "include/capi/views/cef_menu_button_delegate_capi.h"

#include "include/capi/views/cef_panel_capi.h"
#include "include/capi/views/cef_panel_delegate_capi.h"

#include "include/capi/views/cef_scroll_view_capi.h"

#include "include/capi/views/cef_textfield_capi.h"
#include "include/capi/views/cef_textfield_delegate_capi.h"

#include "include/capi/views/cef_box_layout_capi.h"
#include "include/capi/views/cef_fill_layout_capi.h"
#include "include/capi/views/cef_layout_capi.h"

#include "include/capi/views/cef_display_capi.h"
#include "include/capi/views/cef_overlay_controller_capi.h"

// v8
#include "include/capi/cef_v8_capi.h"

#endif
