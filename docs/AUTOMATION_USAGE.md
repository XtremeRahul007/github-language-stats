# GitHub Actions Automation

This guide explains how to integrate GitHub Language Stats into another repository using GitHub Actions.

Once configured, GitHub Actions will automatically generate language statistics, update your README, and commit the generated files.

## Step 1. Copy the Rust CLI Project

Copy the contents of this repository into your target repository's `.github/scripts/` directory.

Your target repository should have this structure:

```text
your-project/
├── .github/
│   ├── scripts/
│   │   ├── Cargo.toml
│   │   ├── Cargo.lock
│   │   └── src/
│   │       ├── main.rs
│   │       ├── config.rs
│   │       ├── cloc.rs
│   │       ├── stats.rs
│   │       ├── colors.rs
│   │       ├── svg.rs
│   │       └── readme.rs
│   │
│   └── workflows/
│       └── language-stats.yml
│
├── docs/
│   └── language-stats.svg
│
└── README.md
```

## Step 2. Copy the GitHub Actions Workflow

Copy this workflow from this repository:

```text
./github-language-stats/src/workflows/language-stats.yml
```

Into your target repository:

```text
your-project/.github/workflows/language-stats.yml
```

Create the `workflows` directory if it doesn't already exist.

## Step 3. Configure the Workflow (If Step 2 is already complete, skip this)

Create or update `language-stats.yml` with the following configuration:

```yaml
name: Update Language Statistics

on:
  push:
    branches:
      - main # Change to your branch
    paths-ignore:
      - "README.md"
      - "docs/language-stats.svg"

  workflow_dispatch:

permissions:
  contents: write

concurrency:
  group: language-statistics
  cancel-in-progress: true

jobs:
  update-stats:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install cloc
        run: |
          sudo apt-get update
          sudo apt-get install -y cloc

      - name: Generate language statistics
        run: |
          cargo run --release --manifest-path .github/scripts/Cargo.toml -- \
            --input - \
            --exclude-dir=node_modules,target,dist,build,.git,.github,.venv,venv,coverage,pkg \
            --exclude-lang=JSON,Markdown,TOML,SVG \
            --output docs/language-stats.svg \
            --readme README.md \
            --max-rows 8

      - name: Commit generated files
        run: |
          git config user.name "github-actions[bot]"
          git config user.email "41898282+github-actions[bot]@users.noreply.github.com"

          git add README.md docs/language-stats.svg

          if git diff --cached --quiet; then
            echo "No changes to commit."
          else
            git commit -m "chore: update language statistics"
            git push
          fi
```

**Note:** Change `main` if your repository uses another branch.

## Available Options

| Option | Default | Description |
|---|---|---|
| `--input` | `stats.json` | Input cloc JSON file; use `-` to run cloc |
| `--output` | `docs/language-stats.svg` | Generated SVG path |
| `--readme` | `README.md` | README file to update |
| `--max-rows` | `8` | Maximum chart rows, including Other |
| `--exclude-dir` | None | Comma-separated directories to exclude |
| `--exclude-lang` | None | Comma-separated languages to exclude |

## Step 4. Enable Repository Permissions

Go to:

`Settings → Actions → General → Workflow permissions`

Enable **Read and write permissions**.

This allows GitHub Actions to commit the generated SVG and README changes.

## Step 5. Add README Markers

Add the following marker pair to your target repository's README, exactly where you want the chart:

```html
<!-- LANG_STATS_START -->
<!-- LANG_STATS_END -->
```

Keep exactly one start marker and one end marker outside Markdown code blocks.

## Step 6. Test the Automation

Commit and push your changes to the configured branch.

You can also trigger the workflow manually:

`GitHub Repository → Actions → Update Language Statistics → Run workflow`

Once successful, GitHub Actions will automatically update your language statistics.

[← Back to Main README](../README.md)
