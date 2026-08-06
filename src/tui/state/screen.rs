/// Which screen the TUI is currently showing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    /// Form, navigating fields.
    Form,
    /// Form, editing the focused field.
    Editing,
    /// Generated repo preview.
    Generated,
    /// Export destination picker.
    Exported,
}
