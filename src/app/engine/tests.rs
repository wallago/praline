//! Checks that hold for every option combination.

use std::{collections::HashSet, fs, path::Path};

use include_dir::Dir;
use strum::IntoEnumIterator;

use super::{
    generate,
    marker::marker,
    slot::{SLOT_MARKER, SLOTS_DIR},
};
use crate::app::{
    App,
    opt::{Emit, OptId},
};

/// One app per distinct active set, reached by ticking every subset of options
/// the UI lets through: no two checked options conflict.
fn every_setup() -> Vec<(Vec<OptId>, App)> {
    let ids: Vec<OptId> = OptId::iter().collect();
    let mut seen = HashSet::new();
    let mut setups = Vec::new();
    for mask in 0..1_u64 << ids.len() {
        let checked: HashSet<OptId> = ids
            .iter()
            .enumerate()
            .filter(|(at, _)| (mask >> at) & 1 == 1)
            .map(|(_, &id)| id)
            .collect();
        if checked
            .iter()
            .any(|&a| checked.iter().any(|&b| a.conflicts_with(b)))
        {
            continue;
        }
        let active: Vec<OptId> = ids
            .iter()
            .copied()
            .filter(|id| id.is_active(&checked))
            .collect();
        if !seen.insert(active.clone()) {
            continue;
        }
        let mut app = App {
            name: "demo-app".into(),
            desc: "Demo app.".into(),
            owner: "demo".into(),
            ..App::default()
        };
        for opt in &mut app.options {
            opt.checked = checked.contains(&opt.id);
        }
        setups.push((active, app));
    }
    setups
}

#[test]
fn every_setup_renders() {
    for (active, app) in every_setup() {
        let tree = generate(&app);
        assert!(tree.is_ok(), "{active:?}: {:?}", tree.err());
    }
}

#[test]
fn no_marker_survives_rendering() {
    for (active, app) in every_setup() {
        for (path, bytes) in &generate(&app).unwrap() {
            let text = String::from_utf8_lossy(bytes);
            for left in [SLOT_MARKER, "{if:", "{ifnot:", "{endif:"] {
                assert!(
                    !text.contains(left),
                    "{active:?}: {} keeps `{left}`",
                    path.display()
                );
            }
        }
    }
}

#[test]
fn every_toml_parses() {
    for (active, app) in every_setup() {
        for (path, bytes) in &generate(&app).unwrap() {
            if path.extension().is_some_and(|ext| ext == "toml") {
                let parsed = std::str::from_utf8(bytes)
                    .map_err(|error| error.to_string())
                    .and_then(|text| {
                        toml::from_str::<toml::Table>(text).map_err(|error| error.to_string())
                    });
                assert!(
                    parsed.is_ok(),
                    "{active:?}: {} is not TOML:\n{}",
                    path.display(),
                    parsed.err().unwrap_or_default()
                );
            }
        }
    }
}

/// Every UTF-8 file under `dir`, recursively.
fn texts(dir: &'static Dir<'static>) -> Vec<&'static str> {
    let mut out: Vec<&str> = dir
        .files()
        .filter_map(|file| file.contents_utf8())
        .collect();
    for sub in dir.dirs() {
        out.extend(texts(sub));
    }
    out
}

#[test]
fn every_slot_has_a_hub_and_a_contributor() {
    let mut hubs = HashSet::new();
    let mut contributed = HashSet::new();
    for id in OptId::iter() {
        let Emit::Dir(dir) = id.def().emit else {
            continue;
        };
        if let Some(slots) = dir.get_dir(SLOTS_DIR) {
            contributed.extend(
                slots
                    .files()
                    .filter_map(|file| file.path().file_name()?.to_str()),
            );
        }
        hubs.extend(
            texts(dir)
                .into_iter()
                .flat_map(str::lines)
                .filter_map(|line| marker(line, SLOT_MARKER)),
        );
    }
    let orphans: Vec<_> = contributed.difference(&hubs).collect();
    assert!(
        orphans.is_empty(),
        "contributed to, never declared: {orphans:?}"
    );
    let empty: Vec<_> = hubs.difference(&contributed).collect();
    assert!(
        empty.is_empty(),
        "declared, never contributed to: {empty:?}"
    );
}

#[test]
fn every_template_folder_is_rendered_by_its_option() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("templates");
    let unrendered: Vec<String> = fs::read_dir(root)
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .filter(|name| {
            !matches!(
                name.parse::<OptId>().map(|id| id.def().emit),
                Ok(Emit::Dir(_))
            )
        })
        .collect();
    assert!(
        unrendered.is_empty(),
        "never rendered from templates/: {unrendered:?}"
    );
}
