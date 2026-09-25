use std::collections::HashSet;

use strum::IntoEnumIterator;

use crate::app::engine::slot::{Slots, collect_slots};
use crate::app::opt::Emit;
use crate::app::{App, opt::OptId};
use crate::error::EngineError::UnknownOption;
use crate::prelude::*;

/// What every template can read. Built once per render, read-only after.
#[derive(Debug)]
pub(crate) struct Ctx {
    /// `{key}` placeholders and their values.
    pub vars: Vec<(&'static str, String)>,
    /// Options taking part: checked, with their parents and `requires` on.
    pub active: HashSet<OptId>,
    /// Contributions of the active options, in `OptId` order.
    pub slots: Slots,
}

impl Ctx {
    /// Snapshots the builder: its fields, what's active, and the slots.
    pub(crate) fn new(repo: &App) -> Self {
        let checked: HashSet<OptId> = repo
            .options
            .iter()
            .filter(|opt| opt.checked)
            .map(|opt| opt.id)
            .collect();
        let mut active = HashSet::new();
        let mut slots = Slots::new();
        for id in OptId::iter().filter(|&id| id.is_active(&checked)) {
            active.insert(id);
            if let Emit::Dir(dir) = id.def().emit {
                collect_slots(dir, &mut slots);
            }
        }
        Self {
            vars: vec![
                ("ident", ident(&repo.name)),
                ("name", repo.name.clone()),
                ("desc", repo.desc.clone()),
                ("owner", repo.owner.clone()),
                // Facts about the combination go here, computed once, e.g.
                // ("release_bin", release_bin(&active, &repo.name)),
            ],
            active,
            slots,
        }
    }

    /// Answers `{if:<name>}`. An unknown name is an error, not a silent "off".
    pub(crate) fn is_on(&self, name: &str) -> Result<bool> {
        let id: OptId = name
            .parse()
            .map_err(|_| <EngineError as Into<Error>>::into(UnknownOption(name.to_string())))?;
        Ok(self.active.contains(&id))
    }
}

#[must_use]
pub(super) fn ident(name: &str) -> String {
    let mut out: String = name
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();
    if out.starts_with(|c: char| c.is_ascii_digit()) {
        out.insert(0, '_');
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn child_is_inactive_while_its_parent_is_off() {
        assert!(!OptId::RustFirmware.is_active(&HashSet::from([OptId::RustFirmware])));
        assert!(OptId::RustFirmware.is_active(&HashSet::from([OptId::Rust, OptId::RustFirmware])));
    }
}
