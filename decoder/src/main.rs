use std::{
    env,
    io::{self, Read, Write},
};

mod codecs;

#[derive(Debug, Copy, Clone)]
#[repr(u32)]
enum Mode {
    Encode,
    Decode,
}

use codecs::{get_available_plugins, Codec, CodecError, Plugin};

fn get_codec_by_prefix(prefix: &str) -> Option<&'static Plugin> {
    let plugins = get_available_plugins();
    let mut found_plugin = None;
    for codec in plugins {
        if codec.name().eq_ignore_ascii_case(prefix) {
            return Some(codec);
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
                    eprintln!(
                        "{:?} can match both {} and {}",
                        prefix,
                        p.name(),
                        codec.name()
                    );
                    return None;
                }
            }
        }
    }

    found_plugin
}

fn read_stdin() -> io::Result<Vec<u8>> {
    let mut stdin = io::stdin().lock();
    let mut buffer = Vec::new();
    stdin.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn main() -> codecs::Result<()> {
    let mut args = env::args().skip(1);
    let mut transformations = Vec::new();

    let mode = match args.next() {
        Some(m) => {
            if m == "-e" || m == "--encode" {
                Mode::Encode
            } else if m == "-d" || m == "--decode" {
                Mode::Decode
            } else if m == "-h" || m == "--help" {
                eprintln!("Usage:");
                eprintln!(
                    "  {} [[-e | -d] CODECS] [INPUT]\n",
                    env::args().next().unwrap()
                );
                eprintln!("Options:");
                eprintln!("  -e, --encode CODECS: encode input with the specified codecs (comma separated)");
                eprintln!("  -d, --decode CODECS: decode input with the specified codecs (comma separated)");
                eprintln!("  -h, --help: show this message and exit");
                eprintln!("  -l, --list: list available codecs\n");
                eprintln!("By default, if neither -e or -d is specified, 'auto-recurse' codec in decode mode is used");
                eprintln!("If INPUT is not specified, it is read from STDIN");

                return Ok(());
            } else if m == "-l" || m == "--list" {
                eprintln!("Available plugins:");
                for p in get_available_plugins() {
                    eprintln!("  {}: {}", p.name(), p.description());
                }
                return Ok(());
            } else {
                let msg = format!("Unrecognized argument: {:?}", m);
                return Err(io::Error::new(io::ErrorKind::InvalidInput, msg).into());
            }
        }
        None => Mode::Decode,
    };

    if let Some(trans) = args.next() {
        for t in trans.split(',') {
            let codec =
                get_codec_by_prefix(t).ok_or_else(|| CodecError::InvalidPluginPrefix(t.into()))?;
            transformations.push(codec);
        }
    } else {
        let auto_recurse_codec = codecs::auto::AutoRecurseCodec;
        transformations.push(
            get_codec_by_prefix(auto_recurse_codec.name())
                .expect("AutoRecurseCodec should be registered"),
        );
    }

    let mut input = match args.next() {
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
        let output = match mode {
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
