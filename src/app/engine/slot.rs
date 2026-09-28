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
/// disappears. Blocks are filled in turn, so a block can hold slots of its
/// own.
pub(super) fn fill_slots(content: &str, slots: &Slots) -> String {
    fill(content, slots, &mut Vec::new())
}

/// [`fill_slots`], dropping any slot already being expanded in `open` so a
/// block that names its own slot can't recurse forever.
fn fill(content: &str, slots: &Slots, open: &mut Vec<String>) -> String {
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
        if open.iter().any(|o| o == name) {
            continue;
        }
        let indent = &line[..line.len() - line.trim_start().len()];
        open.push(name.to_string());
        for block in slots.get(name).into_iter().flatten() {
            for entry in fill(block, slots, open).lines() {
                if !entry.is_empty() {
                    out.push_str(indent);
                }
                out.push_str(entry);
                out.push('\n');
            }
        }
        open.pop();
    }
    out
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    /// `Slots` holding each `(name, block)` pair, in the order given.
    fn slots(entries: &[(&str, &str)]) -> Slots {
        let mut slots = Slots::new();
        for &(name, block) in entries {
            slots
                .entry(name.to_string())
                .or_default()
                .push(block.to_string());
        }
        slots
    }

    #[test]
    fn the_block_replaces_the_whole_marker_line() {
        let hub = "[dependencies]\n# {slot:deps}\n\n[dev-dependencies]\n";
        let filled = fill_slots(hub, &slots(&[("deps", "axum = \"0.8\"\n")]));
        assert_eq!(
            filled,
            "[dependencies]\naxum = \"0.8\"\n\n[dev-dependencies]\n"
        );
    }

    #[test]
    fn every_line_gets_the_marker_indent() {
        let hub = "ci:\n    # {slot:ci}\n";
        let filled = fill_slots(hub, &slots(&[("ci", "cargo test\ncargo clippy\n")]));
        assert_eq!(filled, "ci:\n    cargo test\n    cargo clippy\n");
    }

    #[test]
    fn blank_lines_stay_unindented() {
        let hub = "fn run() {\n    // {slot:body}\n}\n";
        let filled = fill_slots(hub, &slots(&[("body", "a();\n\nb();\n")]));
        assert_eq!(filled, "fn run() {\n    a();\n\n    b();\n}\n");
    }

    #[test]
    fn blocks_come_out_in_contribution_order() {
        let both = slots(&[("deps", "a = 1\n"), ("deps", "b = 2\n")]);
        assert_eq!(fill_slots("# {slot:deps}\n", &both), "a = 1\nb = 2\n");
    }

    #[test]
    fn a_slot_nobody_fills_disappears() {
        assert_eq!(fill_slots("a\n# {slot:deps}\nb\n", &Slots::new()), "a\nb\n");
    }

    #[test]
    fn a_block_can_hold_a_slot_of_its_own() {
        let nested = slots(&[
            ("outer", "select! {\n    // {slot:inner}\n}\n"),
            ("inner", "a();\n"),
        ]);
        assert_eq!(
            fill_slots("fn run() {\n    // {slot:outer}\n}\n", &nested),
            "fn run() {\n    select! {\n        a();\n    }\n}\n"
        );
    }

    #[test]
    fn a_block_naming_its_own_slot_stops() {
        let looped = slots(&[("deps", "a = 1\n# {slot:deps}\n")]);
        assert_eq!(fill_slots("# {slot:deps}\n", &looped), "a = 1\n");
    }
}
