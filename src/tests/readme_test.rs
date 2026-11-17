#[cfg(test)]
mod tests {
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    use crate::readme::update_readme;

    const START: &str = "<!-- LANG_STATS_START -->";
    const END: &str = "<!-- LANG_STATS_END -->";

    fn temp_readme() -> std::path::PathBuf {
        let id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!("readme-test-{id}.md"))
    }

    #[test]
    fn replaces_content_between_markers() {
        let path = temp_readme();

        let original = format!("Before\n{START}\nOld content\n{END}\nAfter");

        fs::write(&path, original).unwrap();

        update_readme(&path, "docs/language-stats.svg").unwrap();

        let updated = fs::read_to_string(&path).unwrap();

        assert!(updated.starts_with("Before\n"));
        assert!(updated.ends_with("\nAfter"));

        assert!(updated.contains(r#"<img src="docs/language-stats.svg""#));

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn rejects_missing_markers() {
        let path = temp_readme();

        fs::write(&path, "No markers here").unwrap();

        let result = update_readme(&path, "chart.svg");

        assert!(result.is_err());

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn rejects_duplicate_markers() {
        let path = temp_readme();

        let content = format!("{START}\n{END}\n{START}\n{END}");

        fs::write(&path, content).unwrap();

        let result = update_readme(&path, "chart.svg");

        assert!(result.is_err());

        fs::remove_file(path).unwrap();
    }
}
