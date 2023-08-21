use std::cmp::Ordering;

mod error;
mod pacman;
mod uname;

pub use crate::error::{Error, Result};

fn main() -> Result<()> {
    let pkg = pacman::Pacman::new();
    let uts = uname::UtsName::new()?;

    let suffix = if uts.release.ends_with("-lts") {
        "-lts"
    } else if uts.release.ends_with("-hardened") {
        "-hardened"
    } else {
        ""
    };
    let linux_pkg_name = format!("linux{}", suffix);
    let linux_pkg = match pkg.query(&linux_pkg_name)? {
        Some(p) => p,
        None => {
            panic!("Cannot find package {}", linux_pkg_name);
        }
    };

    let running_version: semver::Version = uts.release.strip_suffix(suffix).unwrap().parse()?;

    match running_version.partial_cmp(&linux_pkg.version) {
        Some(Ordering::Less) => {
            println!(
                "REBOOT (curr={} < last={})",
                &running_version, &linux_pkg.version
            );
        }
        Some(Ordering::Equal) => {
            println!("OK ({})", &running_version);
        }
        Some(Ordering::Greater) => {
            println!(
                "REBOOT (curr={} > last={})",
                &running_version, &linux_pkg.version
            );
        }
        None => {
            println!(
                "REBOOT (curr={} != last={})",
                &running_version, &linux_pkg.version
            );
        }
    }
    Ok(())
}
