use std::fs;

mod cloc;
mod config;
mod readme;
mod stats;
mod svg;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = fs::read_to_string("stats.json")?;

    let stats = stats::parse_cloc_json(&json);

    println!("{stats:#?}");
    Ok(())
}
