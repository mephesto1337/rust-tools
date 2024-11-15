use std::{ffi::CStr, io, mem::MaybeUninit, ptr};

mod error;
mod process;

pub use error::{Error, Result};
use process::Process;

fn usage() {
    let prog = std::path::PathBuf::from(std::env::args().next().unwrap())
        .file_name()
        .and_then(|os| os.to_str())
        .map(|s| s.to_string())
        .unwrap();

    println!("Usages:");
    println!("  {} -h | --help", &prog);
    println!("  {} [-q | --quiet]", &prog);
    println!();
    println!("Options:");
    println!("  -h, --help  : displays this message and exits.");
    println!("  -q, --quiet : enables quiet mode.");
}

fn get_username(uid: u32) -> Result<String> {
    let mut pwd = MaybeUninit::uninit();
    let mut result = ptr::null_mut();
    let mut buf = Vec::with_capacity(1024);
    let ret = unsafe {
        libc::getpwuid_r(
            uid,
            pwd.as_mut_ptr(),
            buf.as_mut_ptr(),
            buf.capacity(),
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

fn err_main() -> Result<()> {
    let mut quiet = false;

    // Just a plain vector, as there should not be many entries
    let mut usernames: Vec<(u32, String)> = Vec::new();

    if let Some(first_arg) = std::env::args().nth(1) {
        if first_arg == "-q" || first_arg == "--quiet" {
            quiet = true;
        } else if first_arg == "-h" || first_arg == "--help" {
            usage();
            return Ok(());
        } else {
            usage();
            return Err(Error::InvalidArgument(first_arg));
        }
    }
    let processes = Process::all()?;
    for p in &processes {
        if let Some(deleted_file) = p.deleted_file() {
            if !quiet {
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

fn main() -> ! {
    let code = if let Err(e) = err_main() {
        eprintln!("ERROR: {}", e);
        1
    } else {
        0
    };
    std::process::exit(code);
}
