# GitHub Language Stats

A lightweight Rust CLI tool that generates language statistics SVG charts from [cloc](https://github.com/AlDanial/cloc) output and optionally updates your repository's README.

<!-- LANG_STATS_START -->
<p align="center"><img src="docs/language-stats.svg" alt="Language breakdown" width="480"></p>
<!-- LANG_STATS_END -->

## Features

- Generate SVG language statistics charts.
- Sort languages by code line count.
- Group smaller languages into `Other`.
- Customize excluded directories and languages.
- Automatically update README sections.
- Integrate with GitHub Actions.
- No external SVG rendering dependency.

## Documentation

| Guide                                        | Description                                             |
| -------------------------------------------- | ------------------------------------------------------- |
| [Local Usage](docs/LOCAL_USAGE.md)           | Installation, CLI usage, options and README integration |
| [Automation Usage](docs/AUTOMATION_USAGE.md) | Automate statistics generation using GitHub Actions     |
| [Samples](docs/SAMPLE.md)                    | Example charts and supported languages                  |

## Requirements

- Rust (stable toolchain)
- [cloc](https://github.com/AlDanial/cloc)

## License

MIT License. See [LICENSE](LICENSE).
