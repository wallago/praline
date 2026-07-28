use crate::tui::state::State;

impl State {
    /// Returns the key bindings.
    pub fn get_key_bindings(&self) -> Vec<(&str, &str)> {
        vec![("q", "Quit"), ("i/e", "prev/next")]
    }
}
