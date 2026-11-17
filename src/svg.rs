use crate::stats::LanguageEntry;

const WIDTH: u32 = 480;
const PADDING: u32 = 20;

const BAR_X: u32 = 135;
const BAR_WIDTH: u32 = 205;
const BAR_HEIGHT: u32 = 10;

const FIRST_ROW: u32 = 60;
const ROW_HEIGHT: u32 = 36;

pub fn render_svg(entries: &[LanguageEntry]) -> String {
    let height = FIRST_ROW + entries.len() as u32 * ROW_HEIGHT + PADDING;

    let mut svg = String::new();

    svg.push_str(&format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" width="{WIDTH}" height="{height}" viewBox="0 0 {WIDTH} {height}">"#
    ));

    svg.push_str(&format!(
        r#"<rect width="{WIDTH}" height="{height}" fill="{{#0d1117}}"/>"#
    ));

    svg.push_str(
        r#"<style>
            .name { fill: #e6edf3; font-size: 13px; font-weight: 600; }
            .muted { fill: #8b949e; font-size: 12px; }
            .track { fill: #30363d; }
        </style>"#,
    );

    for (index, entry) in entries.iter().enumerate() {
        let y = FIRST_ROW + index as u32 * ROW_HEIGHT;

        let bar_width = BAR_WIDTH as f64 * entry.percentage / 100.0;

        svg.push_str(&format!(
            r#"<text x="{PADDING}" y="{y}" class="name">{}</text>"#,
            escape_xml(&entry.name)
        ));

        svg.push_str(&format!(
            r#"<rect x="{BAR_X}" y="{}" width="{BAR_WIDTH}" height="{BAR_HEIGHT}" rx="5" class="track"/>"#,
            y - 10
        ));

        svg.push_str(&format!(
            r#"<rect x="{BAR_X}" y="{}" width="{bar_width:.2}" height="{BAR_HEIGHT}" rx="5" fill="{{#58a6ff}}"/>"#,
            y - 10
        ));

        svg.push_str(&format!(
            r#"<text x="{}" y="{y}" class="muted">{:.1}%</text>"#,
            BAR_X + BAR_WIDTH + 12,
            entry.percentage
        ));

        svg.push_str(&format!(
            r#"<text x="{}" y="{y}" class="muted">{}</text>"#,
            BAR_X + BAR_WIDTH + 65,
            entry.stats.code
        ));
    }

    svg.push_str("</svg>");

    svg
}

fn escape_xml(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}
