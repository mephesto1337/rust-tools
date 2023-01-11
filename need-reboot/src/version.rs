use std::cmp::{Ordering, PartialOrd};
use std::fmt;
use std::str::FromStr;

use crate::{Error, Result};

#[derive(Debug, PartialEq)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub release: Option<u16>,
    pub tag: Option<String>,
}

impl Version {
    pub fn partial_cmp_ignore_tag(&self, other: &Version) -> Option<Ordering> {
        match self.major.partial_cmp(&other.major) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }
        match self.minor.partial_cmp(&other.minor) {
            Some(Ordering::Equal) => {}
            ord => return ord,
        }
        self.release.partial_cmp(&other.release)
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(release) = self.release {
            if let Some(ref tag) = self.tag {
                write!(f, "{}.{}.{}-{}", self.major, self.minor, release, tag)
            } else {
                write!(f, "{}.{}.{}", self.major, self.minor, release)
            }
        } else if let Some(ref tag) = self.tag {
            write!(f, "{}.{}-{}", self.major, self.minor, tag)
        } else {
            write!(f, "{}.{}", self.major, self.minor)
        }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.partial_cmp_ignore_tag(other) {
            Some(Ordering::Equal) => self.tag.partial_cmp(&other.tag),
            ord => ord,
        }
    }
}

impl FromStr for Version {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (numbers, tag) = if let Some((n, t)) = s.split_once('-') {
            (n, Some(t.to_string()))
        } else {
            (s, None)
        };
        let mut parts = numbers.split('.');
        let major = parts
            .next()
            .ok_or_else(|| Error::MissingField("major".into()))?
            .parse()?;
        let minor = parts
            .next()
            .ok_or_else(|| Error::MissingField("major".into()))?
            .parse()?;
        let release = if let Some(rel) = parts.next() {
            Some(rel.parse()?)
        } else {
            None
        };

        Ok(Self {
            major,
            minor,
            release,
            tag,
        })
    }
}
