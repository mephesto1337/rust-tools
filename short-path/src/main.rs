use std::env;
use std::fs::DirEntry;
use std::io;
use std::path::{Path, PathBuf};

fn get_shortest<'p, I>(collection: &'_ [I], prefix: &'p str) -> &'p str
where
    I: AsRef<str> + std::fmt::Debug,
{
    for size in 1..prefix.len() {
        if collection
            .iter()
            .filter(|e| e.as_ref().starts_with(&prefix[..size]))
            .count()
            == 1
        {
            return &prefix[..size];
        }
    }
    return prefix;
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
    let cwd = env::current_dir()?;
    assert!(cwd.is_absolute());

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
        let shortest = get_shortest(&entries[..], prefix);
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
