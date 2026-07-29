use ratatui::style::Color;

/// The kind of toast, which drives its accent color.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ToastType {
    /// Informational message.
    Info,
    /// Success message.
    Success,
    /// Warning message.
    Warning,
    /// Error message.
    Error,
}

impl ToastType {
    /// Accent color for this toast kind.
    pub fn color(self) -> Color {
        match self {
            Self::Info => Color::Blue,
            Self::Success => Color::Green,
            Self::Warning => Color::Yellow,
            Self::Error => Color::Red,
        }
    }
}

/// A toast notification: a short message rendered on a single row.
pub struct Toast {
    /// Message text.
    pub message: String,
    /// Toast kind (drives the accent color).
    pub kind: ToastType,
}
