pub unsafe trait CefInterface: Sized + 'static {
    type VTable;
    type Super: CefInterface;

    fn as_base(&self) -> &Base {
        unsafe { core::mem::transmute(self) }
    }

    fn as_raw(&self) -> core::ptr::NonNull<Self::VTable>;

    const INNER_SIZE: usize = core::mem::size_of::<Self::VTable>();
}

#[repr(transparent)]
pub struct Base {
    ref_count: std::sync::atomic::AtomicU32,
}

impl Base {
    unsafe extern "C" fn add_ref(this: *mut cef_sys::cef_base_ref_counted_t) {
        let self_ = unsafe { core::mem::transmute::<_, &Self>(this) };
        <_ as BaseImpl>::add_ref(self_);
    }

    unsafe extern "C" fn release(this: *mut cef_sys::cef_base_ref_counted_t) -> i32 {
        let self_ = unsafe { core::mem::transmute::<_, &Self>(this) };
        let should_release = <_ as BaseImpl>::release(self_);
        if should_release {
            _ = Box::from(this);
        }
        should_release as _
    }

    unsafe extern "C" fn has_one_ref(this: *mut cef_sys::cef_base_ref_counted_t) -> i32 {
        let self_ = unsafe { core::mem::transmute::<_, &Self>(this) };
        <_ as BaseImpl>::has_one_ref(self_) as _
    }

    unsafe extern "C" fn has_at_least_one_ref(this: *mut cef_sys::cef_base_ref_counted_t) -> i32 {
        let self_ = unsafe { core::mem::transmute::<_, &Self>(this) };
        <_ as BaseImpl>::has_at_least_one_ref(self_) as _
    }
}

pub trait BaseImpl {
    unsafe fn add_ref(&self);
    unsafe fn release(&self) -> bool;
    unsafe fn has_one_ref(&self) -> bool;
    unsafe fn has_at_least_one_ref(&self) -> bool;
}

impl BaseImpl for Base {
    unsafe fn add_ref(&self) {
        ref_counted::add_ref(&self.ref_count);
    }

    unsafe fn has_one_ref(&self) -> bool {
        ref_counted::has_one_ref(&self.ref_count)
    }

    unsafe fn has_at_least_one_ref(&self) -> bool {
        ref_counted::has_at_least_one_ref(&self.ref_count)
    }

    unsafe fn release(&self) -> bool {
        ref_counted::release(&self.ref_count)
    }
}

impl Drop for Base {
    fn drop(&mut self) {
        unsafe {
            self.release();
        }
    }
}

unsafe impl CefInterface for Base {
    type VTable = cef_sys::cef_base_ref_counted_t;
    type Super = Base;

    fn as_raw(&self) -> core::ptr::NonNull<Self::VTable> {
        core::ptr::NonNull::new(Box::into_raw(Box::new(cef_sys::cef_base_ref_counted_t {
            size: Self::INNER_SIZE,
            add_ref: Some(Self::add_ref),
            release: Some(Self::release),
            has_one_ref: Some(Self::has_one_ref),
            has_at_least_one_ref: Some(Self::has_at_least_one_ref),
        })))
        .unwrap()
    }
}

pub(crate) mod ref_counted {
    use std::sync::atomic::{AtomicU32, Ordering};

    #[inline(always)]
    pub fn add_ref(ref_count: &AtomicU32) {
        ref_count.fetch_add(1, Ordering::Relaxed);
    }

    #[inline(always)]
    pub fn has_one_ref(ref_count: &AtomicU32) -> bool {
        if ref_count.load(Ordering::Acquire) == 1 {
            true
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn has_at_least_one_ref(ref_count: &AtomicU32) -> bool {
        if ref_count.load(Ordering::Acquire) >= 1 {
            true
        } else {
            false
        }
    }

    #[inline(always)]
    pub fn release(ref_count: &AtomicU32) -> bool {
        ref_count.fetch_sub(1, Ordering::AcqRel) != 1
    }
}

pub unsafe trait CefAbi: Sized {
    type Abi;

    fn get_abi(&self) -> Self::Abi;
}

unsafe impl CefAbi for i32 {
    type Abi = core::ffi::c_int;

    fn get_abi(&self) -> Self::Abi {
        *self
    }
}

unsafe impl<T: CefInterface> CefAbi for T {
    type Abi = core::ptr::NonNull<<T as CefInterface>::VTable>;
    fn get_abi(&self) -> Self::Abi {
        self.as_raw()
    }
}

unsafe impl<T> CefAbi for *mut T {
    type Abi = Self;
    fn get_abi(&self) -> Self::Abi {
        *self
    }
}

unsafe impl<T> CefAbi for *const T {
    type Abi = Self;
    fn get_abi(&self) -> Self::Abi {
        *self
    }
}

unsafe impl<T: CefInterface> CefAbi for Option<T> {
    type Abi = *mut <T as CefInterface>::VTable;
    fn get_abi(&self) -> Self::Abi {
        self.as_ref()
            .map(|p| p.as_raw().as_ptr())
            .unwrap_or(::core::ptr::null_mut())
    }
}
