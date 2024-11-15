use std::{
    cmp::{Ordering, PartialOrd},
    fmt,
};

#[derive(Debug, Eq, PartialEq)]
pub struct Version {
    inner: String,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl<S> From<S> for Version
where
    S: Into<String>,
{
    fn from(value: S) -> Self {
        Self::new(value.into())
    }
}

impl Version {
    pub fn new(mut inner: String) -> Self {
        if let Some(i) = inner.find('~') {
            inner.truncate(i);
        }
        if let Some(i) = inner.find('-') {
            if let Some(j) = inner[i + 1..].find(['.', '-']) {
                inner.truncate(i + j + 1);
            }
        }
        Self { inner }
    }
}

impl PartialOrd for Version {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let splitter = ['-', '.'];
        for (s, o) in self.inner.split(splitter).zip(other.inner.split(splitter)) {
            match (s.parse::<u32>(), o.parse::<u32>()) {
                (Ok(s), Ok(o)) => {
                    let r = s.cmp(&o);
                    if r != Ordering::Equal {
                        return Some(r);
                    }
                }
                (Err(_), Err(_)) => {
                    let r = s.cmp(o);
                    if r != Ordering::Equal {
                        return Some(r);
                    }
                }
                _ => return None,
            }
        }
        Some(Ordering::Equal)
    }
}
