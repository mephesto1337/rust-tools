#[allow(non_camel_case_types)]
pub type time_t = std::os::raw::c_long;
unsafe extern "C" {
    unsafe fn time(t: *mut time_t) -> time_t;
}

pub fn c_time() -> time_t {
    // SAFETY: time (3) returns the current timestamp and null can be passed as the first
    // parameter
    unsafe { time(std::ptr::null_mut()) }
}
