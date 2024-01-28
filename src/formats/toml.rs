use crate::formats::{MyDeserializer, MySerializer};
use anyhow::{Context, Result};
use std::{
    io::{Read, Write},
    marker::PhantomData,
};

pub struct Des<R: Read> {
    content: String,
    _phantom: PhantomData<R>,
}

impl<R: Read> MyDeserializer<R> for Des<R> {
    fn new(mut reader: R) -> Result<Self> {
        let mut content = String::new();
        reader
            .read_to_string(&mut content)
            .context("failed to read contents of file")?;
        Ok(Self {
            content,
            _phantom: PhantomData,
        })
    }
    fn deserializer(&mut self) -> impl serde::de::Deserializer<'_> {
        toml::Deserializer::new(&self.content)
    }
}

pub struct Ser<W> {
    content: String,
    writer: W,
}

impl<W: Write> MySerializer<W> for Ser<W> {
    fn new(writer: W) -> Self {
        Self {
            content: String::new(),
            writer,
        }
    }
    fn serializer(&mut self) -> impl serde::Serializer {
        toml::Serializer::new(&mut self.content)
    }

    fn flush(&mut self) -> Result<()> {
        self.writer
            .write_all(self.content.as_bytes())
            .context("failed to flush TOML string")
    }
}
