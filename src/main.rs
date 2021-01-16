use std::io::{self, Read, Write};

use anyhow::{Context, Result};
use clap::{self, crate_name, crate_version};
use serde_json;
use serde_yaml;

fn clap_app() -> clap::App<'static, 'static> {
    clap::App::new(crate_name!())
        .version(crate_version!())
        .about("Convert YAML to JSON and back again")
        .arg(
            clap::Arg::with_name("reverse")
                .long("reverse")
                .short("r")
                .help("Convert JSON to YAML")
                .takes_value(false)
                .multiple(false),
        )
}

fn yaml_to_json<R: Read, W: Write>(reader: R, writer: W) -> Result<()> {
    let value: serde_yaml::Value =
        serde_yaml::from_reader(reader).with_context(|| "Could not parse input as YAML")?;
    serde_json::to_writer(writer, &value).with_context(|| "Could not serialise output as JSON")
}

fn json_to_yaml<R: Read, W: Write>(reader: R, writer: W) -> Result<()> {
    let value: serde_json::Value =
        serde_json::from_reader(reader).with_context(|| "Could not parse input as JSON")?;
    serde_yaml::to_writer(writer, &value).with_context(|| "Could not serialise output as YAML")
}

fn main() -> Result<()> {
    let matches = clap_app().get_matches();

    if matches.is_present("reverse") {
        json_to_yaml(io::stdin(), io::stdout())?
    } else {
        yaml_to_json(io::stdin(), io::stdout())?
    }

    Ok(())
}
