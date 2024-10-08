fn main() {
    unsafe {
        libz_sys::deflate(core::ptr::null_mut(), libz_sys::Z_NO_FLUSH);
    }
}
