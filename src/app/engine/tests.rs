//! Checks that hold for every option combination.

use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
    sync::LazyLock,
};

use include_dir::Dir;
use strum::IntoEnumIterator;
use yaml_rust2::{Yaml, YamlLoader};

use super::{
    Tree, generate,
    marker::marker,
    slot::{SLOT_MARKER, SLOTS_DIR},
};
use crate::{
    app::{
        App,
        opt::{Emit, OptId},
    },
    prelude::*,
};

/// Options that only add repo-wide files and checks. They meet the others
/// through `{if:}` blocks and slots, never by changing what another option
/// renders, so they aren't crossed with every other combination.
const REPO_WIDE: &[OptId] = &[
    OptId::Editorconfig,
    OptId::Typos,
    OptId::Lychee,
    OptId::Readme,
    OptId::Git,
    OptId::Github,
    OptId::GithubCi,
    OptId::CargoDeny,
    OptId::CargoAudit,
    OptId::CargoUdeps,
    OptId::CargoCodecov,
    OptId::Claude,
];

/// The largest conflict-free set of the other options.
const KITCHEN_SINK: &[OptId] = &[
    OptId::Just,
    OptId::Flake,
    OptId::FlakePackage,
    OptId::FlakeModuleHome,
    OptId::FlakeModuleNixos,
    OptId::Rust,
    OptId::RustCommon,
    OptId::RustServer,
    OptId::RustServerWS,
    OptId::RustCan,
    OptId::RustNotif,
    OptId::RustMcp,
    OptId::RustMcpHttp,
];

/// Every subset of `ids`.
fn subsets(ids: &[OptId]) -> impl Iterator<Item = HashSet<OptId>> + '_ {
    (0..1_u64 << ids.len()).map(move |mask| {
        ids.iter()
            .enumerate()
            .filter(|(at, _)| (mask >> at) & 1 == 1)
            .map(|(_, &id)| id)
            .collect()
    })
}

/// Top-level options that conflict with each other, each with its own
/// subtree. Each is crossed with every non-stack option on its own: crossing
/// them with each other only adds conflicting sets, and walking those
/// subsets would take most of the test time.
const STACKS: &[OptId] = &[OptId::Rust, OptId::Web];

/// The top-level option `id` hangs off.
fn root(id: OptId) -> OptId {
    id.def().parent.map_or(id, root)
}

/// One app per distinct active set. Every subset of the non-repo-wide options
/// the UI lets through (no two checked options conflict), with the repo-wide
/// ones all off and all on; then every subset of the repo-wide ones, on an
/// empty repo, on `just` + `flake`, and on the kitchen sink.
fn every_setup() -> Vec<(Vec<OptId>, App)> {
    let all_repo_wide: HashSet<OptId> = REPO_WIDE.iter().copied().collect();
    let mut candidates = Vec::new();
    for &stack in STACKS {
        let others: Vec<OptId> = OptId::iter()
            .filter(|id| !REPO_WIDE.contains(id))
            .filter(|&id| !STACKS.contains(&root(id)) || root(id) == stack)
            .collect();
        for base in subsets(&others) {
            candidates.push(base.union(&all_repo_wide).copied().collect());
            candidates.push(base);
        }
    }
    for repo_wide in subsets(REPO_WIDE) {
        for base in [&[][..], &[OptId::Just, OptId::Flake][..], KITCHEN_SINK] {
            candidates.push(repo_wide.iter().chain(base).copied().collect());
        }
    }

    let mut seen = HashSet::new();
    let mut setups = Vec::new();
    for checked in candidates {
        if checked
            .iter()
            .any(|&a| checked.iter().any(|&b| a.conflicts_with(b)))
        {
            continue;
        }
        let active: Vec<OptId> = OptId::iter().filter(|id| id.is_active(&checked)).collect();
        if !seen.insert(active.clone()) {
            continue;
        }
        let mut app = App {
            name: "demo-app".into(),
            desc: "Demo app.".into(),
            owner: "demo".into(),
            ..App::new(Args::default()).unwrap()
        };
        for opt in &mut app.apps[0].options {
            opt.checked = checked.contains(&opt.id);
        }
        setups.push((active, app));
    }
    setups
}

/// Every setup that renders, rendered once and shared by the checks below.
fn renders() -> &'static [(Vec<OptId>, Tree)] {
    static RENDERS: LazyLock<Vec<(Vec<OptId>, Tree)>> = LazyLock::new(|| {
        every_setup()
            .into_iter()
            .filter_map(|(active, app)| Some((active, generate(&app).ok()?)))
            .collect()
    });
    &RENDERS
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
    for (active, tree) in renders() {
        for (path, bytes) in tree {
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
    for (active, tree) in renders() {
        for (path, bytes) in tree {
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

#[test]
fn every_json_parses() {
    for (active, tree) in renders() {
        for (path, bytes) in tree {
            if path.extension().is_some_and(|ext| ext == "json") {
                let parsed = serde_json::from_slice::<serde_json::Value>(bytes);
                assert!(
                    parsed.is_ok(),
                    "{active:?}: {} is not JSON: {}",
                    path.display(),
                    parsed.err().map_or_else(String::new, |err| err.to_string())
                );
            }
        }
    }
}

/// `text` parsed as one YAML document, or why it isn't one.
fn yaml(text: &str) -> std::result::Result<Yaml, String> {
    YamlLoader::load_from_str(text)
        .map_err(|err| err.to_string())?
        .into_iter()
        .next()
        .ok_or_else(|| "empty document".to_string())
}

#[test]
fn every_yaml_parses() {
    for (active, tree) in renders() {
        for (path, bytes) in tree {
            if path
                .extension()
                .is_some_and(|ext| ext == "yml" || ext == "yaml")
            {
                let parsed = std::str::from_utf8(bytes)
                    .map_err(|err| err.to_string())
                    .and_then(yaml);
                assert!(
                    parsed.is_ok(),
                    "{active:?}: {} is not YAML: {}",
                    path.display(),
                    parsed.err().unwrap_or_default()
                );
            }
        }
    }
}

/// Recipe name → command lines, over the rendered `justfile` and `just/*.just`.
fn just_recipes(tree: &Tree) -> HashMap<String, Vec<String>> {
    let mut recipes = HashMap::new();
    for (path, bytes) in tree {
        let is_just =
            path == Path::new("justfile") || path.extension().is_some_and(|ext| ext == "just");
        if !is_just {
            continue;
        }
        let mut current: Option<&mut Vec<String>> = None;
        for line in String::from_utf8_lossy(bytes).lines() {
            if line.starts_with(char::is_whitespace) {
                let command = line.trim();
                if let Some(body) = current.as_mut()
                    && !command.is_empty()
                    && !command.starts_with('#')
                {
                    body.push(command.to_string());
                }
                continue;
            }
            current = None;
            let header = !line.is_empty()
                && !line.starts_with(['#', '['])
                && !line.starts_with("import ")
                && !line.contains(":=")
                && line.contains(':');
            if header {
                let name = line
                    .split([' ', ':'])
                    .next()
                    .unwrap_or_default()
                    .to_string();
                current = Some(recipes.entry(name).or_default());
            }
        }
    }
    recipes
}

#[test]
fn every_just_file_is_imported() {
    for (active, tree) in renders() {
        let Some(justfile) = tree.get(Path::new("justfile")) else {
            continue;
        };
        let justfile = String::from_utf8_lossy(justfile);
        for path in tree.keys().filter(|path| path.starts_with("just")) {
            let import = format!("import '{}'", path.display());
            assert!(
                justfile.lines().any(|line| line == import),
                "{active:?}: the justfile never imports {}",
                path.display()
            );
        }
    }
}

/// The recipes `just ci` calls, in order. Panics on a line that isn't
/// `@just <recipe>`: CI runs recipes, so `ci` may only call recipes.
fn ci_recipes(active: &[OptId], recipes: &HashMap<String, Vec<String>>) -> Vec<String> {
    let lines = recipes.get("ci").into_iter().flatten();
    for line in lines.clone() {
        assert!(
            line.starts_with("@just "),
            "{active:?}: `ci` runs `{line}`, not a recipe"
        );
    }
    lines
        .filter_map(|line| line.strip_prefix("@just "))
        .map(str::to_string)
        .collect()
}

#[test]
fn github_ci_runs_exactly_the_just_ci_recipes() {
    for (active, tree) in renders() {
        let Some(workflow) = tree.get(Path::new(".github/workflows/ci.yml")) else {
            continue;
        };
        let workflow = yaml(std::str::from_utf8(workflow).unwrap()).unwrap();
        let matrix = workflow["jobs"]["check"]["strategy"]["matrix"]["recipe"].as_vec();
        assert!(matrix.is_some(), "{active:?}: the check matrix is empty");
        let mut in_github: Vec<&str> = matrix
            .into_iter()
            .flatten()
            .filter_map(Yaml::as_str)
            .collect();
        let recipes = just_recipes(tree);
        let mut in_just = ci_recipes(active, &recipes);
        in_github.sort_unstable();
        in_just.sort_unstable();
        assert_eq!(in_github, in_just, "{active:?}");
    }
}

#[test]
fn every_recipe_claude_md_names_exists() {
    for (active, tree) in renders() {
        let Some(claude_md) = tree.get(Path::new("CLAUDE.md")) else {
            continue;
        };
        let recipes = just_recipes(tree);
        let claude_md = String::from_utf8_lossy(claude_md);
        for named in claude_md.split("`just ").skip(1) {
            let recipe = named.split(['`', ' ']).next().unwrap_or_default();
            assert!(
                recipe.starts_with('-') || recipes.contains_key(recipe),
                "{active:?}: CLAUDE.md names `just {recipe}`, which doesn't exist"
            );
        }
    }
}

/// The nixpkgs package providing the program a just command line starts with,
/// or `None` for a shell builtin.
fn package_of(command: &str) -> Option<String> {
    let mut words = command
        .trim_start_matches(['@', '-'])
        .split_whitespace()
        .skip_while(|word| word.contains('='));
    let program = words.next()?;
    if matches!(program, "cd" | "echo" | "test" | "true") {
        return None;
    }
    if program == "cargo" {
        let sub = words.next().unwrap_or_default();
        let plugin = matches!(sub, "deny" | "audit" | "machete" | "nextest" | "llvm-cov");
        return Some(if plugin {
            format!("cargo-{sub}")
        } else {
            "rust".into()
        });
    }
    Some(program.to_string())
}

#[test]
fn every_tool_just_ci_runs_is_in_the_flake_ci_shell() {
    for (active, tree) in renders() {
        let Some(flake) = tree.get(Path::new("flake.nix")) else {
            continue;
        };
        let flake = String::from_utf8_lossy(flake);
        let ci_tools: HashSet<&str> = flake
            .lines()
            .skip_while(|line| !line.contains("ciTools = with pkgs; ["))
            .skip(1)
            .take_while(|line| !line.contains("];"))
            .map(str::trim)
            .collect();
        let recipes = just_recipes(tree);
        // `nix develop .#ci --command just <recipe>` needs `just` itself.
        if recipes.contains_key("ci") {
            assert!(
                ci_tools.contains("just"),
                "{active:?}: no `just` in ciTools"
            );
        }
        for recipe in ci_recipes(active, &recipes) {
            assert!(
                recipes.contains_key(&recipe),
                "{active:?}: `ci` calls missing recipe `{recipe}`"
            );
            for package in recipes[&recipe]
                .iter()
                .flat_map(|line| line.split("&&"))
                .filter_map(package_of)
            {
                assert!(
                    ci_tools.contains(package.as_str()),
                    "{active:?}: `just {recipe}` needs `{package}`, not in ciTools {ci_tools:?}"
                );
            }
        }
    }
}

/// Category slugs the templates may use. crates.io refuses any slot not in
/// <https://crates.io/category_slugs>, so copy new ones from there.
const CRATES_IO_CATEGORIES: &[&str] = &[
    "command-line-utilities",
    "hardware-support",
    "web-programming::http-server",
    "web-programming::websocket",
];

/// `[package].<key>` of `manifest`, as strings.
fn package_list<'a>(manifest: &'a toml::Table, key: &str) -> Vec<&'a str> {
    manifest
        .get("package")
        .and_then(|package| package.get(key))
        .and_then(toml::Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(toml::Value::as_str)
        .collect()
}

/// Whether crates.io takes `keyword`: ASCII, at most 20 characters, starts
/// with a letter, then letters, digits, `_`, `-` or `+`.
fn is_crates_io_keyword(keyword: &str) -> bool {
    keyword.len() <= 20
        && keyword
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic())
        && keyword
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '+'))
}

#[test]
fn every_cargo_manifest_passes_crates_io_metadata_rules() {
    for (active, tree) in renders() {
        let Some(bytes) = tree.get(Path::new("Cargo.toml")) else {
            continue;
        };
        let manifest: toml::Table = toml::from_str(std::str::from_utf8(bytes).unwrap()).unwrap();
        for key in ["keywords", "categories"] {
            let list = package_list(&manifest, key);
            assert!(list.len() <= 5, "{active:?}: more than 5 {key}: {list:?}");
            let unique: HashSet<_> = list.iter().collect();
            assert_eq!(
                unique.len(),
                list.len(),
                "{active:?}: repeated {key}: {list:?}"
            );
        }
        for keyword in package_list(&manifest, "keywords") {
            assert!(
                is_crates_io_keyword(keyword),
                "{active:?}: bad keyword `{keyword}`"
            );
        }
        for category in package_list(&manifest, "categories") {
            assert!(
                CRATES_IO_CATEGORIES.contains(&category),
                "{active:?}: unknown category `{category}`"
            );
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
    // Filled by the engine itself, from the list of apps.
    contributed.insert("workspace.apps");
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

/// `deps` as `(name, version range)` pairs, sorted.
fn json_deps(deps: Option<&serde_json::Value>) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = deps
        .and_then(serde_json::Value::as_object)
        .into_iter()
        .flatten()
        .map(|(name, range)| (name.clone(), range.as_str().unwrap_or_default().to_owned()))
        .collect();
    out.sort();
    out
}

/// `deps` of the lockfile's root importer as `(name, version range)` pairs, sorted.
fn lock_deps(deps: &Yaml) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = deps
        .as_hash()
        .into_iter()
        .flatten()
        .map(|(name, dep)| {
            (
                name.as_str().unwrap_or_default().to_owned(),
                dep["specifier"].as_str().unwrap_or_default().to_owned(),
            )
        })
        .collect();
    out.sort();
    out
}

#[test]
fn web_lockfile_matches_package_json() {
    let (_, tree) = renders()
        .iter()
        .find(|(active, _)| active.contains(&OptId::WebReact))
        .expect("no setup renders web.react");
    let manifest: serde_json::Value =
        serde_json::from_slice(&tree[Path::new("package.json")]).unwrap();
    let lock = yaml(std::str::from_utf8(&tree[Path::new("pnpm-lock.yaml")]).unwrap()).unwrap();
    let root = &lock["importers"]["."];
    for key in ["dependencies", "devDependencies"] {
        assert_eq!(
            json_deps(manifest.get(key)),
            lock_deps(&root[key]),
            "{key} in package.json and pnpm-lock.yaml differ: run `just web-lock`"
        );
    }
}

/// The render of a repo with these `--app` specs and repo-wide `opts`.
fn render_apps(apps: &[&str], opts: &[&str]) -> Tree {
    let app = App::new(Args {
        name: Some("demo-repo".into()),
        owner: Some("demo".into()),
        desc: Some("Demo repo.".into()),
        app: apps.iter().map(|&a| a.into()).collect(),
        opt: opts.iter().map(|&o| o.into()).collect(),
        ..Args::default()
    })
    .unwrap();
    generate(&app).unwrap()
}

/// Three apps, two of them Rust, with the whole repo-wide tooling.
fn three_apps() -> Tree {
    let mut opts: Vec<&str> = REPO_WIDE.iter().map(|&id| id.into()).collect();
    opts.extend(["just", "flake"]);
    opts.retain(|&opt| opt != "cargo.codecov");
    render_apps(
        &[
            "server:rust.server,rust.server.ws",
            "client:rust.client_ws,rust.notif",
            "site:web.react",
        ],
        &opts,
    )
}

/// `path` of `tree` as text.
fn text<'a>(tree: &'a Tree, path: &str) -> &'a str {
    let bytes = tree.get(Path::new(path));
    assert!(bytes.is_some(), "no `{path}` in {:?}", tree.keys());
    std::str::from_utf8(bytes.unwrap()).unwrap()
}

#[test]
fn several_apps_each_render_in_their_own_folder() {
    let tree = three_apps();
    for path in [
        "apps/server/Cargo.toml",
        "apps/server/src/server/mod.rs",
        "apps/client/src/client_ws/mod.rs",
        "apps/site/package.json",
        "apps/site/src/App.tsx",
        "apps/server/justfile",
        "flake.nix",
        "justfile",
        ".github/workflows/ci.yml",
    ] {
        assert!(tree.contains_key(Path::new(path)), "no `{path}`");
    }
    assert!(
        !tree.keys().any(|path| path.starts_with("src")),
        "an app leaked to the root"
    );
}

#[test]
fn an_app_keeps_its_own_dependencies() {
    let tree = three_apps();
    assert!(text(&tree, "apps/server/Cargo.toml").contains("axum"));
    assert!(!text(&tree, "apps/client/Cargo.toml").contains("axum"));
    assert!(text(&tree, "apps/client/Cargo.toml").contains("notify-rust"));
}

#[test]
fn the_shared_flake_gets_every_app_tool_once() {
    let tree = three_apps();
    let flake = text(&tree, "flake.nix");
    for tool in ["rust-overlay.url", "rust-analyzer", "taplo", "pnpm"] {
        assert_eq!(flake.matches(tool).count(), 1, "`{tool}` in flake.nix");
    }
}

#[test]
fn the_root_cargo_toml_is_a_workspace_of_the_rust_apps() {
    let tree = three_apps();
    let root: toml::Table = toml::from_str(text(&tree, "Cargo.toml")).unwrap();
    let members = root["workspace"]["members"].as_array().unwrap();
    let members: Vec<&str> = members.iter().filter_map(toml::Value::as_str).collect();
    assert_eq!(members, ["apps/server", "apps/client"]);
    // Cargo only reads profiles at the root.
    assert!(root.contains_key("profile"));
    let member: toml::Table = toml::from_str(text(&tree, "apps/server/Cargo.toml")).unwrap();
    assert!(!member.contains_key("profile"));
    assert_eq!(member["package"]["name"].as_str(), Some("server"));
    assert_eq!(
        member["package"]["repository"].as_str(),
        Some("https://github.com/demo/demo-repo")
    );
}

#[test]
fn a_firmware_stays_out_of_the_workspace() {
    let tree = render_apps(&["host:rust.server", "fw:rust.firmware"], &["just"]);
    let root: toml::Table = toml::from_str(text(&tree, "Cargo.toml")).unwrap();
    let excluded = root["workspace"]["exclude"].as_array().unwrap();
    assert_eq!(excluded[0].as_str(), Some("apps/fw"));
}

#[test]
fn the_root_gate_runs_each_app_ci_and_ci_runs_the_same() {
    let tree = three_apps();
    let justfile = text(&tree, "justfile");
    for app in ["server", "client", "site"] {
        assert!(
            justfile.contains(&format!("mod {app} 'apps/{app}'")),
            "{app}"
        );
        let app_justfile = text(&tree, &format!("apps/{app}/justfile"));
        assert!(app_justfile.contains("ci:"), "{app} has no ci recipe");
        // The repo-wide checks stay at the root.
        assert!(!app_justfile.contains("typos"), "{app} runs typos");
    }
    let recipes = just_recipes(&tree);
    let mut in_just = ci_recipes(&[], &recipes);
    let workflow = yaml(text(&tree, ".github/workflows/ci.yml")).unwrap();
    let mut in_github: Vec<String> = workflow["jobs"]["check"]["strategy"]["matrix"]["recipe"]
        .as_vec()
        .unwrap()
        .iter()
        .filter_map(Yaml::as_str)
        .map(str::to_string)
        .collect();
    in_just.sort_unstable();
    in_github.sort_unstable();
    assert_eq!(in_just, in_github);
    assert!(in_just.iter().any(|recipe| recipe == "site::ci"));
}

#[test]
fn every_file_of_a_multi_app_render_parses() {
    let tree = three_apps();
    for (path, bytes) in &tree {
        let text = std::str::from_utf8(bytes).unwrap();
        for left in [SLOT_MARKER, "{if:", "{ifnot:", "{endif:"] {
            assert!(!text.contains(left), "{} keeps `{left}`", path.display());
        }
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("toml") => assert!(
                toml::from_str::<toml::Table>(text).is_ok(),
                "{}",
                path.display()
            ),
            Some("json") => assert!(
                serde_json::from_str::<serde_json::Value>(text).is_ok(),
                "{}",
                path.display()
            ),
            Some("yml" | "yaml") => assert!(yaml(text).is_ok(), "{}", path.display()),
            _ => {}
        }
    }
}

#[test]
fn single_app_only_options_drop_out_with_several_apps() {
    let tree = render_apps(
        &["site:web.pages", "api:rust.server"],
        &["just", "github.ci"],
    );
    assert!(!tree.keys().any(|path| path.ends_with("pages.yml")));
}
