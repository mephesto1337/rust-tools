use std::cmp::Ordering;

mod error;
mod package;
mod uname;
mod version;

pub use crate::{
    error::{Error, Result},
    package::PackageManager,
    uname::UtsName,
    version::Version,
};

fn main() -> Result<()> {
    let package_managers = vec![
        Box::new(package::pacman::Pacman::new()) as Box<dyn PackageManager>,
        Box::new(package::apt::Apt::new()) as Box<dyn PackageManager>,
    ];
    let uts = UtsName::new()?;

    for pm in &package_managers {
        if !pm.is_present() {
            continue;
        }
        let linux_pkg_name = pm.kernel_package(&uts);
        let linux_pkg = pm.query(&linux_pkg_name)?;
        let running_version: Version = uts.release.clone().into();

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
    }
    Ok(())
}
