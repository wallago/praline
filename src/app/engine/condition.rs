use crate::app::engine::context::Ctx;
use crate::app::engine::marker::marker;
use crate::prelude::*;

/// Marker opening a tool-conditional template block.
const IF_MARKER: &str = "{if:";
/// Marker closing a tool-conditional template block.
const ENDIF_MARKER: &str = "{endif:";
/// Marker filtering a tool-conditional template block.
const IFNOT_MARKER: &str = "{ifnot:";

/// Drops `{if:<tool>}`/`{ifnot:<tool>}`/`{endif:<tool>}` blocks whose tool is not selected,
/// keeping the body of the ones whose tool is. The marker lines themselves are
/// removed either way.
pub(super) fn strip_conditionals(content: &str, ctx: &Ctx) -> Result<String> {
    if !content.contains(IF_MARKER) && !content.contains(IFNOT_MARKER) {
        return Ok(content.to_string());
    }
    let mut out = String::with_capacity(content.len());
    let mut skipping: Option<&str> = None;
    for line in content.lines() {
        if let Some(tool) = marker(line, ENDIF_MARKER) {
            if skipping == Some(tool) {
                skipping = None;
            }
            continue;
        } else if let Some(tool) = marker(line, IF_MARKER) {
            let on = ctx.is_on(tool)?;
            if skipping.is_none() && !on {
                skipping = Some(tool);
            }
            continue;
        } else if let Some(tool) = marker(line, IFNOT_MARKER) {
            let on = ctx.is_on(tool)?;
            if skipping.is_none() && on {
                skipping = Some(tool);
            }
            continue;
        }
        if skipping.is_none() {
            out.push_str(line);
            out.push('\n');
        }
    }
    Ok(out)
}
