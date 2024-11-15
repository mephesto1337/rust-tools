use std::{
    fs,
    os::unix::fs::MetadataExt,
    path::{Path, PathBuf},
};

use crate::{Error, Result};

/// File's status
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileState {
    /// File is still present on disk
    Present,

    /// File was deleted
    Deleted,
}

const DELETED_FILE_SUFFIX: &str = " (deleted)";

/// A running process
#[derive(Debug)]
pub struct Process {
    /// Process Identifier
    pub pid: u64,

    /// Process owner
    pub uid: u32,

    /// File being executed. Index into `Process.files`
    executable_idx: usize,

    /// Files beeing loaded into memory
    pub files: Vec<(PathBuf, FileState)>,
}

impl Process {
    pub fn new(pid: u64) -> Result<Process> {
        let maps_file = PathBuf::from("/proc").join(format!("{}", pid)).join("maps");

        let uid = fs::metadata(maps_file.parent().unwrap())?.uid();
        let maps = fs::read_to_string(&maps_file).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                Error::NonExistantProcess(pid)
            } else {
                e.into()
            }
        })?;

        let exe_file = maps_file.parent().unwrap().join("exe");
        let raw_executable = fs::read_link(exe_file)?;
        let executable = if let Some(executable) = raw_executable
            .as_os_str()
            .to_str()
            .expect("Weird executable path")
            .strip_suffix(DELETED_FILE_SUFFIX)
        {
            PathBuf::from(executable)
        } else {
            raw_executable
        };

        let mut files = Vec::new();
        for line in maps.lines() {
            if let Some((filename, state)) = Self::extract_file(line) {
                let obj = (PathBuf::from(filename), state);
                if !files.contains(&obj) {
                    files.push(obj);
                }
            }
        }

        let executable_idx = files
            .iter()
            .enumerate()
            .find_map(|(idx, path_state)| {
                let (path, _) = path_state;
                if path == &executable {
                    Some(idx)
                } else {
                    None
                }
            })
            .ok_or_else(|| Error::NoExecutableFile(pid))?;

        Ok(Self {
            pid,
            uid,
            executable_idx,
            files,
        })
    }

    pub fn executable(&self) -> &Path {
        &self.files[self.executable_idx].0
    }

    fn extract_file(line: &str) -> Option<(&str, FileState)> {
        const FILEPATH_FIELD_NUM: usize = 6;

        let mut buffer = line;
        for _ in 1..FILEPATH_FIELD_NUM {
            let next_whitespace = buffer.find(|c: char| c.is_ascii_whitespace())?;
            buffer = &buffer[next_whitespace..];

            let next_field = buffer.find(|c: char| !c.is_ascii_whitespace())?;
            buffer = &buffer[next_field..];
        }

        if buffer.is_empty() {
            None
        } else if let Some(filename) = buffer.strip_suffix(DELETED_FILE_SUFFIX) {
            if fs::metadata(filename).is_ok() {
                Some((filename, FileState::Deleted))
            } else {
                None
            }
        } else {
            Some((buffer, FileState::Present))
        }
    }

    pub fn all() -> Result<Vec<Self>> {
        let mut processes = Vec::new();
        for entry in PathBuf::from("/proc").read_dir()? {
            let entry = entry?;

            if let Some(pid) = entry
                .file_name()
                .to_str()
                .and_then(|s| s.parse::<u64>().ok())
            {
                match Self::new(pid) {
                    Ok(process) => processes.push(process),
                    Err(e) => {
                        if !e.is_not_found() {
                            eprintln!("Could not retrieve process {}: {}", pid, e);
                        }
                    }
                }
            }
        }

        Ok(processes)
    }

    pub fn deleted_file(&self) -> Option<&PathBuf> {
        self.files.iter().find_map(|(path, state)| {
            if state == &FileState::Deleted {
                Some(path)
            } else {
                None
            }
        })
    }
}
