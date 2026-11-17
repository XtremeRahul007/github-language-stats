#[cfg(test)]
mod tests {
    use crate::{
        stats::{LanguageEntry, LanguageStats},
        svg::render_svg,
    };

    fn sample_entry(name: &str, code: u32, percentage: f64) -> LanguageEntry {
        LanguageEntry {
            name: name.to_string(),
            stats: LanguageStats {
                n_files: 1,
                blank: 0,
                comment: 0,
                code,
            },
            percentage,
        }
    }

    #[test]
    fn renders_svg_with_language_color() {
        let entries = vec![sample_entry("Rust", 500, 50.0)];

        let svg = render_svg(&entries);

        assert!(svg.contains("<svg"));
        assert!(svg.contains("Language Statistics"));
        assert!(svg.contains("#dea584"));
        assert!(svg.contains("50.0%"));
        assert!(svg.contains("500"));
    }

    #[test]
    fn escapes_language_names() {
        let entries = vec![sample_entry("C++ & Rust", 100, 100.0)];

        let svg = render_svg(&entries);

        assert!(svg.contains("C++ &amp; Rust"));
        assert!(!svg.contains("C++ & Rust</text>"));
    }

    #[test]
    fn handles_empty_entries() {
        let svg = render_svg(&[]);

        assert!(svg.contains("No language statistics available"));
        assert!(svg.contains("</svg>"));
    }

    #[test]
    fn handles_invalid_percentage() {
        let entries = vec![sample_entry("Rust", 100, f64::NAN)];

        let svg = render_svg(&entries);

        assert!(svg.contains("0.0%"));
        assert!(!svg.contains("NaN"));
    }
}
