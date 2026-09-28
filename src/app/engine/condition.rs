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

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use rstest::rstest;

    use super::*;
    use crate::app::engine::slot::Slots;
    use crate::app::opt::OptId;

    /// `b` kept only while `just` is on.
    const IF_JUST: &str = "a\n# {if:just}\nb\n# {endif:just}\nc\n";
    /// `b` kept only while `just` is off.
    const IFNOT_JUST: &str = "a\n# {ifnot:just}\nb\n# {endif:just}\nc\n";
    /// A `just` block inside a `rust` block.
    const NESTED: &str = "# {if:rust}\na\n# {if:just}\nb\n# {endif:just}\nc\n# {endif:rust}\nd\n";

    /// A context where exactly `active` is on.
    fn ctx(active: &[OptId]) -> Ctx {
        Ctx {
            vars: Vec::new(),
            active: active.iter().copied().collect(),
            slots: Slots::new(),
        }
    }
    #[rstest]
    #[case::if_on(IF_JUST, &[OptId::Just], "a\nb\nc\n")]
    #[case::if_off(IF_JUST, &[], "a\nc\n")]
    #[case::ifnot_on(IFNOT_JUST, &[OptId::Just], "a\nc\n")]
    #[case::ifnot_off(IFNOT_JUST, &[], "a\nb\nc\n")]
    #[case::outer_off_drops_inner(NESTED, &[OptId::Just], "d\n")]
    #[case::inner_off_keeps_outer(NESTED, &[OptId::Rust], "a\nc\nd\n")]
    #[case::both_on(NESTED, &[OptId::Rust, OptId::Just], "a\nb\nc\nd\n")]
    fn keeps_a_block_only_while_its_condition_holds(
        #[case] template: &str,
        #[case] active: &[OptId],
        #[case] expected: &str,
    ) {
        assert_eq!(
            strip_conditionals(template, &ctx(active)).unwrap(),
            expected
        );
    }

    #[rstest]
    #[case::in_if("# {if:nope}\n# {endif:nope}\n")]
    #[case::in_ifnot("# {ifnot:nope}\n# {endif:nope}\n")]
    #[case::inside_a_dropped_block("# {if:just}\n# {if:nope}\n# {endif:nope}\n# {endif:just}\n")]
    fn an_unknown_option_is_an_error(#[case] template: &str) {
        let err = strip_conditionals(template, &ctx(&[])).unwrap_err();
        assert!(
            matches!(&err, Error::Engine(EngineError::UnknownOption(name)) if name == "nope"),
            "{err}"
        );
    }
}
