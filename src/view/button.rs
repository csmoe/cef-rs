use cef_sys::{cef_button_t, cef_label_button_t};

crate::wrapper!(
    #[doc = "See [cef_button_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct Button(cef_button_t);
);

crate::wrapper!(
    #[doc = "See [cef_label_button_t] for more documentation."]
    #[derive(Debug, Clone)]
    pub struct LabelButton(cef_label_button_t);
);
