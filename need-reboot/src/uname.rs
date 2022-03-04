use std::io;
use std::mem;

use crate::Result;

mod c {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn c_uname() -> io::Result<c::utsname> {
    let mut buf: mem::MaybeUninit<c::utsname> = mem::MaybeUninit::zeroed();

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
    let raw_string: &[u8] = unsafe { std::mem::transmute(c_raw_string) };
    let null_byte_idx = raw_string
        .iter()
        .enumerate()
        .find_map(|(i, b)| if *b == 0 { Some(i + 1) } else { None })
        .unwrap_or(raw_string.len());
    Ok(
        std::ffi::CStr::from_bytes_with_nul(&raw_string[..null_byte_idx])?
            .to_str()?
            .to_owned(),
    )
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
