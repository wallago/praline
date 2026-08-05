use core::fmt;
use std::str::FromStr;

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::Deserialize;

/// Configurable key bindings, one field per action.
#[derive(Clone, Debug, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Keybindings {
    /// Quit the application.
    pub quit: KeyPattern,
    /// Scroll UP.
    pub scroll_up: KeyPattern,
    /// Scroll DOWN.
    pub scroll_down: KeyPattern,
    /// Generate repo.
    pub generate: KeyPattern,
    /// Export repo.
    pub export: KeyPattern,
    /// Create repo.
    pub create: KeyPattern,
    /// Leave focus.
    pub leave: KeyPattern,
    /// Confirm.
    pub confirm: KeyPattern,
    /// Enter.
    pub enter: KeyPattern,
}

impl Default for Keybindings {
    fn default() -> Self {
        Self {
            quit: KeyPattern::plain(KeyCode::Char('q')),
            generate: KeyPattern::plain(KeyCode::Char('g')),
            export: KeyPattern::plain(KeyCode::Char('x')),
            create: KeyPattern::plain(KeyCode::Char('c')),
            scroll_up: KeyPattern::plain(KeyCode::Char('k')),
            scroll_down: KeyPattern::plain(KeyCode::Char('j')),
            leave: KeyPattern::plain(KeyCode::Esc),
            confirm: KeyPattern::plain(KeyCode::Enter),
            enter: KeyPattern::plain(KeyCode::Enter),
        }
    }
}

/// A single key chord, e.g. `q`, `esc` or `ctrl+d`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyPattern {
    /// Key code.
    pub code: KeyCode,
    /// Modifier keys that must be held.
    pub modifiers: KeyModifiers,
}

impl KeyPattern {
    /// A modifier-less key chord; used for compile-time known defaults.
    const fn plain(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::NONE,
        }
    }

    /// Whether this pattern matches a pressed key event.
    pub(crate) fn matches(&self, event: &KeyEvent) -> bool {
        self.code == event.code && self.modifiers == event.modifiers
    }
}

impl<'de> Deserialize<'de> for KeyPattern {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

/// Parses a bare key name (no modifiers) into a [`KeyCode`].
fn parse_key_code(s: &str) -> Result<KeyCode, String> {
    Ok(match s {
        "esc" | "escape" => KeyCode::Esc,
        "enter" | "return" => KeyCode::Enter,
        "tab" => KeyCode::Tab,
        "backtab" => KeyCode::BackTab,
        "up" => KeyCode::Up,
        "down" => KeyCode::Down,
        "left" => KeyCode::Left,
        "right" => KeyCode::Right,
        "home" => KeyCode::Home,
        "end" => KeyCode::End,
        "pageup" => KeyCode::PageUp,
        "pagedown" => KeyCode::PageDown,
        "backspace" => KeyCode::Backspace,
        "delete" | "del" => KeyCode::Delete,
        "space" => KeyCode::Char(' '),
        other => {
            let mut chars = other.chars();
            match (chars.next(), chars.next()) {
                (Some(c), None) => KeyCode::Char(c),
                _ => return Err(format!("unknown key `{other}`")),
            }
        }
    })
}

impl FromStr for KeyPattern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut modifiers = KeyModifiers::NONE;
        let mut code = None;
        for part in s.split('+') {
            match part.trim().to_ascii_lowercase().as_str() {
                "ctrl" | "control" => modifiers |= KeyModifiers::CONTROL,
                "alt" => modifiers |= KeyModifiers::ALT,
                "shift" => modifiers |= KeyModifiers::SHIFT,
                key => {
                    if code.is_some() {
                        return Err(format!("`{s}`: more than one key"));
                    }
                    code = Some(parse_key_code(key)?);
                }
            }
        }
        Ok(Self {
            code: code.ok_or_else(|| format!("`{s}`: missing key"))?,
            modifiers,
        })
    }
}

impl fmt::Display for KeyPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.modifiers.contains(KeyModifiers::CONTROL) {
            write!(f, "Ctrl+")?;
        }
        if self.modifiers.contains(KeyModifiers::ALT) {
            write!(f, "Alt+")?;
        }
        if self.modifiers.contains(KeyModifiers::SHIFT) {
            write!(f, "⇧+")?;
        }
        match self.code {
            KeyCode::Esc => write!(f, "Esc"),
            KeyCode::Enter => write!(f, "⏎"),
            KeyCode::Tab => write!(f, "Tab"),
            KeyCode::BackTab => write!(f, "⇧+Tab"),
            KeyCode::Up => write!(f, "↑"),
            KeyCode::Down => write!(f, "↓"),
            KeyCode::Left => write!(f, "←"),
            KeyCode::Right => write!(f, "→"),
            KeyCode::Home => write!(f, "Home"),
            KeyCode::End => write!(f, "End"),
            KeyCode::PageUp => write!(f, "PgUp"),
            KeyCode::PageDown => write!(f, "PgDn"),
            KeyCode::Backspace => write!(f, "⌫"),
            KeyCode::Delete => write!(f, "Del"),
            KeyCode::Char(' ') => write!(f, "Space"),
            KeyCode::Char(c) => write!(f, "{c}"),
            other => write!(f, "{other:?}"),
        }
    }
}
