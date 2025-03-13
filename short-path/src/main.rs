use std::{
    env,
    ffi::{OsStr, OsString},
    io,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

fn get_shortest<'p, F>(collection: &'_ [OsString], prefix: &'p [u8], match_prefix: F) -> &'p [u8]
where
    F: Fn(&[u8], &[u8]) -> bool,
{
    for size in 1..prefix.len() {
        if collection
            .iter()
            .filter(|e| match_prefix(e.as_encoded_bytes(), &prefix[..size]))
            .count()
            == 1
        {
            return &prefix[..size];
        }
    }
    prefix
}

fn dir_entries<P: AsRef<Path>>(path: P) -> io::Result<Vec<OsString>> {
    Ok(path
        .as_ref()
        .read_dir()?
        .filter_map(|p| Some(p.ok()?.file_name()))
        .collect::<Vec<_>>())
}

fn main() -> io::Result<()> {
    let mut ignore_case = false;

    let cwd = env::current_dir()?;
    assert!(
        cwd.is_absolute(),
        "Current working directory is not absolute?: {cwd:?}"
    );

    if let Some(first) = env::args().nth(1) {
        if first == "-i" || first == "--ignore-case" {
            ignore_case = true;
        } else {
            eprintln!("Invalid argument {:?}", first);
        }
    }

    let (Some(current), Some(parents)) = (cwd.file_name(), cwd.parent()) else {
        println!("/");
        return Ok(());
    };

    let mut path = PathBuf::new();
    for component in parents.ancestors() {
        let prefix = match component.file_name() {
            Some(prefix) => prefix,
            None => {
                assert_eq!(component, Path::new("/"));
                // Add root and then break
                path = PathBuf::from(component).join(path);
                break;
            }
        };
        let entries = dir_entries(component.parent().unwrap())?;
        let shortest = if ignore_case {
            get_shortest(&entries, prefix.as_encoded_bytes(), |entry, p| {
                if entry.len() < p.len() {
                    false
                } else {
                    entry[..p.len()].eq_ignore_ascii_case(p)
                }
            })
        } else {
            get_shortest(&entries[..], prefix.as_encoded_bytes(), |entry, p| {
                entry.starts_with(p)
            })
        };
        path = PathBuf::from(OsStr::from_bytes(shortest)).join(path);
    }

    path = path.join(current);

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
