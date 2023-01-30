use base64::{
    alphabet::{STANDARD, URL_SAFE},
    engine::fast_portable::{FastPortable, NO_PAD, PAD},
};
use std::io::copy;

use crate::codecs::{Codec, Result};

#[derive(Default)]
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
    let mut reader = data;
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

    fn decoded_size_hint(&self, size: usize) -> usize {
        (size / 4) * 3
    }

    fn encoded_size_hint(&self, size: usize) -> usize {
        ((size + 3) / 3) * 4
    }
}

#[derive(Default)]
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

    fn decoded_size_hint(&self, size: usize) -> usize {
        (size / 4) * 3
    }

    fn encoded_size_hint(&self, size: usize) -> usize {
        ((size + 3) / 3) * 4
    }
}

#[derive(Default)]
pub struct Base64AutoCodec;

impl Codec for Base64AutoCodec {
    fn name(&self) -> &'static str {
        "base64-auto"
    }

    fn description(&self) -> &'static str {
        "Tries both base64-urlsafe and base64-standard"
    }

    fn encode_into(&self, data: &[u8], output: &mut Vec<u8>) -> Result<()> {
        let std = Base64StandardCodec::default();
        std.encode_into(data, output)
    }

    fn decode_into(&self, data: &[u8], output: &mut Vec<u8>) -> Result<()> {
        const STD_SPECIALS: &[u8] = &[b'/', b'=', b'+'];

        if STD_SPECIALS.iter().any(|s| data.contains(s)) {
            let std = Base64StandardCodec::default();
            std.decode_into(data, output)
        } else {
            let url = Base64UrlCodec::default();
            url.decode_into(data, output)
        }
    }

    fn decoded_size_hint(&self, size: usize) -> usize {
        (size / 4) * 3
    }

    fn encoded_size_hint(&self, size: usize) -> usize {
        ((size + 3) / 3) * 4
    }
}
