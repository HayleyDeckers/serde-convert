mod formats;

use std::io::{BufReader, Read};
use std::{fs::File, io::Write};
use std::{path::PathBuf, str::FromStr};

use anyhow::{anyhow, Context, Result};
use clap::Parser;

#[derive(Clone, Copy, Debug, clap::ValueEnum, PartialEq, Eq)]
enum Format {
    Json,
    Yaml,
    Toml,
    XML,
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
            Ok(Self::XML)
        } else {
            Err(())
        }
    }
}

/// Convert between various config file-formats
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// file to convert
    #[arg(long)]
    file: String,

    /// Format to convert into
    #[arg(long)]
    to: Format,

    /// Format of the input file, will be guessed based on extension of the input file if not given
    #[arg(long)]
    from: Option<Format>,
}

fn convert2<R: Read, W: Write>(from: Format, to: Format, reader: R, writer: W) -> Result<()> {
    fn inner_convert<
        FROM: formats::MyDeserializer<R>,
        TO: formats::MySerializer<W>,
        R: Read,
        W: Write,
    >(
        reader: R,
        writer: W,
    ) -> Result<()> {
        let mut from = FROM::new(reader)?;
        let mut to = TO::new(writer);

        let de = from.deserializer();
        let se = to.serializer();
        //this map err is a bit meh but works.
        serde_transcode::transcode(de, se).map_err(|e| anyhow!("{e:?}"))?;
        to.flush()
    }

    use formats::*;
    use Format::*;

    //todo: macro this.
    match (from, to) {
        //json to...
        (Json, Toml) => inner_convert::<json::Des<R>, toml::Ser<W>, R, W>(reader, writer),
        (Json, Yaml) => inner_convert::<json::Des<R>, yaml::Ser<W>, R, W>(reader, writer),
        (Json, XML) => inner_convert::<json::Des<R>, xml::Ser<W>, R, W>(reader, writer),
        (Json, Json) => Ok(()),
        //toml to...
        (Toml, Json) => inner_convert::<toml::Des<R>, json::Ser<W>, R, W>(reader, writer),
        (Toml, Yaml) => inner_convert::<toml::Des<R>, yaml::Ser<W>, R, W>(reader, writer),
        (Toml, XML) => inner_convert::<toml::Des<R>, xml::Ser<W>, R, W>(reader, writer),
        (Toml, Toml) => Ok(()),
        //xml to...
        (XML, Json) => inner_convert::<xml::Des<R>, json::Ser<W>, R, W>(reader, writer),
        (XML, Yaml) => inner_convert::<xml::Des<R>, yaml::Ser<W>, R, W>(reader, writer),
        (XML, Toml) => inner_convert::<xml::Des<R>, toml::Ser<W>, R, W>(reader, writer),
        (XML, XML) => Ok(()),
        //yaml to...
        (Yaml, Json) => inner_convert::<yaml::Des<R>, json::Ser<W>, R, W>(reader, writer),
        (Yaml, Toml) => inner_convert::<yaml::Des<R>, toml::Ser<W>, R, W>(reader, writer),
        (Yaml, XML) => inner_convert::<yaml::Des<R>, xml::Ser<W>, R, W>(reader, writer),
        (Yaml, Yaml) => Ok(()),
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let filename = PathBuf::from(args.file);
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
    let out_file = filename.with_extension(format!("{:?}", args.to).to_lowercase());
    if args.to != from {
        let writer = File::create(out_file).context("failed to make output file")?;
        convert2(from, args.to, reader, writer)
    } else {
        Err(anyhow::anyhow!("Input and output format are the same"))
    }
}
