use std::io::{self, Read, Write};

use clap::{Parser, ValueEnum};

mod codecs;

#[derive(Debug, Copy, Clone, ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
enum Mode {
    Encode,
    Decode,
}

#[derive(Debug, Parser)]
struct Options {
    /// Mode to operate on
    #[arg(short, long, value_enum, default_value_t = Mode::Decode)]
    mode: Mode,

    /// List all available codecs
    #[arg(short, long)]
    list: bool,

    /// List of codecs to apply in the form NAME[:ARGS]
    #[arg(short, long)]
    codecs: Vec<String>,

    /// Input to operate on
    input: Option<String>,
}

use codecs::{get_available_plugins, Codec, Plugin};

fn get_codec_by_prefix(prefix: &str) -> Plugin {
    let (prefix, args) = match prefix.split_once(':') {
        Some((p, a)) => (p, a),
        None => (prefix, ""),
    };

    let plugins = get_available_plugins();
    let mut found_plugin = None;
    for codec in plugins {
        if codec.name().eq_ignore_ascii_case(prefix) {
            found_plugin = Some(codec);
            break;
        }

        let name = codec.name();
        if name.len() < prefix.len() {
            continue;
        } else if name[..prefix.len()].eq_ignore_ascii_case(prefix) {
            match found_plugin {
                None => {
                    found_plugin = Some(codec);
                }
                Some(p) => {
                    panic!(
                        "{:?} can match both {} and {}",
                        prefix,
                        p.name(),
                        codec.name()
                    );
                }
            }
        }
    }

    match found_plugin {
        None => {
            panic!("No plugin found with prefix {prefix:?}");
        }
        Some(p) => match p.build(args) {
            None => panic!(
                "Could not build plugin {name} with {args:?}",
                name = p.name()
            ),
            Some(c) => c,
        },
    }
}

fn read_stdin() -> io::Result<Vec<u8>> {
    let mut stdin = io::stdin().lock();
    let mut buffer = Vec::new();
    stdin.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn main() -> codecs::Result<()> {
    let args = Options::parse();

    if args.list {
        println!("Available plugins:");
        for p in get_available_plugins() {
            println!("  {}: {}", p.name(), p.description());
        }
        return Ok(());
    }

    let mut transformations: Vec<_> = args
        .codecs
        .iter()
        .map(|na| get_codec_by_prefix(na))
        .collect();

    if transformations.is_empty() {
        let auto_recurse_codec = codecs::auto::AutoRecurseCodec;
        transformations.push(auto_recurse_codec.build("").unwrap());
    }

    let mut input = match args.input {
        None => read_stdin()?,
        Some(ref p) => {
            if p == "-" {
                read_stdin()?
            } else {
                std::fs::read(p)?
            }
        }
    };

    for t in &transformations[..] {
        let output = match args.mode {
            Mode::Encode => {
                eprintln!("Encoding with {} ({})", t.name(), t.description());
                t.encode(&input[..])?
            }
            Mode::Decode => {
                eprintln!("Decoding with {} ({})", t.name(), t.description());
                t.decode(&input[..])?
            }
        };

        input = output;
    }

    let mut stdout = io::stdout().lock();
    stdout.write_all(&input[..])?;

    Ok(())
}
