use base64::{
    alphabet::{STANDARD, URL_SAFE},
    engine::fast_portable::{FastPortable, NO_PAD, PAD},
};
use std::io::{self, copy, Read};

use crate::codecs::{Codec, Result};

struct StripWhitespacesReader<R> {
    inner: R,
}

impl<R: Read> Read for StripWhitespacesReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        loop {
            let n = self.inner.read(buf)?;
            if n == 0 {
                break Ok(0);
            }

            let mut i = 0;
            let mut j = 0;

            // To optimize bounds checks
            assert!(n < buf.len() && i < n);
            while i < n {
                if let Some(next) = buf[i..]
                    .iter()
                    .enumerate()
                    .find_map(|(idx, b)| (!b.is_ascii_whitespace()).then_some(idx))
                {
                    buf[j] = buf[i + next];
                    j += 1;
                    i += next + 1;
                } else {
                    // No more bytes
                    break;
                }
            }

            if j > 1 {
                break Ok(j - 1);
            }
        }
    }
}

#[derive(Default, Clone)]
pub struct Base64StandardCodec;

fn encode_into(
    data: &[u8],
    writer: &mut Vec<u8>,
    alphabet: &base64::alphabet::Alphabet,
    pad: bool,
) -> Result<()> {
    let engine = if pad {
        FastPortable::from(alphabet, PAD)
    } else {
        FastPortable::from(alphabet, NO_PAD)
    };
    let mut encoder = base64::write::EncoderWriter::from(writer, &engine);

    let mut reader = data;

    copy(&mut reader, &mut encoder)?;
    encoder.finish()?;
    Ok(())
}

fn decode_into(
    data: &[u8],
    writer: &mut Vec<u8>,
    alphabet: &base64::alphabet::Alphabet,
    pad: bool,
) -> Result<()> {
    let engine = if pad {
        FastPortable::from(alphabet, PAD)
    } else {
        FastPortable::from(alphabet, NO_PAD)
    };
    let mut inner_reader = data;
    let mut reader = StripWhitespacesReader {
        inner: &mut inner_reader,
    };
    let mut decoder = base64::read::DecoderReader::from(&mut reader, &engine);
    copy(&mut decoder, writer)?;
    Ok(())
}

impl Codec for Base64StandardCodec {
    fn name(&self) -> &'static str {
        "base64-standard"
    }

    fn description(&self) -> &'static str {
        "Base64 standard alphabet with padding"
    }

    fn encode_into(&self, data: &[u8], output: &mut Vec<u8>) -> Result<()> {
        encode_into(data, output, &STANDARD, true)
    }

    fn decode_into(&self, data: &[u8], output: &mut Vec<u8>) -> Result<()> {
        decode_into(data, output, &STANDARD, true)
    }

    fn build(&self, args: &str) -> Option<super::Plugin> {
        let _ = args;
        Some(Box::new(Self) as super::Plugin)
    }

    fn decoded_size_hint(&self, size: usize) -> usize {
        (size / 4) * 3
    }

    fn encoded_size_hint(&self, size: usize) -> usize {
        ((size + 3) / 3) * 4
    }
}

#[derive(Default, Clone)]
pub struct Base64UrlCodec;

impl Codec for Base64UrlCodec {
    fn name(&self) -> &'static str {
        "base64-urlsafe"
    }

    fn description(&self) -> &'static str {
        "Base64 url-safe alphabet without padding"
    }

    fn encode_into(&self, data: &[u8], output: &mut Vec<u8>) -> Result<()> {
        encode_into(data, output, &URL_SAFE, false)
    }

    fn decode_into(&self, data: &[u8], output: &mut Vec<u8>) -> Result<()> {
        decode_into(data, output, &URL_SAFE, false)
    }

    fn build(&self, args: &str) -> Option<super::Plugin> {
        let _ = args;
        Some(Box::new(Self) as super::Plugin)
    }

    fn decoded_size_hint(&self, size: usize) -> usize {
        (size / 4) * 3
    }

    fn encoded_size_hint(&self, size: usize) -> usize {
        ((size + 3) / 3) * 4
    }
}

#[derive(Default, Clone)]
pub struct Base64AutoCodec;

impl Codec for Base64AutoCodec {
    fn name(&self) -> &'static str {
        "base64-auto"
    }

    fn description(&self) -> &'static str {
        "Tries both base64-urlsafe and base64-standard"
    }

    fn encode_into(&self, data: &[u8], output: &mut Vec<u8>) -> Result<()> {
        let std = Base64StandardCodec;
        std.encode_into(data, output)
    }

    fn decode_into(&self, data: &[u8], output: &mut Vec<u8>) -> Result<()> {
        const STD_SPECIALS: &[u8] = b"/=+";

        if STD_SPECIALS.iter().any(|s| data.contains(s)) {
            let std = Base64StandardCodec;
            std.decode_into(data, output)
        } else {
            let url = Base64UrlCodec;
            url.decode_into(data, output)
        }
    }

    fn build(&self, args: &str) -> Option<super::Plugin> {
        let _ = args;
        Some(Box::new(Self) as super::Plugin)
    }

    fn decoded_size_hint(&self, size: usize) -> usize {
        (size / 4) * 3
    }

    fn encoded_size_hint(&self, size: usize) -> usize {
        ((size + 3) / 3) * 4
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_whitespaces() {
        let input = "Q29udGVudC1EaXNwb3NpdGlvbjogaW5saW5lOyBmaWxlbmFtZT0iaW1hZ2UwMDEuanBnQDAxRDhB\nQjFDLjI4QzQzMTkwLmRvY3MiOwo=\n";
        let mut inner_reader = input.as_bytes();
        let mut buffer = [0u8; 256];
        let mut reader = StripWhitespacesReader {
            inner: &mut inner_reader,
        };

        let n = reader.read(&mut buffer[..]).unwrap();
        assert_eq!(std::str::from_utf8(&buffer[..n]).unwrap(), "Q29udGVudC1EaXNwb3NpdGlvbjogaW5saW5lOyBmaWxlbmFtZT0iaW1hZ2UwMDEuanBnQDAxRDhBQjFDLjI4QzQzMTkwLmRvY3MiOwo=");
    }
}
