use std::collections::HashMap;

use include_dir::Dir;

use crate::app::engine::marker::marker;

/// Marker a hub file puts, alone on its line, where contributions go.
pub(super) const SLOT_MARKER: &str = "{slot:";

/// Slot name → blocks contributed to it, in option order.
pub(super) type Slots = HashMap<String, Vec<String>>;

/// Folder in an option's templates holding its slot contributions.
pub(super) const SLOTS_DIR: &str = "_slots";

/// Adds each `_slots/<name>` file of `dir` to `slots[<name>]`.
pub(super) fn collect_slots(dir: &Dir<'_>, slots: &mut Slots) {
    let Some(contrib) = dir.get_dir(SLOTS_DIR) else {
        return;
    };
    for file in contrib.files() {
        if let (Some(name), Some(text)) = (file.path().file_name(), file.contents_utf8()) {
            slots
                .entry(name.to_string_lossy().into_owned())
                .or_default()
                .push(text.to_string());
        }
    }
}

/// Replaces each `{slot:<name>}` line with the blocks contributed to
/// `<name>`, every line indented like the marker line. A slot nobody fills
/// disappears.
pub(super) fn fill_slots(content: &str, slots: &Slots) -> String {
    if !content.contains(SLOT_MARKER) {
        return content.to_string();
    }
    let mut out = String::with_capacity(content.len());
    for line in content.lines() {
        let Some(name) = marker(line, SLOT_MARKER) else {
            out.push_str(line);
            out.push('\n');
            continue;
        };
        let indent = &line[..line.len() - line.trim_start().len()];
        for block in slots.get(name).into_iter().flatten() {
            for entry in block.lines() {
                if !entry.is_empty() {
                    out.push_str(indent);
                }
                out.push_str(entry);
                out.push('\n');
            }
        }
    }
    out
}
