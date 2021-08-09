extern crate libc;

use std::ffi::{CStr, CString};

#[no_mangle]
pub extern "C" fn rusty_call(_buffer: *const libc::c_char, _len: libc::size_t) {
    let buf = unsafe{
        let mut buf = Vec::with_capacity(4096);
        let s = CStr::from_ptr(_buffer);
        buf.set_len(s.to_bytes().len());
        CString::new(buf)
    };
    let buf_str = buf.unwrap();
    println!("buf str: {:}")
}
