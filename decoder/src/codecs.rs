use std::{
    ptr,
    sync::atomic::{AtomicPtr, Ordering},
};

pub mod auto;
pub mod base64;
pub mod error;
pub mod hex;
pub mod rot;
pub mod url;

pub use error::{CodecError, Result};

/// A Codec trait used to encode/decode
pub trait Codec {
    /// Codec's name
    fn name(&self) -> &'static str;

    /// Codec's description
    fn description(&self) -> &'static str;

    /// Encode into specified buffer
    fn encode_into(&self, data: &[u8], output: &mut Vec<u8>) -> Result<()>;

    /// Decode into specified buffer
    fn decode_into(&self, data: &[u8], output: &mut Vec<u8>) -> Result<()>;

    /// Approximation decoded output size
    fn decoded_size_hint(&self, size: usize) -> usize {
        size
    }

    /// Approximation encoded output size
    fn encoded_size_hint(&self, size: usize) -> usize {
        size
    }

    /// Encode data
    fn encode(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut encoded = Vec::with_capacity(self.encoded_size_hint(data.len()));
        self.encode_into(data, &mut encoded)?;
        Ok(encoded)
    }

    /// Decode data
    fn decode(&self, data: &[u8]) -> Result<Vec<u8>> {
        let mut decoded = Vec::with_capacity(self.decoded_size_hint(data.len()));
        self.decode_into(data, &mut decoded)?;
        Ok(decoded)
    }
}

pub type Plugin = Box<dyn Codec>;

pub fn get_available_plugins() -> &'static [Plugin] {
    static AVAILABLE_PLUGINS: AtomicPtr<Vec<Plugin>> = AtomicPtr::new(ptr::null_mut());

    // Ordering::Relaxed is sufficient as this program is mono-threaded
    let available_plugins_ptr = AVAILABLE_PLUGINS.load(Ordering::Relaxed);
    if available_plugins_ptr.is_null() {
        let plugins = Box::new(vec![
            Box::<hex::HexCodec>::default() as Plugin,
            Box::<base64::Base64StandardCodec>::default() as Plugin,
            Box::<base64::Base64UrlCodec>::default() as Plugin,
            Box::<url::UrlCodec>::default() as Plugin,
            Box::<rot::RotCodec>::default() as Plugin,
            Box::<auto::AutoCodec>::default() as Plugin,
            Box::<auto::AutoRecurseCodec>::default() as Plugin,
        ]);
        AVAILABLE_PLUGINS.store(Box::into_raw(plugins), Ordering::Relaxed);
    }
    unsafe { AVAILABLE_PLUGINS.load(Ordering::Relaxed).as_ref() }
        .unwrap()
        .as_slice()
}
