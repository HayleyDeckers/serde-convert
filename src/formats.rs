pub mod json;
pub mod toml;
pub mod xml;
pub mod yaml;

use anyhow::Result;
use std::io::{Read, Write};
pub trait MySerializer<W: Write> {
    fn new(writer: W) -> Self;
    fn serializer(&mut self) -> impl serde::Serializer;
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

pub trait MyDeserializer<R: Read>: Sized {
    fn new(reader: R) -> Result<Self>;
    fn deserializer(&mut self) -> impl serde::de::Deserializer<'_>;
}
