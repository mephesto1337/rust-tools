use clap::Parser;
use std::{
    env,
    ffi::{OsStr, OsString},
    io,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

fn get_shortest<'p, F>(
    collection: &'_ [OsString],
    prefix: &'p [u8],
    match_prefix: F,
) -> Option<&'p [u8]>
where
    F: Fn(&[u8], &[u8]) -> bool,
{
    'next_size: for size in 1..prefix.len() {
        let cur_prefix = &prefix[..size];
        let mut matches = 0usize;
        for entry in collection {
            if match_prefix(entry.as_encoded_bytes(), cur_prefix) {
                matches += 1;
                if matches > 1 {
                    continue 'next_size;
                }
            }
        }
        match matches {
            0 => return None,
            1 => return Some(cur_prefix),
            _ => unreachable!("Should have continued 7 lines above"),
        }
    }
    Some(prefix)
}

fn dir_entries<P: AsRef<Path>>(path: P) -> io::Result<Vec<OsString>> {
    Ok(path
        .as_ref()
        .read_dir()?
        .filter_map(|p| Some(p.ok()?.file_name()))
        .collect::<Vec<_>>())
}

#[derive(Parser, Debug)]
struct Options {
    #[arg(
        short = 'i',
        long = "ignore-case",
        help = "Ignore case when comparing directories prefix",
        default_value_t = false
    )]
    ignore_case: bool,
}

fn main() -> io::Result<()> {
    let options = Options::parse();

    let cwd = env::current_dir()?;
    assert!(
        cwd.is_absolute(),
        "Current working directory is not absolute?: {cwd:?}"
    );

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
        let shortest = if options.ignore_case {
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
        if let Some(shortest) = shortest {
            path = PathBuf::from(OsStr::from_bytes(shortest)).join(path);
        } else {
            path = PathBuf::from(prefix).join(path);
        }
    }

    path = path.join(current);

    if let Ok(home) = env::var("HOME") {
        if cwd.starts_with(&home) {
            let skip = Path::new(&home).components().count();
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
