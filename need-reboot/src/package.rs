use crate::{error::Result, UtsName, Version};
use std::fmt;

pub mod apt;
pub mod pacman;

pub trait PackageManager {
    fn query(&self, name: &str) -> Result<Package>;
    fn kernel_package(&self, uts: &UtsName) -> String;
    fn name(&self) -> &'static str;
    fn is_present(&self) -> bool {
        let Ok(mut child) = std::process::Command::new(self.name()).spawn() else {
            return false;
        };
        let _ = child.kill();
        true
    }
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
