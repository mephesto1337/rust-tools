use std::fmt;
use std::process;
use std::str::FromStr;

use crate::{Error, Result};
use semver::Version;

pub struct Pacman {
    _unused: u8,
}

#[derive(Debug)]
pub struct Package {
    pub name: String,
    pub version: Version,
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.version)
    }
}

impl FromStr for Package {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split_ascii_whitespace();

        let name = parts.next().ok_or_else(|| Error::PackageFormat(s.into()))?;
        let version = parts
            .next()
            .ok_or_else(|| Error::PackageFormat(s.into()))?
            .parse()?;
        if parts.next().is_some() {
            return Err(Error::PackageFormat(s.into()));
        }

        Ok(Self {
            name: name.into(),
            version,
        })
    }
}

impl Pacman {
    pub fn new() -> Self {
        Self { _unused: 0 }
    }

    pub fn query(&self, name: impl AsRef<str>) -> Result<Option<Package>> {
        let output = process::Command::new("pacman")
            .arg("-Q")
            .arg(name.as_ref())
            .output()?;
        if !output.status.success() {
            return Err(output.into());
        }

        let output = std::str::from_utf8(&output.stdout[..])?;
        if let Some(line) = output.lines().next() {
            let package: Package = line.parse()?;
            Ok(Some(package))
        } else {
            Ok(None)
        }
    }
}
