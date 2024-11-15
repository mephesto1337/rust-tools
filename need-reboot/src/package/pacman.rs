use std::process;

use crate::{
    package::{Package, PackageManager},
    Error, Result,
};

pub struct Pacman {
    _unused: (),
}

impl Pacman {
    pub fn new() -> Self {
        Self { _unused: () }
    }

    fn parse_line(s: &str) -> Result<Package> {
        let mut parts = s.split_ascii_whitespace();

        let name = parts.next().ok_or_else(|| Error::PackageFormat(s.into()))?;
        let version = parts
            .next()
            .ok_or_else(|| Error::PackageFormat(s.into()))?
            .into();
        if parts.next().is_some() {
            return Err(Error::PackageFormat(s.into()));
        }

        Ok(Package {
            name: name.into(),
            version,
        })
    }
}

impl PackageManager for Pacman {
    fn query(&self, name: &str) -> Result<Package> {
        let output = process::Command::new(self.name())
            .arg("-Q")
            .arg(name)
            .output()?;
        if !output.status.success() {
            return Err(output.into());
        }

        let output = std::str::from_utf8(&output.stdout[..])?;
        if let Some(line) = output.lines().next() {
            let package: Package = Self::parse_line(line)?;
            Ok(package)
        } else {
            Err(Error::PackageNotFound(name.into()))
        }
    }

    fn kernel_package(&self, uts: &crate::UtsName) -> String {
        let suffix = if uts.release.ends_with("-lts") {
            "-lts"
        } else if uts.release.ends_with("-hardened") {
            "-hardened"
        } else {
            ""
        };
        format!("linux{}", suffix)
    }

    fn name(&self) -> &'static str {
        "pacman"
    }
}
