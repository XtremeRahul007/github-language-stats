#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::stats::{LanguageStats, parse_cloc_json, process_languages};

    #[test]
    fn parses_cloc_json() {
        let json = r#"
        {
            "header": {
                "n_files": 2,
                "n_lines": 100
            },
            "Rust": {
                "nFiles": 1,
                "blank": 10,
                "comment": 5,
                "code": 35
            },
            "TypeScript": {
                "nFiles": 1,
                "blank": 20,
                "comment": 10,
                "code": 20
            },
            "SUM": {
                "nFiles": 2,
                "blank": 30,
                "comment": 15,
                "code": 55
            }
        }
        "#;

        let result = parse_cloc_json(json).unwrap();

        assert_eq!(result.header.n_files, 2);
        assert_eq!(result.languages.len(), 2);
        assert_eq!(result.languages["Rust"].code, 35);
        assert_eq!(result.languages["TypeScript"].code, 20);
    }

    #[test]
    fn groups_remaining_languages_into_other() {
        let languages: HashMap<String, LanguageStats> = (1..=10)
            .map(|i| {
                (
                    format!("Language{i}"),
                    LanguageStats {
                        n_files: 1,
                        blank: 0,
                        comment: 0,
                        code: i * 10,
                    },
                )
            })
            .collect();

        let result = process_languages(languages, 8);

        assert_eq!(result.len(), 8);
        assert_eq!(result[0].name, "Language10");
        assert_eq!(result[6].name, "Language4");
        assert_eq!(result[7].name, "Other");
        assert_eq!(result[7].stats.code, 60);
    }
}
