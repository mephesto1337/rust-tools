use std::{
    cmp::{Ordering, PartialOrd},
    process,
};

use crate::{
    package::{Package, PackageManager},
    Error, Result, UtsName, Version,
};

pub struct Apt {
    _unused: (),
}

impl Apt {
    pub fn new() -> Self {
        Self { _unused: () }
    }
}

impl PackageManager for Apt {
    fn query(&self, name: &str) -> Result<Package> {
        let output = process::Command::new("dpkg-query")
            .arg("--list")
            .arg("--")
            .arg(name)
            .output()?;
        if !output.status.success() {
            return Err(output.into());
        }

        let output = std::str::from_utf8(&output.stdout[..])?;
        let mut last_pkg: Option<Package> = None;
        for line in output.lines() {
            let mut parts = line.split_ascii_whitespace();
            let Some(status) = parts.next() else {
                continue;
            };
            if status != "ii" {
                continue;
            }

            let Some(pkgname) = parts.next() else {
                continue;
            };
            let Some(version) = parts.next().map(Into::<Version>::into) else {
                continue;
            };

            match last_pkg {
                None => {
                    last_pkg = Some(Package {
                        name: pkgname.into(),
                        version,
                    });
                }
                Some(ref lp) => {
                    if let Some(Ordering::Less) = lp.version.partial_cmp(&version) {
                        last_pkg = Some(Package {
                            name: pkgname.into(),
                            version,
                        });
                    }
                }
            }
        }

        last_pkg.ok_or_else(|| Error::PackageFormat("No version specified".into()))
    }

    fn kernel_package(&self, uts: &UtsName) -> String {
        let mut name: String = "linux-image*".into();
        if let Some((_, spec)) = uts.release.rsplit_once('-') {
            name.reserve(1 + spec.len());
            name.push('-');
            name.push_str(spec);
        }

        name
    }

    fn name(&self) -> &'static str {
        "apt"
    }
}
