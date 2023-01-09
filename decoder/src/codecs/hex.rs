use std::fmt::write;

use crate::codecs::{Codec, CodecError, Result};

pub struct HexCodec;

fn get_hex_value(high: u8, low: u8) -> Result<u8> {
    if !high.is_ascii_hexdigit() {
        return Err(CodecError::InvalidHexDigit(high));
    }
    if !low.is_ascii_hexdigit() {
        return Err(CodecError::InvalidHexDigit(low));
    }

    let b = [high, low];

    let s = std::str::from_utf8(&b[..]).unwrap();
    Ok(u8::from_str_radix(s, 16).unwrap())
}

impl Codec for HexCodec {
    fn name(&self) -> &'static str {
        "hex"
    }

    fn description(&self) -> &'static str {
        "lowercase hexadecimal"
    }

    fn encode_into(&self, data: &[u8], output: &mut Vec<u8>) -> Result<()> {
        let mut s = String::new();
        for b in data {
            s.clear();
            write(&mut s, format_args!("{:02x}", b)).unwrap();
            output.extend_from_slice(s.as_bytes());
        }
        Ok(())
    }

    fn decode_into(&self, data: &[u8], output: &mut Vec<u8>) -> Result<()> {
        let data = if data.len() & 1 == 1 {
            output.push(get_hex_value(0, data[0])?);
            &data[1..]
        } else {
            data
        };
        for i in 0..(data.len() / 2) {
            let b = get_hex_value(data[2 * i], data[2 * i + 1])?;
            output.push(b);
        }
        Ok(())
    }

    fn decoded_size_hint(&self, size: usize) -> usize {
        size / 2
    }

    fn encoded_size_hint(&self, size: usize) -> usize {
        size * 2
    }
}
