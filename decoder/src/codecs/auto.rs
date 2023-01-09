use crate::codecs::{get_available_plugins, Codec, CodecError, Result};

pub struct AutoCodec;

impl Codec for AutoCodec {
    fn name(&self) -> &'static str {
        "auto"
    }

    fn description(&self) -> &'static str {
        "Tries all codecs until one works"
    }

    fn encode_into(&self, data: &[u8], output: &mut Vec<u8>) -> Result<()> {
        output.extend_from_slice(data);
        Ok(())
    }

    fn decode_into(&self, data: &[u8], output: &mut Vec<u8>) -> Result<()> {
        let mut temp = Vec::new();

        for t in get_available_plugins() {
            if t.name().starts_with("auto") {
                continue;
            }

            temp.clear();
            temp.reserve(t.decoded_size_hint(data.len()));
            if t.decode_into(data, &mut temp).is_ok() {
                eprintln!("Decoded input with {}", t.name());
                output.append(&mut temp);
                return Ok(());
            }
        }

        Err(CodecError::NoCodecAvailable)
    }
}

pub struct AutoRecurseCodec;

impl Codec for AutoRecurseCodec {
    fn name(&self) -> &'static str {
        "auto-recurse"
    }

    fn description(&self) -> &'static str {
        "Decode input with auto plugin until it does not change or fail"
    }

    fn encode_into(&self, data: &[u8], output: &mut Vec<u8>) -> Result<()> {
        output.extend_from_slice(data);
        Ok(())
    }

    fn decode_into(&self, data: &[u8], output: &mut Vec<u8>) -> Result<()> {
        let auto_codec = AutoCodec;

        let mut next_input = auto_codec.decode(data)?;

        if next_input == data {
            output.append(&mut next_input);
            return Ok(());
        }

        loop {
            let temp = match auto_codec.decode(&next_input[..]) {
                Err(CodecError::NoCodecAvailable) => break,
                Ok(val) => val,
                Err(e) => return Err(e),
            };
            if temp == next_input {
                break;
            }
            next_input = temp;
        }
        output.append(&mut next_input);
        Ok(())
    }
}
