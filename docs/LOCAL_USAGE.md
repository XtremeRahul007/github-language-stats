# Local Usage

This guide explains how to install and use GitHub Language Stats locally.

## Requirements

- Rust (stable toolchain)
- [cloc](https://github.com/AlDanial/cloc)

## Installation

Clone the repository and build:

```bash
git clone https://github.com/XtremeRahul007/github-language-stats.git
cd github-language-stats
cargo build --release
```

The executable will be available under `target/release/`.

## Generate Statistics

Generate statistics directly from the current repository:

```bash
github-language-stats \
  --input - \
  --exclude-dir=node_modules,target,dist,.git,.github \
  --exclude-lang=JSON,Markdown,TOML \
  --output docs/language-stats.svg \
  --readme README.md \
  --max-rows 8
```

You can also provide an existing cloc JSON file:

```bash
github-language-stats --input stats.json
```

## Available Options

| Option | Default | Description |
|---|---|---|
| `--input` | `stats.json` | Input cloc JSON file; use `-` to run cloc |
| `--output` | `docs/language-stats.svg` | Generated SVG path |
| `--readme` | `README.md` | README file to update |
| `--max-rows` | `8` | Maximum chart rows, including Other |
| `--exclude-dir` | None | Comma-separated directories to exclude |
| `--exclude-lang` | None | Comma-separated languages to exclude |
| `--help` | — | Display help |
| `--version` | — | Display version |

## README Integration

You can automatically display the generated language statistics in your repository's README.

1. Add the following marker pair exactly where you want the chart to appear:

   ```html
   <!-- LANG_STATS_START -->
   <!-- LANG_STATS_END -->
   ```

2. The tool replaces the content between these markers with the generated SVG image.

3. Configure the output path using `--output` and provide your README path using `--readme`.

**Important:** Keep exactly one start marker and one end marker in your README. Place them outside Markdown code blocks so the generated image renders correctly.

[← Back to Main README](../README.md)
