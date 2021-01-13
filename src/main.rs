use std::io::{self, Read};

use serde_yaml;
use serde_json;


fn main() {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer).expect("Failed to read stdin");

    let value: serde_yaml::Value = serde_yaml::from_str(&buffer).expect("could not parse input as YAML");
    let output = serde_json::to_string(&value).expect("could not serialise output as JSON");

    print!("{}", output);
}
