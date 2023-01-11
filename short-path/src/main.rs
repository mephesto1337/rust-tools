use std::env;
use std::fs::DirEntry;
use std::io;
use std::path::{Path, PathBuf};

fn get_shortest<'p, I, F>(collection: &'_ [I], prefix: &'p str, match_prefix: F) -> &'p str
where
    I: AsRef<str> + std::fmt::Debug,
    F: Fn(&str, &str) -> bool,
{
    for size in 1..prefix.len() {
        if collection
            .iter()
            .filter(|e| match_prefix(e.as_ref(), &prefix[..size]))
            .count()
            == 1
        {
            return &prefix[..size];
        }
    }
    prefix
}

fn dirname(entry: DirEntry) -> String {
    entry
        .file_name()
        .into_string()
        .expect("Cannot convert from OsString")
}

fn dir_entries<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    Ok(path
        .as_ref()
        .read_dir()?
        .filter_map(|p| Some(dirname(p.ok()?)))
        .collect::<Vec<_>>())
}

fn main() -> io::Result<()> {
    let mut ignore_case = false;

    let cwd = env::current_dir()?;
    assert!(cwd.is_absolute());

    if let Some(first) = env::args().nth(1) {
        if first == "-i" || first == "--ignore-case" {
            ignore_case = true;
        } else {
            eprintln!("Invalid argument {:?}", first);
        }
    }

    let cur = match cwd.file_name() {
        Some(cur) => cur,
        None => {
            println!("/");
            return Ok(());
        }
    };

    let mut path = PathBuf::new();
    for component in cwd.parent().unwrap().ancestors() {
        let prefix = match component.file_name() {
            Some(prefix) => prefix.to_str().expect("Could not convert from OsStr"),
            None => {
                // Add root and then break
                path = PathBuf::from(component).join(path);
                break;
            }
        };
        let entries = dir_entries(component.parent().unwrap())?;
        let shortest = if ignore_case {
            get_shortest(&entries[..], prefix, |entry, p| {
                if entry.len() < p.len() {
                    false
                } else {
                    entry[..p.len()].eq_ignore_ascii_case(p)
                }
            })
        } else {
            get_shortest(&entries[..], prefix, |entry, p| entry.starts_with(p))
        };
        path = PathBuf::from(shortest).join(path);
    }

    path = path.join(cur);

    if let Ok(home) = env::var("HOME") {
        if cwd.starts_with(&home) {
            let skip = PathBuf::from(home).components().count();
            let mut new_path = PathBuf::from("~");
            for part in path.iter().skip(skip) {
                new_path = new_path.join(part);
            }
            path = new_path;
        }
    }

    println!("{}", path.display());
    Ok(())
}
