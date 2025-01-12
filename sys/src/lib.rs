#[allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    clippy::all
)]
mod bindings;
pub use bindings::*;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init() {
        use std::ptr::*;

        unsafe {
            assert_eq!(cef_initialize(null(), null(), null_mut(), null_mut()), 0);
        };
    }
}
