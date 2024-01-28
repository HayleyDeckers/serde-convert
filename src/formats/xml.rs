use crate::formats::{MyDeserializer, MySerializer};
use anyhow::Result;
use std::io::{Read, Write};
pub struct Des<R: Read> {
    reader: serde_xml_rs::Deserializer<R>,
}

impl<R: Read> MyDeserializer<R> for Des<R> {
    fn new(reader: R) -> Result<Self> {
        Ok(Self {
            reader: serde_xml_rs::Deserializer::new_from_reader(reader),
        })
    }
    fn deserializer(&mut self) -> impl serde::de::Deserializer<'_> {
        &mut self.reader
    }
}

pub struct Ser<W: Write> {
    inner: serde_xml_rs::Serializer<W>,
}

impl<W: Write> MySerializer<W> for Ser<W> {
    fn new(writer: W) -> Self {
        Self {
            inner: serde_xml_rs::Serializer::new(writer),
        }
    }
    fn serializer(&mut self) -> impl serde::Serializer {
        &mut self.inner
    }
}
