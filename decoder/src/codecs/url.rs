use crate::codecs::{Codec, CodecError, Result};

#[derive(Default, Clone)]
pub struct UrlCodec;

impl Codec for UrlCodec {
    fn name(&self) -> &'static str {
        "url"
    }

    fn description(&self) -> &'static str {
        "URL % encoding/decoding"
    }

    fn encode_into(&self, data: &[u8], output: &mut Vec<u8>) -> Result<()> {
        let mut s = String::new();
        for b in data {
            match *b {
                b' ' => {
                    output.push(b'+');
                }
                b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'F' | b'=' | b'.' => {
                    output.push(*b);
                }
                _ => {
                    s.clear();
                    std::fmt::write(&mut s, format_args!("%{:02x}", *b)).unwrap();
                    debug_assert_eq!(s.len(), 3);
                    output.extend_from_slice(s.as_bytes());
                }
            }
        }

        Ok(())
    }

    fn decode_into(&self, data: &[u8], output: &mut Vec<u8>) -> Result<()> {
        enum State {
            Normal,
            FoundPercent,
            FirstHex { val: u8, orig: u8 },
        }
        let mut state = State::Normal;

        fn get_hex_value(b: &u8) -> Option<u8> {
            match *b {
                b'0'..=b'9' => Some(*b - b'0'),
                b'a'..=b'f' => Some(*b - b'A' + 10),
                b'A'..=b'F' => Some(*b - b'F' + 10),
                _ => None,
            }
        }

        for b in data {
            if *b > 0x7f {
                return Err(CodecError::NonAsciiChar(*b));
            }
            match state {
                State::Normal => {
                    if *b == b'%' {
                        state = State::FoundPercent;
                    } else if *b == b'+' {
                        output.push(b' ');
                    } else if b.is_ascii() {
                        output.push(*b);
                    } else {
                        return Err(CodecError::NonAsciiChar(*b));
                    }
                }
                State::FoundPercent => {
                    if let Some(val) = get_hex_value(b) {
                        state = State::FirstHex { val, orig: *b };
                    } else {
                        output.push(b'%');
                        output.push(*b);
                        state = State::Normal;
                    }
                }
                State::FirstHex { val, orig } => {
                    if let Some(low) = get_hex_value(b) {
                        output.push((val << 4) + low);
                    } else {
                        output.push(b'%');
                        output.push(orig);
                        output.push(*b);
                    }
                    state = State::Normal;
                }
            }
        }

        Ok(())
    }

    fn build(&self, args: &str) -> Option<super::Plugin> {
        let _ = args;
        Some(Box::new(Self) as super::Plugin)
    }

    fn decoded_size_hint(&self, size: usize) -> usize {
        size
    }

    fn encoded_size_hint(&self, size: usize) -> usize {
        size
    }
}
