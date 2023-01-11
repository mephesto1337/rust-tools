use clap::Parser;
use std::{collections::HashSet, env, fs, path::PathBuf};

mod cbindings;
pub mod error;
mod pattern;
mod squid;
mod timespan;

use error::Error;
use pattern::Pattern;
use timespan::TimeSpan;

#[derive(Parser, Debug)]
struct Options {
    #[arg(short = 'n', long = "newer-than", default_value = "1d", value_parser)]
    newer_than: TimeSpan,

    /// Patterns to search for
    patterns: Vec<String>,
}

fn main() -> Result<(), Error> {
    let options = Options::parse();

    let patterns = options
        .patterns
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

    let timespan: i64 = options.newer_than.into();
    let squid_access_content = fs::read_to_string("/var/log/squid/access.log")?;
    let maxium_ts = cbindings::c_time() - timespan;
    let entries = squid::parse_squid_access_file(squid_access_content.as_str(), |se| {
        se.time >= maxium_ts && se.method == "GET"
    })?;
    let mut printed_urls = HashSet::new();
    for entry in &entries {
        if patterns.iter().any(|p| p.is_match(entry.url)) && printed_urls.insert(entry.url) {
            println!("{}", entry.url);
        }
    }

    Ok(())
}
