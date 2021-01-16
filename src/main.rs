use std::io::{self, Read};

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

fn yaml_to_json(buffer: &str) -> Result<String> {
    let value: serde_yaml::Value =
        serde_yaml::from_str(buffer).with_context(|| "Could not parse input as YAML")?;
    serde_json::to_string(&value).with_context(|| "Could not serialise output as JSON")
}

fn json_to_yaml(buffer: &str) -> Result<String> {
    let value: serde_json::Value =
        serde_json::from_str(buffer).with_context(|| "Could not parse input as JSON")?;
    serde_yaml::to_string(&value).with_context(|| "Could not serialise output as YAML")
}

fn main() -> Result<()> {
    let matches = clap_app().get_matches();

    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin
        .read_to_string(&mut buffer)
        .expect("Failed to read stdin");

    let output = if matches.is_present("reverse") {
        json_to_yaml(&buffer)
    } else {
        yaml_to_json(&buffer)
    }?;

    print!("{}", output);

    Ok(())
}
