use std::{
    fs,
    io::{Error, ErrorKind, Result},
    path::Path,
};

const START: &str = "<!-- LANG_STATS_START -->";
const END: &str = "<!-- LANG_STATS_END -->";

pub fn update_readme(path: &Path, image_path: &str) -> Result<()> {
    let content = fs::read_to_string(path)?;

    let start_count = content.matches(START).count();
    let end_count = content.matches(END).count();

    if start_count != 1 || end_count != 1 {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "README must contain exactly one start and end marker",
        ));
    }

    let start = content.find(START).unwrap();
    let end = content.find(END).unwrap();

    if start >= end {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "README markers are incorrectly ordered",
        ));
    }

    let replacement = format!(
        "{START}\n<p align=\"center\"><img src=\"{image_path}\" alt=\"Language breakdown\" width=\"480\"></p>\n{END}"
    );

    let updated = format!(
        "{}{}{}",
        &content[..start],
        replacement,
        &content[end + END.len()..]
    );

    let temp_path = path.with_extension("tmp");

    fs::write(&temp_path, updated)?;
    fs::rename(&temp_path, path)?;

    Ok(())
}
