use std::{collections::HashSet, env, fs, path::PathBuf};

mod cbindings;
mod error;
mod pattern;
mod squid;

use error::Error;
use pattern::Pattern;

fn main() -> Result<(), Error> {
    let raw_patterns = env::args().skip(1).collect::<Vec<_>>();
    let patterns = raw_patterns
        .iter()
        .map(|p| p.as_str().into())
        .collect::<Vec<Pattern<'_>>>();

    if patterns.is_empty() {
        let prog = PathBuf::from(env::args().next().unwrap());

        println!(
            "Usage: {} PATTERN [PATTERN...]",
            prog.file_name().unwrap().to_str().unwrap()
        );
        return Ok(());
    }

    let squid_access_content = fs::read_to_string("/var/log/squid/access.log")?;
    let last_day = cbindings::c_time() - (24 * 60 * 60);
    let entries = squid::parse_squid_access_file(squid_access_content.as_str(), |se| {
        se.time >= last_day && se.method == "GET"
    })?;
    let mut printed_urls = HashSet::new();
    for entry in &entries {
        if patterns.iter().any(|p| p.is_match(entry.url)) && printed_urls.insert(entry.url) {
            println!("{}", entry.url);
        }
    }

    Ok(())
}
