use std::io::{Read, Write};

use anyhow::Result;

use crate::formats::{MyDeserializer, MySerializer};

pub struct Des<R: Read> {
	reader: R,
}

impl<R: Read> MyDeserializer<R> for Des<R> {
	fn new(reader: R) -> Result<Self> {
		Ok(Self { reader })
	}
	fn deserializer(&mut self) -> impl serde::de::Deserializer<'_> {
		serde_yaml::Deserializer::from_reader(&mut self.reader)
	}
}

pub struct Ser<W: Write> {
	inner: serde_yaml::Serializer<W>,
}

impl<W: Write> MySerializer<W> for Ser<W> {
	fn new(writer: W) -> Self {
		Self {
			inner: serde_yaml::Serializer::new(writer),
		}
	}
	fn serializer(&mut self) -> impl serde::Serializer {
		&mut self.inner
	}
}
