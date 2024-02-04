use std::io::{Read, Write};

use anyhow::Result;
use serde_json::ser::PrettyFormatter;

use crate::formats::{MyDeserializer, MySerializer};

pub struct Des<R: Read> {
	inner: serde_json::Deserializer<serde_json::de::IoRead<R>>,
}

impl<R: Read> MyDeserializer<R> for Des<R> {
	fn new(reader: R) -> Result<Self> {
		Ok(Self {
			inner: serde_json::Deserializer::from_reader(reader),
		})
	}
	fn deserializer(&mut self) -> impl serde::de::Deserializer<'_> {
		&mut self.inner
	}
}

pub struct Ser<W: Write> {
	inner: serde_json::Serializer<W, PrettyFormatter<'static>>,
}

impl<W: Write> MySerializer<W> for Ser<W> {
	fn new(writer: W) -> Self {
		Self {
			inner: serde_json::Serializer::pretty(writer),
		}
	}
	fn serializer(&mut self) -> impl serde::Serializer {
		&mut self.inner
	}
}
