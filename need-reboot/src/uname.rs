use std::{ffi::CStr, io, mem};

use crate::Result;

mod c {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    const MEMBER_SIZE: usize = 65;

    #[derive(Debug, Copy, Clone)]
    #[repr(C)]
    pub(super) struct utsname {
        pub sysname: [i8; MEMBER_SIZE],
        pub nodename: [i8; MEMBER_SIZE],
        pub release: [i8; MEMBER_SIZE],
        pub version: [i8; MEMBER_SIZE],
        pub machine: [i8; MEMBER_SIZE],
        pub domainname: [i8; MEMBER_SIZE],
    }

    unsafe extern "C" {
        pub unsafe fn uname(buf: *mut utsname) -> i32;
    }
}

fn c_uname() -> io::Result<c::utsname> {
    let mut buf = mem::MaybeUninit::zeroed();

    let ret = unsafe { c::uname(buf.as_mut_ptr()) };
    if ret == 0 {
        Ok(unsafe { buf.assume_init() })
    } else {
        Err(io::Error::last_os_error())
    }
}

#[derive(Debug, Default)]
pub struct UtsName {
    pub sysname: String,
    pub nodename: String,
    pub release: String,
    pub version: String,
    pub machine: String,
}

fn get_string(c_raw_string: &[i8]) -> Result<String> {
    let raw_string: &[u8] =
        unsafe { std::slice::from_raw_parts(c_raw_string.as_ptr().cast(), c_raw_string.len()) };

    let s = CStr::from_bytes_until_nul(raw_string)?;

    Ok(s.to_str()?.to_owned())
}

impl UtsName {
    pub fn new() -> Result<Self> {
        let raw_uname = c_uname()?;
        let sysname = get_string(&raw_uname.sysname[..])?;
        let nodename = get_string(&raw_uname.nodename[..])?;
        let release = get_string(&raw_uname.release[..])?;
        let version = get_string(&raw_uname.version[..])?;
        let machine = get_string(&raw_uname.machine[..])?;

        Ok(Self {
            sysname,
            nodename,
            release,
            version,
            machine,
        })
    }
}
