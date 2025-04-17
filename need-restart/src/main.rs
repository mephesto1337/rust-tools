use clap::Parser;
use std::{ffi::CStr, io, mem::MaybeUninit, ptr};

mod error;
mod process;

pub use error::{Error, Result};
use process::Process;

#[derive(Parser, Debug)]
struct Options {
    #[arg(short = 'q', long = "quiet")]
    quiet: bool,
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

fn main() -> Result<()> {
    let options = Options::parse();

    // Just a plain vector, as there should not be many entries
    let mut usernames: Vec<(u32, String)> = Vec::new();

    let processes = Process::all()?;
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
