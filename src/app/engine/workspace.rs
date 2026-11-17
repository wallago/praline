use super::context::active_in;
use crate::app::{App, engine::context::is_member, opt::OptId};

/// `rust.common`'s manifest, which holds the build profiles.
const MEMBER_MANIFEST: &str = include_str!("../../../templates/rust.common/Cargo.toml");

/// The build profiles of `rust.common`. Cargo only reads them at the
/// workspace root, so members drop them (`{ifnot:workspace}`) and the root
/// takes them over, word for word.
fn profiles() -> &'static str {
    let start = "# {ifnot:workspace}\n";
    MEMBER_MANIFEST
        .split_once(start)
        .and_then(|(_, rest)| rest.split_once("# {endif:workspace}"))
        .map_or("", |(profiles, _)| profiles)
}

/// The root `Cargo.toml`: every hosted Rust app is a member, sharing one
/// `Cargo.lock` and one `target/`; a firmware is excluded, since it builds
/// for its own target with its own `.cargo/config.toml`.
pub(super) fn manifest(app: &App) -> Vec<u8> {
    let quoted = |at: usize| format!("\"{}\"", app.prefix(at).display());
    let members: Vec<String> = (0..app.apps.len())
        .filter(|&at| is_member(app, at))
        .map(quoted)
        .collect();
    let excluded: Vec<String> = (0..app.apps.len())
        .filter(|&at| active_in(app, at).contains(&OptId::RustFirmware))
        .map(quoted)
        .collect();
    let mut lines = vec![
        "[workspace]".to_string(),
        "resolver = \"3\"".to_string(),
        format!("members  = [{}]", members.join(", ")),
    ];
    if !excluded.is_empty() {
        lines.push(format!("exclude  = [{}]", excluded.join(", ")));
    }
    let mut out = lines.join("\n");
    out.push_str("\n\n");
    out.push_str(profiles());
    out.into_bytes()
}
