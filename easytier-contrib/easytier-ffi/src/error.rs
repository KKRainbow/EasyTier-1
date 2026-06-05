use std::{
    cell::RefCell,
    ffi::{CString, c_char},
};

thread_local! {
    // # Thread Safety
    // set_error_msg and get_error_msg must be called on the same thread to
    // get correct error. And since `Handle::block_on` polls the top-level
    // future on the calling thread, set_error_msg always runs on the same
    // thread as the corresponding get_error_msg.
    static ERROR_MSG: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

pub(crate) fn set_error_msg(msg: &str) {
    ERROR_MSG.with(|cell| {
        let mut buf = cell.borrow_mut();
        buf.clear();
        buf.extend_from_slice(msg.as_bytes());
    });
}

pub(crate) unsafe fn get_error_msg(out: *mut *const c_char) {
    let cstr = ERROR_MSG.with(|cell| {
        let buf = cell.borrow();
        if buf.is_empty() {
            None
        } else {
            CString::new(&buf[..]).ok()
        }
    });
    unsafe {
        *out = match cstr {
            Some(s) => s.into_raw() as *const c_char,
            None => std::ptr::null(),
        };
    }
}

pub(crate) fn free_string(s: *const c_char) {
    if s.is_null() {
        return;
    }
    unsafe {
        let _ = CString::from_raw(s as *mut c_char);
    }
}
