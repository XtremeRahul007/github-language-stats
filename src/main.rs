use std::{fs, path::Path};

mod cloc;
mod config;
mod readme;
mod stats;
mod svg;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let json = fs::read_to_string("stats.json")?;

    let stats = stats::parse_cloc_json(&json)?;

    let languages = stats::process_languages(stats.languages, 8);

    let svg = svg::render_svg(&languages);

    readme::update_readme(Path::new("README.md"), "docs/language-stats.svg")?;

    fs::create_dir_all("docs")?;
    fs::write("docs/language-stats.svg", svg)?;

    Ok(())
}
