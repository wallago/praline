use std::{cell::LazyCell, path::Path};

use ratatui::text::Text;
use syntect_assets::assets::HighlightingAssets;
use tui_syntax_highlight::Highlighter;

thread_local! {
    static ASSETS: LazyCell<HighlightingAssets> = LazyCell::new(HighlightingAssets::from_binary);
}

/// Highlights source into styled lines. The syntax is chosen from the
/// file's extension, falling back to the first line, then to plain text.
pub(crate) fn highlight(path: &Path, content: &str) -> (Text<'static>, String) {
    ASSETS.with(|assets| {
        let Ok(syntax_set) = assets.get_syntax_set() else {
            return (Text::raw(content.to_string()), String::from("None"));
        };

        let syntax = path
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext| syntax_set.find_syntax_by_extension(ext))
            .or_else(|| {
                content
                    .lines()
                    .next()
                    .and_then(|line| syntax_set.find_syntax_by_first_line(line))
            })
            .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

        (
            Highlighter::new(assets.get_theme("ansi").clone())
                .highlight_reader(content.as_bytes(), syntax, syntax_set)
                .unwrap_or_else(|_| Text::raw(content.to_string())),
            syntax.name.clone(),
        )
    })
}
