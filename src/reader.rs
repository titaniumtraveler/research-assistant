use crate::config::ResolvedSourceFormat;
use anyhow::Result;
use encoding_rs_io::DecodeReaderBytesBuilder;
use serde::de::DeserializeOwned;
use std::{fs::File, io::BufReader, path::Path};

pub struct Reader {
    buf: Vec<u8>,
}

impl Default for Reader {
    fn default() -> Self {
        Self::new()
    }
}

impl Reader {
    pub fn new() -> Self {
        Self {
            buf: vec![0; 8 * (1 << 10)],
        }
    }

    pub fn deserialize_from<T: DeserializeOwned>(
        &mut self,
        path: &Path,
        format: ResolvedSourceFormat,
    ) -> Result<T> {
        let reader = DecodeReaderBytesBuilder::new()
            .encoding(format.encoding)
            .strip_bom(true) // Strip bom, even if encoding is explicitly set
            .bom_sniffing(format.autodetect)
            .build_with_buffer(BufReader::new(File::open(path)?), &mut self.buf)?;

        let mut de = serde_json::Deserializer::from_reader(reader);
        de.allow_trailing_comma();
        let t = T::deserialize(&mut de)?;
        de.end()?;
        Ok(t)
    }
}
