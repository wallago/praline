use crate::app::engine::condition::strip_conditionals;
use crate::app::engine::context::Ctx;
use crate::app::engine::slot::fill_slots;
use crate::prelude::*;

/// Renders one template: slots, then `{if:}` blocks, then placeholders.
pub(super) fn substitute(content: &str, ctx: &Ctx) -> Result<Vec<u8>> {
    let filled = fill_slots(content, &ctx.slots);
    let mut out = strip_conditionals(&filled, ctx)?;
    for (key, value) in &ctx.vars {
        out = out.replace(&format!("{{{key}}}"), value);
    }
    Ok(out.into_bytes())
}
