use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Padding},
};

use super::{placeholder::Category, state::Level};

pub(crate) const BACKGROUND: Color = Color::Rgb(36, 53, 42);
pub(crate) const ASCENT: Color = Color::Rgb(75, 227, 116);
pub(crate) const ASCENT_BIS: Color = Color::Rgb(22, 53, 33);
pub(crate) const PRIMARY: Color = Color::Rgb(207, 233, 212);
pub(crate) const SECONDARY: Color = Color::Rgb(103, 128, 112);
pub(crate) const HIGHLIGHT: Color = Color::Rgb(255, 190, 60);

// /// Color a category is listed in.
// pub(crate) const fn category(category: Category) -> Color {
//     match category {
//         Category::Git => ORANGE,
//         Category::Env => EMERALD,
//         Category::Build => BLUE,
//         Category::Format => CYAN,
//         Category::Lint => PURPLE,
//         Category::Test => YELLOW,
//         Category::Security => RED,
//         Category::Release => GREEN,
//     }
// }
//
// /// Color of a footer message's level tag.
// pub(crate) const fn level(level: Level) -> Color {
//     match level {
//         Level::Info => BLUE,
//         Level::Done => GREEN,
//         Level::Warn => PURPLE,
//         Level::Error => RED,
//     }
// }

// /// Sprout's pane frame: rounded, centered title, cyan with `> ` when focused.
// pub(crate) fn pane(title: &str, focused: bool) -> Block<'static> {
//     let (color, title) = if focused {
//         (CYAN, format!("> {title}"))
//     } else {
//         (GREEN, title.to_owned())
//     };
//     Block::bordered()
//         .border_type(BorderType::Rounded)
//         .border_style(color)
//         .title_top(Line::from(title).centered())
//         .padding(Padding::horizontal(1))
// }
//
// /// Sprout's `n of m` list counter, for a pane's bottom border.
// pub(crate) fn counter(selected: Option<usize>, len: usize) -> Line<'static> {
//     let at = match len {
//         0 => 0,
//         len => selected.unwrap_or(0).min(len - 1) + 1,
//     };
//     Line::from(vec![
//         Span::styled(format!(" {at} of {len} "), CYAN),
//         Span::raw("─"),
//     ])
//     .right_aligned()
// }
//
// /// Selected-row style: reverse video while the list has focus.
// pub(crate) fn selection(focused: bool) -> Style {
//     if focused {
//         Style::new().reversed()
//     } else {
//         Style::new().bold()
//     }
// }
//
// /// `key what │ key what` hints: keys bold, descriptions gray.
// pub(crate) fn hints(pairs: &[(String, &'static str)]) -> Vec<Span<'static>> {
//     let mut spans = Vec::new();
//     for (at, (key, what)) in pairs.iter().enumerate() {
//         if at > 0 {
//             spans.push(Span::styled(" │ ", GRAY));
//         }
//         spans.push(Span::raw(key.clone()).bold());
//         spans.push(Span::styled(format!(" {what}"), GRAY));
//     }
//     spans
// }
