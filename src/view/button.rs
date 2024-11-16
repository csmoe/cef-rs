crate::wrapper!(
    #[doc = "See `[cef_sys::cef_button_t]` for more documentation."]
    #[derive(Debug, Clone)]
    pub struct Button(cef_sys::cef_button_t);
);

crate::wrapper!(
    #[doc = "See `[cef_sys::cef_label_button_t]` for more documentation."]
    #[derive(Debug, Clone)]
    pub struct LabelButton(cef_sys::cef_label_button_t);
);

crate::wrapper!(
    #[doc = "See `[cef_sys::cef_menu_button_t]` for more documentation."]
    #[derive(Debug, Clone)]
    pub struct MenuButton(cef_sys::cef_menu_button_t);
);

crate::convert_view! {
    (Button, as_label_button, LabelButton),
    (LabelButton, as_menu_button, MenuButton)
}
