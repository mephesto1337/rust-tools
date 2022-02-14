use crate::cbindings::time_t;
use crate::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct SquidAccessEntry<'a> {
    pub method: &'a str,
    pub time: time_t,
    pub url: &'a str,
}

impl<'a> SquidAccessEntry<'a> {
    pub fn from_str(s: &'a str) -> Result<Self, Error> {
        // 1634718299.586     84 127.0.0.1 TCP_MISS/200 785 POST https://spclient.wg.spotify.com/gabo-receiver-service/v3/events/ - HIER_DIRECT/35.186.224.25 -
        let s = if let Some(idx) = s.find(|c| c != '\0') {
            &s[idx..]
        } else {
            s
        };

        let mut parts = s.split_ascii_whitespace();

        let time_str = parts.next().ok_or_else(|| Error::MissingField("time"))?;
        let time = time_str.split('.').next().unwrap().parse()?;

        let _elapsed = parts.next().ok_or_else(|| Error::MissingField("elapsed"))?;
        let _remotehost = parts
            .next()
            .ok_or_else(|| Error::MissingField("remotehost"))?;
        let _codestatus = parts
            .next()
            .ok_or_else(|| Error::MissingField("code/status"))?;
        let _bytes = parts.next().ok_or_else(|| Error::MissingField("bytes"))?;

        let method = parts.next().ok_or_else(|| Error::MissingField("method"))?;
        let url = parts.next().ok_or_else(|| Error::MissingField("url"))?;

        Ok(Self { time, method, url })
    }
}

pub fn parse_squid_access_file<P>(
    content: &str,
    predicate: P,
) -> Result<Vec<SquidAccessEntry<'_>>, Error>
where
    P: Fn(&'_ SquidAccessEntry) -> bool,
{
    let mut entries = Vec::new();

    for line in content.lines() {
        let entry = SquidAccessEntry::from_str(line)?;
        if predicate(&entry) {
            entries.push(entry);
        }
    }

    Ok(entries)
}
