use crate::codecs::{Codec, Result};

const DEFAULT_KEY: u8 = 13;

pub struct RotCodec(u8);

impl Default for RotCodec {
    fn default() -> Self {
        Self(DEFAULT_KEY)
    }
}

impl RotCodec {
    fn apply(key: u8, b: u8) -> u8 {
        match b {
            b'a'..=b'z' => ((b - b'a' + key) % 26) + b'a',
            b'A'..=b'Z' => ((b - b'A' + key) % 26) + b'A',
            b'0'..=b'9' => ((b - b'0' + key) % 10) + b'0',
            _ => b,
        }
    }
}

impl Codec for RotCodec {
    fn name(&self) -> &'static str {
        "rot"
    }

    fn description(&self) -> &'static str {
        "rotate ascii letters"
    }

    fn encode_into(&self, data: &[u8], output: &mut Vec<u8>) -> Result<()> {
        for b in data {
            output.push(Self::apply(self.0, *b));
        }

        Ok(())
    }

    fn decode_into(&self, data: &[u8], output: &mut Vec<u8>) -> Result<()> {
        let key = 26 - self.0;
        for b in data {
            output.push(Self::apply(key, *b));
        }

        Ok(())
    }
}
