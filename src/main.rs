use anyhow::Result;
use clap::Parser;
use convat_json2yaml::convert;
use std::io::{self, Read};

#[derive(Parser)]
#[command(name = "convat-json2yaml", version = "0.1.0", about = "Convert JSON-lines stdin to YAML (one document per line)")]
struct Args {}

fn main() -> Result<()> {
    let _args = Args::parse();
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let output = convert(&input)?;
    print!("{output}");
    Ok(())
}
