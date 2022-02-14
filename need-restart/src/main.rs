mod error;
mod process;

pub use error::{Error, Result};
use process::Process;

fn usage() {
    let prog = std::path::PathBuf::from(std::env::args().next().unwrap())
        .file_name()
        .and_then(|os| os.to_str())
        .map(|s| s.to_string())
        .unwrap();

    println!("Usages:");
    println!("  {} -h | --help", &prog);
    println!("  {} [-q | --quiet]", &prog);
    println!("");
    println!("Options:");
    println!("  -h, --help  : displays this message and exits.");
    println!("  -q, --quiet : enables quiet mode.");
}

fn err_main() -> Result<()> {
    let mut quiet = false;

    if let Some(first_arg) = std::env::args().skip(1).next() {
        if first_arg == "-q" || first_arg == "--quiet" {
            quiet = true;
        } else if first_arg == "-h" || first_arg == "--help" {
            usage();
            return Ok(());
        } else {
            usage();
            return Err(Error::InvalidArgument(first_arg));
        }
    }
    let processes = Process::all()?;
    for p in &processes {
        if let Some(deleted_file) = p.deleted_file() {
            if !quiet {
                println!(
                    "{:5} {} ({} is deleted)",
                    p.pid,
                    p.executable().display(),
                    deleted_file.display()
                );
            } else {
                println!("{}", p.pid);
            }
        }
    }

    Ok(())
}

fn main() -> ! {
    let code = if let Err(e) = err_main() {
        eprintln!("ERROR: {}", e);
        1
    } else {
        0
    };
    std::process::exit(code);
}
