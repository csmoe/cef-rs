use std::time::Duration;

use crate::prelude::*;
use crate::rc::RcImpl;
use crate::CefThreadId;

pub fn currently_on(id: CefThreadId) -> bool {
    unsafe { cef_currently_on(id) > 0 }
}

pub fn post_task<F: FnOnce()>(id: CefThreadId, task: F) -> crate::error::Result<()> {
    if currently_on(id) {
        task();
        return Ok(());
    }

    let task = TaskWrapper { func: Some(task) };
    let ret = unsafe { cef_post_task(id, task.into_raw()) };
    if ret > 0 {
        Ok(())
    } else {
        Err(Error::CannotPostTask(id as _))
    }
}

pub fn post_delayed_task<F: FnOnce()>(
    thread_id: CefThreadId,
    task: F,
    delayed_ms: Duration,
) -> crate::error::Result<()> {
    if currently_on(thread_id) {
        task();
        return Ok(());
    }
    let task = TaskWrapper { func: Some(task) };
    let ret =
        unsafe { cef_post_delayed_task(thread_id, task.into_raw(), delayed_ms.as_millis() as _) };
    if ret > 0 {
        Ok(())
    } else {
        Err(Error::CannotPostTask(thread_id as _))
    }
}

pub struct TaskWrapper<F: FnOnce()> {
    func: Option<F>,
}

impl<F: FnOnce()> TaskWrapper<F> {
    unsafe extern "C" fn execute(this: *mut cef_task_t) {
        let task: &mut RcImpl<_, Self> = RcImpl::get(this);
        if let Some(func) = task.interface.func.take() {
            (func)();
        }
    }

    fn into_raw(self) -> *mut cef_task_t {
        let mut object: cef_task_t = unsafe { std::mem::zeroed() };
        object.execute = Some(Self::execute);
        RcImpl::new(object, self).cast()
    }
}
