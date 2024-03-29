mod formats;

use std::{
	fs::File,
	io::{BufReader, Read, Write},
	path::PathBuf,
	str::FromStr,
};

use anyhow::{anyhow, Context, Result};
use clap::Parser;

#[derive(Clone, Copy, Debug, clap::ValueEnum, PartialEq, Eq)]
enum Format {
	Json,
	Yaml,
	Toml,
	Xml,
}

impl FromStr for Format {
	type Err = ();
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		if s.eq_ignore_ascii_case("yaml") | s.eq_ignore_ascii_case("yml") {
			Ok(Self::Yaml)
		} else if s.eq_ignore_ascii_case("json") {
			Ok(Self::Json)
		} else if s.eq_ignore_ascii_case("toml") {
			Ok(Self::Toml)
		} else if s.eq_ignore_ascii_case("xml") {
			Ok(Self::Xml)
		} else {
			Err(())
		}
	}
}

/// Convert between various config file-formats.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
	/// file to convert
	#[arg(long, short)]
	input: String,

	/// Format to convert into
	#[arg(long, short)]
	to: Format,

	/// Format of the input file, will be guessed based on extension of the input file if not given
	#[arg(long, short)]
	from: Option<Format>,

	/// automatically determine output filename and write to it instead of stdout
	#[arg(long, short)]
	auto: bool,
}

fn convert<R: Read, W: Write>(from: Format, to: Format, reader: R, writer: W) -> Result<()> {
	fn inner_convert<R: Read, W: Write>(
		mut from: impl formats::MyDeserializer<R>,
		mut to: impl formats::MySerializer<W>,
	) -> Result<()> {
		let de = from.deserializer();
		let se = to.serializer();
		//this map err is a bit meh but works.
		serde_transcode::transcode(de, se)
			.map_err(|e| anyhow!("conversion failed with error \"{e}\""))?;
		to.flush()
	}

	use formats::*;
	use Format::*;

	//todo: macro this.
	match from {
		Json => {
			let from = json::Des::new(reader)?;
			match to {
				Json => Ok(()),
				Toml => inner_convert(from, toml::Ser::new(writer)),
				Xml => inner_convert(from, xml::Ser::new(writer)),
				Yaml => inner_convert(from, yaml::Ser::new(writer)),
			}
		}
		Toml => {
			let from = toml::Des::new(reader)?;
			match to {
				Json => inner_convert(from, json::Ser::new(writer)),
				Toml => Ok(()),
				Xml => inner_convert(from, xml::Ser::new(writer)),
				Yaml => inner_convert(from, yaml::Ser::new(writer)),
			}
		}
		Xml => {
			let from = xml::Des::new(reader)?;
			match to {
				Json => inner_convert(from, json::Ser::new(writer)),
				Toml => inner_convert(from, toml::Ser::new(writer)),
				Xml => Ok(()),
				Yaml => inner_convert(from, yaml::Ser::new(writer)),
			}
		}
		Yaml => {
			let from = yaml::Des::new(reader)?;
			match to {
				Json => inner_convert(from, json::Ser::new(writer)),
				Toml => inner_convert(from, toml::Ser::new(writer)),
				Xml => inner_convert(from, xml::Ser::new(writer)),
				Yaml => Ok(()),
			}
		}
	}
}

fn main() -> Result<()> {
	let args = Args::parse();
	let filename = PathBuf::from(args.input);
	let from = if let Some(from) = args.from {
		from
	} else {
		filename
			.extension()
			.context("Couldn't determine input filetype")?
			.to_str()
			.context("failed to convert OsStr to str")?
			.parse()
			.map_err(|_| anyhow::anyhow!("unsupported extension"))?
	};
	let reader = BufReader::new(File::open(&filename).context("failed to open file")?);

	if args.to != from {
		if args.auto {
			let out_file = filename.with_extension(format!("{:?}", args.to).to_lowercase());
			let writer = File::create(out_file).context("failed to make output file")?;
			convert(from, args.to, reader, writer)
		} else {
			convert(from, args.to, reader, std::io::stdout())
		}
	} else {
		Err(anyhow::anyhow!("Input and output format are the same"))
	}
}
