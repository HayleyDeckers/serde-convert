A small utility for changing between different file formats using serde. Such as yaml => json, json => toml or toml => json.
It can easilly be extended to support other formats, as long as a serde (de)serializer exists.

Round trips are not guarenteed to be idempotent. If you do `serde-convert -a --input=Cargo.toml --to json; serde-convert -a --file=Cargo.json --to toml` you will not get back the same layout of  `Cargo.toml` but it should still contain all the data (XML deserialization seems to need work). Use at your own risk.
