use std::collections::HashMap;

use fugue_core::syntax::Highlight;
use once_cell::sync::Lazy;

pub use crate::graphics::{Color, Modifier, Style, UnderlineStyle};

#[derive(Clone, Debug, Default)]
pub struct Theme {
    name: String,

    // UI styles are stored in a HashMap
    styles: HashMap<String, Style>,
    // tree-sitter highlight styles are stored in a Vec to optimize lookups
    scopes: Vec<String>,
    highlights: Vec<Style>,
    rainbow_length: usize,
}

/// Hardcoded base16_transparent theme - cannot be changed
pub static THEME: Lazy<Theme> = Lazy::new(|| {
    let mut styles = HashMap::new();
    let mut scopes = Vec::new();
    let mut highlights = Vec::new();

    // Helper macro to add a style
    macro_rules! add_style {
        ($name:expr, $style:expr) => {
            styles.insert($name.to_string(), $style);
            scopes.push($name.to_string());
            highlights.push($style);
        };
    }

    // Rainbow styles (for bracket matching)
    let rainbow_styles = vec![
        Style::default().fg(Color::Red),
        Style::default().fg(Color::Yellow),
        Style::default().fg(Color::Green),
        Style::default().fg(Color::Blue),
        Style::default().fg(Color::Cyan),
        Style::default().fg(Color::Magenta),
    ];

    for (i, style) in rainbow_styles.iter().enumerate() {
        let name = format!("rainbow.{}", i);
        styles.insert(name.clone(), *style);
        scopes.push(name);
        highlights.push(*style);
    }
    let rainbow_length = rainbow_styles.len();

    // UI Styles - base16_transparent theme
    add_style!("ui.background", Style::default().fg(Color::White));
    add_style!("ui.background.separator", Style::default().fg(Color::Gray));
    add_style!("ui.text", Style::default().fg(Color::LightGray));
    add_style!("ui.text.focus", Style::default().fg(Color::White));
    add_style!("ui.menu", Style::default().fg(Color::White));
    add_style!(
        "ui.menu.selected",
        Style::default().add_modifier(Modifier::REVERSED)
    );
    add_style!("ui.menu.scroll", Style::default().fg(Color::LightGray));
    add_style!("ui.linenr", Style::default().add_modifier(Modifier::DIM));
    add_style!(
        "ui.linenr.selected",
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD)
    );
    add_style!("ui.popup", Style::default().fg(Color::White));
    add_style!("ui.window", Style::default().fg(Color::Gray));
    add_style!("ui.selection", Style::default().bg(Color::Gray));
    add_style!("ui.statusline", Style::default().fg(Color::White));
    add_style!("ui.statusline.inactive", Style::default().fg(Color::Gray));
    add_style!(
        "ui.statusline.normal",
        Style::default().fg(Color::Black).bg(Color::Blue)
    );
    add_style!(
        "ui.statusline.insert",
        Style::default().fg(Color::Black).bg(Color::Green)
    );
    add_style!(
        "ui.statusline.select",
        Style::default().fg(Color::Black).bg(Color::Magenta)
    );
    add_style!("ui.help", Style::default().fg(Color::LightGray));
    add_style!(
        "ui.cursor",
        Style::default().add_modifier(Modifier::REVERSED)
    );
    add_style!(
        "ui.cursor.match",
        Style::default()
            .fg(Color::LightYellow)
            .underline_color(Color::LightYellow)
            .underline_style(UnderlineStyle::Line)
    );
    add_style!(
        "ui.cursor.primary",
        Style::default().add_modifier(Modifier::REVERSED | Modifier::SLOW_BLINK)
    );
    add_style!(
        "ui.cursor.secondary",
        Style::default().add_modifier(Modifier::REVERSED)
    );
    add_style!(
        "ui.cursorline.primary",
        Style::default()
            .underline_color(Color::LightGray)
            .underline_style(UnderlineStyle::Line)
    );
    add_style!(
        "ui.cursorline.secondary",
        Style::default()
            .underline_color(Color::LightGray)
            .underline_style(UnderlineStyle::Line)
    );
    add_style!("ui.cursorcolumn.primary", Style::default().bg(Color::Gray));
    add_style!(
        "ui.cursorcolumn.secondary",
        Style::default().bg(Color::Gray)
    );
    add_style!("ui.virtual.ruler", Style::default().bg(Color::Gray));
    add_style!("ui.virtual.whitespace", Style::default().fg(Color::Gray));
    add_style!("ui.virtual.indent-guide", Style::default().fg(Color::Gray));
    add_style!(
        "ui.virtual.inlay-hint",
        Style::default().fg(Color::White).bg(Color::Gray)
    );
    add_style!(
        "ui.virtual.inlay-hint.parameter",
        Style::default().fg(Color::White).bg(Color::Gray)
    );
    add_style!(
        "ui.virtual.inlay-hint.type",
        Style::default().fg(Color::White).bg(Color::Gray)
    );
    add_style!("ui.virtual.wrap", Style::default().fg(Color::Gray));
    add_style!(
        "ui.virtual.jump-label",
        Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::BOLD)
            .underline_style(UnderlineStyle::Line)
    );
    add_style!("ui.gutter", Style::default().fg(Color::Gray));

    // Syntax Highlighting
    add_style!("comment", Style::default().fg(Color::Gray));
    add_style!("variable", Style::default().fg(Color::LightRed));
    add_style!("constant.numeric", Style::default().fg(Color::Yellow));
    add_style!("constant", Style::default().fg(Color::Yellow));
    add_style!("attribute", Style::default().fg(Color::Yellow));
    add_style!("type", Style::default().fg(Color::LightYellow));
    add_style!("string", Style::default().fg(Color::LightGreen));
    add_style!("variable.other.member", Style::default().fg(Color::Green));
    add_style!(
        "constant.character.escape",
        Style::default().fg(Color::LightCyan)
    );
    add_style!("function", Style::default().fg(Color::LightBlue));
    add_style!("constructor", Style::default().fg(Color::LightBlue));
    add_style!("special", Style::default().fg(Color::LightBlue));
    add_style!("keyword", Style::default().fg(Color::LightMagenta));
    add_style!("label", Style::default().fg(Color::LightMagenta));
    add_style!("namespace", Style::default().fg(Color::LightMagenta));

    // Markup
    add_style!("markup.heading", Style::default().fg(Color::LightBlue));
    add_style!("markup.list", Style::default().fg(Color::LightRed));
    add_style!(
        "markup.bold",
        Style::default()
            .fg(Color::LightYellow)
            .add_modifier(Modifier::BOLD)
    );
    add_style!(
        "markup.italic",
        Style::default()
            .fg(Color::LightMagenta)
            .add_modifier(Modifier::ITALIC)
    );
    add_style!(
        "markup.strikethrough",
        Style::default().add_modifier(Modifier::CROSSED_OUT)
    );
    add_style!(
        "markup.link.url",
        Style::default()
            .fg(Color::Yellow)
            .underline_color(Color::Yellow)
            .underline_style(UnderlineStyle::Line)
    );
    add_style!("markup.link.text", Style::default().fg(Color::LightRed));
    add_style!("markup.quote", Style::default().fg(Color::LightCyan));
    add_style!("markup.raw", Style::default().fg(Color::Green));
    add_style!("markup.normal", Style::default().fg(Color::Blue));
    add_style!("markup.insert", Style::default().fg(Color::Green));
    add_style!("markup.select", Style::default().fg(Color::Magenta));

    // Diff
    add_style!("diff.plus", Style::default().fg(Color::LightGreen));
    add_style!("diff.delta", Style::default().fg(Color::LightBlue));
    add_style!("diff.delta.moved", Style::default().fg(Color::Blue));
    add_style!("diff.minus", Style::default().fg(Color::LightRed));

    // Diagnostics
    add_style!("info", Style::default().fg(Color::LightBlue));
    add_style!("hint", Style::default().fg(Color::LightGray));
    add_style!("debug", Style::default().fg(Color::LightGray));
    add_style!("warning", Style::default().fg(Color::LightYellow));
    add_style!("error", Style::default().fg(Color::LightRed));

    add_style!(
        "diagnostic.info",
        Style::default()
            .underline_color(Color::LightBlue)
            .underline_style(UnderlineStyle::Dotted)
    );
    add_style!(
        "diagnostic.hint",
        Style::default()
            .underline_color(Color::LightGray)
            .underline_style(UnderlineStyle::DoubleLine)
    );
    add_style!(
        "diagnostic.debug",
        Style::default()
            .underline_color(Color::LightGray)
            .underline_style(UnderlineStyle::Dashed)
    );
    add_style!(
        "diagnostic.warning",
        Style::default()
            .underline_color(Color::LightYellow)
            .underline_style(UnderlineStyle::Curl)
    );
    add_style!(
        "diagnostic.error",
        Style::default()
            .underline_color(Color::LightRed)
            .underline_style(UnderlineStyle::Curl)
    );

    Theme {
        name: "base16_transparent".to_string(),
        styles,
        scopes,
        highlights,
        rainbow_length,
    }
});

impl Theme {
    /// To allow `Highlight` to represent arbitrary RGB colors without turning it into an enum,
    /// we interpret the last 256^3 numbers as RGB.
    const RGB_START: u32 = (u32::MAX << (8 + 8 + 8)) - 1 - (u32::MAX - Highlight::MAX);

    /// Interpret a Highlight with the RGB foreground
    fn decode_rgb_highlight(highlight: Highlight) -> Option<(u8, u8, u8)> {
        (highlight.get() > Self::RGB_START).then(|| {
            let [b, g, r, ..] = (highlight.get() + 1).to_le_bytes();
            (r, g, b)
        })
    }

    /// Create a Highlight that represents an RGB color
    pub fn rgb_highlight(r: u8, g: u8, b: u8) -> Highlight {
        // -1 because highlight is "non-max": u32::MAX is reserved for the null pointer
        // optimization.
        Highlight::new(u32::from_le_bytes([b, g, r, u8::MAX]) - 1)
    }

    #[inline]
    pub fn highlight(&self, highlight: Highlight) -> Style {
        if let Some((red, green, blue)) = Self::decode_rgb_highlight(highlight) {
            Style::new().fg(Color::Rgb(red, green, blue))
        } else {
            self.highlights[highlight.idx()]
        }
    }

    #[inline]
    pub fn scope(&self, highlight: Highlight) -> &str {
        &self.scopes[highlight.idx()]
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn get(&self, scope: &str) -> Style {
        self.try_get(scope).unwrap_or_default()
    }

    /// Get the style of a scope, falling back to dot separated broader
    /// scopes. For example if `ui.text.focus` is not defined in the theme,
    /// `ui.text` is tried and then `ui` is tried.
    pub fn try_get(&self, scope: &str) -> Option<Style> {
        std::iter::successors(Some(scope), |s| Some(s.rsplit_once('.')?.0))
            .find_map(|s| self.styles.get(s).copied())
    }

    /// Get the style of a scope, without falling back to dot separated broader
    /// scopes. For example if `ui.text.focus` is not defined in the theme, it
    /// will return `None`, even if `ui.text` is.
    pub fn try_get_exact(&self, scope: &str) -> Option<Style> {
        self.styles.get(scope).copied()
    }

    #[inline]
    pub fn scopes(&self) -> &[String] {
        &self.scopes
    }

    pub fn find_highlight_exact(&self, scope: &str) -> Option<Highlight> {
        self.scopes()
            .iter()
            .position(|s| s == scope)
            .map(|idx| Highlight::new(idx as u32))
    }

    pub fn find_highlight(&self, mut scope: &str) -> Option<Highlight> {
        loop {
            if let Some(highlight) = self.find_highlight_exact(scope) {
                return Some(highlight);
            }
            if let Some(new_end) = scope.rfind('.') {
                scope = &scope[..new_end];
            } else {
                return None;
            }
        }
    }

    pub fn is_16_color(&self) -> bool {
        self.styles.iter().all(|(_, style)| {
            [style.fg, style.bg]
                .into_iter()
                .all(|color| !matches!(color, Some(Color::Rgb(..))))
        })
    }

    pub fn rainbow_length(&self) -> usize {
        self.rainbow_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // tests for parsing an RGB `Highlight`

    #[test]
    fn convert_to_and_from() {
        let (r, g, b) = (0xFF, 0xFE, 0xFA);
        let highlight = Theme::rgb_highlight(r, g, b);
        assert_eq!(Theme::decode_rgb_highlight(highlight), Some((r, g, b)));
    }

    /// make sure we can store all the colors at the end
    #[test]
    fn full_numeric_range() {
        assert_eq!(Highlight::MAX - Theme::RGB_START, 256_u32.pow(3));
    }

    #[test]
    fn retrieve_color() {
        // color in the middle
        let (r, g, b) = (0x14, 0xAA, 0xF7);
        assert_eq!(
            THEME.highlight(Theme::rgb_highlight(r, g, b)),
            Style::new().fg(Color::Rgb(r, g, b))
        );
        // pure black
        let (r, g, b) = (0x00, 0x00, 0x00);
        assert_eq!(
            THEME.highlight(Theme::rgb_highlight(r, g, b)),
            Style::new().fg(Color::Rgb(r, g, b))
        );
        // pure white
        let (r, g, b) = (0xff, 0xff, 0xff);
        assert_eq!(
            THEME.highlight(Theme::rgb_highlight(r, g, b)),
            Style::new().fg(Color::Rgb(r, g, b))
        );
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn out_of_bounds() {
        let highlight = Highlight::new(Theme::rgb_highlight(0, 0, 0).get() - 1);
        THEME.highlight(highlight);
    }
}
