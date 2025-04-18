use clap::Parser;
use std::{
    ffi::{CStr, CString},
    io,
    mem::MaybeUninit,
    process::exit,
    ptr,
};

mod error;
mod process;

pub use error::{Error, Result};
use process::Process;

#[derive(Parser, Debug)]
struct Options {
    #[arg(
        short = 'q',
        long = "quiet",
        help = "Only shows PID. Otherwise outputs usernames and commandlines"
    )]
    quiet: bool,

    #[arg(
        short = 'u',
        long = "user",
        help = "Username of UID to filter processes"
    )]
    username: Option<String>,
}

fn get_username(uid: u32) -> Result<String> {
    let mut pwd = MaybeUninit::uninit();
    let mut result = ptr::null_mut();
    let mut buf = [0; 1024];
    let ret = unsafe {
        libc::getpwuid_r(
            uid,
            pwd.as_mut_ptr(),
            buf.as_mut_ptr(),
            buf.len(),
            ptr::addr_of_mut!(result),
        )
    };

    if result.is_null() {
        if ret == 0 {
            Ok(format!("{uid}"))
        } else {
            Err(Error::GetPwuid(io::Error::last_os_error()))
        }
    } else {
        let pwd = unsafe { pwd.assume_init() };
        // SAFETY:
        // * pwd contains only valid pointers to `buf` memory
        // * strings are valid ASCII nul-terminated strings
        let name = unsafe { CStr::from_ptr(pwd.pw_name) }
            .to_str()
            .expect("pw_name should be valid UTF-8");
        Ok(name.into())
    }
}

fn get_uid(username: &str) -> Result<u32> {
    // Fast case username represents an UID
    if let Ok(uid) = username.parse() {
        return Ok(uid);
    }

    let mut pwd = MaybeUninit::uninit();
    let mut result = ptr::null_mut();
    let mut buf = [0; 1024];
    let name = CString::new(username)?;
    let ret = unsafe {
        libc::getpwnam_r(
            name.as_ptr(),
            pwd.as_mut_ptr(),
            buf.as_mut_ptr(),
            buf.len(),
            ptr::addr_of_mut!(result),
        )
    };

    if ret == 0 {
        if result.is_null() {
            Err(Error::UnknownUser(username.into()))
        } else {
            let pwd = unsafe { pwd.assume_init() };
            // SAFETY:
            // * pwd contains only valid pointers to `buf` memory
            // * strings are valid ASCII nul-terminated strings
            Ok(pwd.pw_uid)
        }
    } else {
        Err(Error::GetPwuid(io::Error::last_os_error()))
    }
}

fn main() -> ! {
    match main_helper() {
        Ok(()) => exit(0),
        Err(e) => {
            eprintln!("Error: {e}");
            exit(1);
        }
    }
}

fn main_helper() -> Result<()> {
    let options = Options::parse();

    let uid = if let Some(name_or_uid) = options.username.as_deref() {
        Some(get_uid(name_or_uid)?)
    } else {
        None
    };

    // Just a plain vector, as there should not be many entries
    let mut usernames: Vec<(u32, String)> = Vec::new();

    let mut processes = Process::all()?;
    if let Some(uid) = uid {
        processes = processes.drain(..).filter(|p| p.uid == uid).collect();
    }

    for p in &processes {
        if let Some(deleted_file) = p.deleted_file() {
            if !options.quiet {
                let username = match usernames
                    .iter()
                    .find_map(|(uid, username)| (uid == &p.uid).then_some(username.as_str()))
                {
                    Some(u) => u,
                    None => {
                        let username = get_username(p.uid)?;
                        usernames.push((p.uid, username));
                        usernames.last().unwrap().1.as_str()
                    }
                };
                println!(
                    "{:5} {:4} {} ({} is deleted)",
                    p.pid,
                    username,
                    p.executable().display(),
                    deleted_file.display()
                );
            } else {
                println!("{}", p.pid);
            }
        }
    }

    Ok(())
}
