use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    widgets::TableState,
};
use tui_input::{Input, backend::crossterm::EventHandler};

use super::placeholder::{self, PRESETS, Preset, TOOLS};
use crate::config::binds::Keybindings;

/// Rows a page key scrolls the preview by.
const PAGE: u16 = 10;

/// A pane that can take focus, numbered like sprout's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Pane {
    /// `[1]` repo identity line.
    Repo,
    /// `[2]` tool list.
    Tools,
    /// `[3]` files the active tools write.
    Files,
    /// `[4]` details of the last list used.
    Preview,
}

impl Pane {
    /// Every pane, in tab order.
    const ALL: [Self; 4] = [Self::Repo, Self::Tools, Self::Files, Self::Preview];

    /// The pane after this one in tab order, or before it.
    fn cycle(self, forward: bool) -> Self {
        let len = Self::ALL.len();
        let at = Self::ALL.iter().position(|pane| *pane == self).unwrap_or(0);
        let next = if forward { at + 1 } else { at + len - 1 };
        Self::ALL[next % len]
    }
}

/// What the preview describes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Subject {
    /// The tool selected in `[2]`.
    Tool,
    /// The file selected in `[3]`.
    File,
}

/// Severity of the footer message.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Level {
    /// Neutral feedback.
    Info,
    /// Something went through.
    Done,
    /// Needs attention.
    Warn,
    /// Refused.
    Error,
}

impl Level {
    /// Tag printed before the message.
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Info => "INFO",
            Self::Done => "DONE",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
        }
    }
}

/// A dialog drawn over the panes; it takes every key while open.
#[derive(Debug)]
pub(crate) enum Modal {
    /// Owner, name and description fields.
    Repo {
        /// The three inputs, in display order.
        fields: [Input; 3],
        /// Index of the focused field.
        focus: usize,
    },
    /// Preset picker.
    Preset(TableState),
    /// Destination prompt before writing.
    Generate(Input),
    /// Key reference.
    Help,
}

/// A generated file and the tool writing it.
#[derive(Clone, Copy, Debug)]
pub(crate) struct File {
    /// Path from the repo root.
    pub(crate) path: &'static str,
    /// Index of the writing tool in `TOOLS`.
    pub(crate) tool: usize,
}

/// What a key asks for, once bindings are resolved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Action {
    /// Leave the app.
    Quit,
    /// Move up or scroll up.
    Up,
    /// Move down or scroll down.
    Down,
    /// Back to the list the preview describes.
    Left,
    /// Over to the preview.
    Right,
    /// Scroll the preview a page up.
    PageUp,
    /// Scroll the preview a page down.
    PageDown,
    /// Next pane in tab order.
    NextPane,
    /// Previous pane in tab order.
    PrevPane,
    /// Jump to a pane.
    Focus(Pane),
    /// Tick or untick the selected tool.
    Toggle,
    /// The focused pane's main action.
    Open,
    /// Step back.
    Close,
    /// Open the repo dialog.
    Repo,
    /// Open the preset picker.
    Preset,
    /// Open the generate dialog.
    Generate,
    /// Open the key reference.
    Help,
    /// Unbound key.
    Nothing,
}

/// Everything the UI shows and edits.
#[derive(Debug)]
pub(crate) struct State {
    /// Cleared to leave the event loop.
    pub(crate) running: bool,
    /// Key bindings from `config.toml`.
    pub(crate) keys: Keybindings,
    /// Pane taking keys while no dialog is open.
    pub(crate) focus: Pane,
    /// What the preview describes.
    pub(crate) subject: Subject,
    /// Repo owner.
    pub(crate) owner: String,
    /// Repo name.
    pub(crate) name: String,
    /// Repo description.
    pub(crate) desc: String,
    /// Ticked state of each tool, parallel to `TOOLS`.
    pub(crate) checked: Vec<bool>,
    /// Selection in `[2]`.
    pub(crate) tools: TableState,
    /// Selection in `[3]`.
    pub(crate) files: TableState,
    /// Preview scroll, in rows.
    pub(crate) scroll: u16,
    /// Open dialog, if any.
    pub(crate) modal: Option<Modal>,
    /// Level of the footer message.
    pub(crate) level: Level,
    /// Footer message.
    pub(crate) message: String,
}

impl State {
    /// A state with placeholder repo fields and the `rust` preset applied.
    pub(crate) fn new(keys: Keybindings) -> Self {
        let mut state = Self {
            running: true,
            keys,
            focus: Pane::Tools,
            subject: Subject::Tool,
            owner: String::from("wallago"),
            name: String::from("tidy-crab"),
            desc: String::from("A tidy little crab."),
            checked: vec![false; TOOLS.len()],
            tools: TableState::new().with_selected(Some(0)),
            files: TableState::new().with_selected(Some(0)),
            scroll: 0,
            modal: None,
            level: Level::Info,
            message: String::from("placeholder data"),
        };
        if let Some(preset) = PRESETS.iter().find(|preset| preset.name == "rust") {
            state.apply(preset);
        }
        state
    }

    /// Index of the tool selected in `[2]`.
    pub(crate) fn tool(&self) -> usize {
        self.tools.selected().unwrap_or(0)
    }

    /// Whether tool `index` is ticked, whatever its parent says.
    pub(crate) fn is_checked(&self, index: usize) -> bool {
        self.checked.get(index).copied().unwrap_or(false)
    }

    /// Id of the parent or requirement keeping tool `index` from counting.
    pub(crate) fn blocker(&self, index: usize) -> Option<&'static str> {
        let tool = TOOLS.get(index)?;
        tool.parent()
            .into_iter()
            .chain(tool.needs)
            .find(|id| !placeholder::position(id).is_some_and(|at| self.is_active(at)))
    }

    /// Whether tool `index` is ticked and nothing blocks it.
    pub(crate) fn is_active(&self, index: usize) -> bool {
        self.is_checked(index) && self.blocker(index).is_none()
    }

    /// How many tools are active.
    pub(crate) fn active(&self) -> usize {
        (0..TOOLS.len()).filter(|&at| self.is_active(at)).count()
    }

    /// Files the active tools write, sorted by path.
    pub(crate) fn files(&self) -> Vec<File> {
        let mut files: Vec<File> = TOOLS
            .iter()
            .enumerate()
            .filter(|&(at, _)| self.is_active(at))
            .flat_map(|(tool, def)| def.writes.iter().map(move |&path| File { path, tool }))
            .collect();
        files.sort_unstable_by_key(|file| file.path);
        files
    }

    /// The file selected in `[3]`, if the list isn't empty.
    pub(crate) fn file(&self) -> Option<File> {
        self.files()
            .into_iter()
            .nth(self.files.selected().unwrap_or(0))
    }

    /// Body of `file`, with the repo fields filled in.
    pub(crate) fn body(&self, file: File) -> String {
        let tool = TOOLS.get(file.tool).map_or("", |tool| tool.id);
        placeholder::body(file.path)
            .replace("{owner}", &self.owner)
            .replace("{name}", &self.name)
            .replace("{desc}", &self.desc)
            .replace("{path}", file.path)
            .replace("{tool}", tool)
    }

    /// The preset the ticked tools match exactly, if any.
    pub(crate) fn preset(&self) -> Option<&'static Preset> {
        PRESETS.iter().find(|preset| {
            TOOLS
                .iter()
                .enumerate()
                .all(|(at, tool)| self.is_checked(at) == preset.includes(tool.id))
        })
    }

    /// What stops the repo from being generated, if anything.
    pub(crate) fn problem(&self) -> Option<&'static str> {
        if self.name.trim().is_empty() {
            Some("name required")
        } else if self.files().is_empty() {
            Some("nothing to write")
        } else {
            None
        }
    }

    /// Replaces the footer message.
    fn say(&mut self, level: Level, message: impl Into<String>) {
        self.level = level;
        self.message = message.into();
    }

    /// Handles a key press.
    pub(crate) fn on_key(&mut self, key: KeyEvent) {
        if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
            self.running = false;
            return;
        }
        match self.modal.take() {
            Some(modal) => self.modal = self.on_modal_key(modal, key),
            None => self.on_action(self.action(&key)),
        }
    }

    /// Resolves `key`: bindings from `config.toml` first, then fixed keys.
    fn action(&self, key: &KeyEvent) -> Action {
        let keys = &self.keys;
        let bound = [
            (&keys.quit, Action::Quit),
            (&keys.scroll_up, Action::Up),
            (&keys.scroll_down, Action::Down),
            (&keys.scroll_left, Action::Left),
            (&keys.scroll_right, Action::Right),
            (&keys.generate, Action::Generate),
            (&keys.enter, Action::Open),
            (&keys.leave, Action::Close),
        ];
        if let Some(&(_, action)) = bound.iter().find(|(pattern, _)| pattern.matches(key)) {
            return action;
        }
        match key.code {
            KeyCode::Up => Action::Up,
            KeyCode::Down => Action::Down,
            KeyCode::Left => Action::Left,
            KeyCode::Right => Action::Right,
            KeyCode::PageUp => Action::PageUp,
            KeyCode::PageDown => Action::PageDown,
            KeyCode::Tab => Action::NextPane,
            KeyCode::BackTab => Action::PrevPane,
            KeyCode::Char(' ') => Action::Toggle,
            KeyCode::Char('1') => Action::Focus(Pane::Repo),
            KeyCode::Char('2') => Action::Focus(Pane::Tools),
            KeyCode::Char('3') => Action::Focus(Pane::Files),
            KeyCode::Char('4') => Action::Focus(Pane::Preview),
            KeyCode::Char('r') => Action::Repo,
            KeyCode::Char('p') => Action::Preset,
            KeyCode::Char('?') => Action::Help,
            _ => Action::Nothing,
        }
    }

    /// Runs `action` against the panes.
    fn on_action(&mut self, action: Action) {
        match action {
            Action::Quit => self.running = false,
            Action::Up => self.step(false),
            Action::Down => self.step(true),
            Action::Left | Action::Close if self.focus == Pane::Preview => {
                self.focus_pane(match self.subject {
                    Subject::Tool => Pane::Tools,
                    Subject::File => Pane::Files,
                });
            }
            Action::Right => self.focus_pane(Pane::Preview),
            Action::PageUp => self.scroll = self.scroll.saturating_sub(PAGE),
            Action::PageDown => self.scroll = self.scroll.saturating_add(PAGE),
            Action::NextPane => self.focus_pane(self.focus.cycle(true)),
            Action::PrevPane => self.focus_pane(self.focus.cycle(false)),
            Action::Focus(pane) => self.focus_pane(pane),
            Action::Toggle if self.focus == Pane::Tools => self.toggle(),
            Action::Open => match self.focus {
                Pane::Repo => self.open_repo(0),
                Pane::Tools => self.toggle(),
                Pane::Files => self.focus_pane(Pane::Preview),
                Pane::Preview => {}
            },
            Action::Repo => self.open_repo(0),
            Action::Preset => self.open_preset(),
            Action::Generate => self.open_generate(),
            Action::Help => self.modal = Some(Modal::Help),
            Action::Left | Action::Close | Action::Toggle | Action::Nothing => {}
        }
    }

    /// Focuses `pane`; a list also becomes what the preview describes.
    fn focus_pane(&mut self, pane: Pane) {
        let subject = match pane {
            Pane::Tools => Subject::Tool,
            Pane::Files => Subject::File,
            Pane::Repo | Pane::Preview => self.subject,
        };
        if subject != self.subject {
            self.subject = subject;
            self.scroll = 0;
        }
        self.focus = pane;
    }

    /// Moves the focused list's selection, or scrolls the preview.
    fn step(&mut self, down: bool) {
        match self.focus {
            Pane::Tools => {
                step_in(&mut self.tools, TOOLS.len(), down);
                self.scroll = 0;
                self.follow();
            }
            Pane::Files => {
                let len = self.files().len();
                step_in(&mut self.files, len, down);
                self.scroll = 0;
            }
            Pane::Preview if down => self.scroll = self.scroll.saturating_add(1),
            Pane::Preview => self.scroll = self.scroll.saturating_sub(1),
            Pane::Repo => {}
        }
    }

    /// Points `[3]` at the first file of the tool selected in `[2]`.
    fn follow(&mut self) {
        let tool = self.tool();
        if let Some(at) = self.files().iter().position(|file| file.tool == tool) {
            self.files.select(Some(at));
        }
    }

    /// Keeps the `[3]` selection inside a list that just changed length.
    fn clamp_files(&mut self) {
        let last = self.files().len().saturating_sub(1);
        self.files
            .select(Some(self.files.selected().unwrap_or(0).min(last)));
    }

    /// Ticks or unticks the selected tool.
    fn toggle(&mut self) {
        let at = self.tool();
        let Some(checked) = self.checked.get_mut(at) else {
            return;
        };
        *checked = !*checked;
        let on = *checked;
        let name = TOOLS.get(at).map_or("", |tool| tool.name());
        match (on, self.blocker(at)) {
            (false, _) => self.say(Level::Info, format!("{name} off")),
            (true, None) => self.say(Level::Info, format!("{name} on")),
            (true, Some(blocker)) => self.say(
                Level::Warn,
                format!("{name} on, but inactive until {blocker} is on"),
            ),
        }
        self.clamp_files();
        self.follow();
    }

    /// Ticks exactly the tools `preset` turns on.
    fn apply(&mut self, preset: &Preset) {
        for (checked, tool) in self.checked.iter_mut().zip(&TOOLS) {
            *checked = preset.includes(tool.id);
        }
        self.clamp_files();
        self.follow();
    }

    /// Opens the repo dialog with `focus` on field 0 (owner), 1 or 2.
    fn open_repo(&mut self, focus: usize) {
        let fields = [
            Input::new(self.owner.clone()),
            Input::new(self.name.clone()),
            Input::new(self.desc.clone()),
        ];
        self.modal = Some(Modal::Repo { fields, focus });
    }

    /// Opens the preset picker on the preset in use, if any.
    fn open_preset(&mut self) {
        let current = self
            .preset()
            .and_then(|preset| PRESETS.iter().position(|other| other.name == preset.name));
        self.modal = Some(Modal::Preset(
            TableState::new().with_selected(current.unwrap_or(0)),
        ));
    }

    /// Opens the generate dialog, or says what is missing first.
    fn open_generate(&mut self) {
        if self.name.trim().is_empty() {
            self.say(Level::Warn, "name the repo first");
            self.open_repo(1);
        } else if self.files().is_empty() {
            self.say(Level::Warn, "turn on a tool that writes files first");
        } else {
            let destination = Input::new(format!("./{}", self.name));
            self.modal = Some(Modal::Generate(destination));
        }
    }

    /// Handles `key` inside `modal`; returns the modal if it stays open.
    fn on_modal_key(&mut self, modal: Modal, key: KeyEvent) -> Option<Modal> {
        if self.keys.leave.matches(&key) {
            return None;
        }
        match modal {
            Modal::Repo { fields, focus } => self.on_repo_key(fields, focus, key),
            Modal::Preset(table) => self.on_preset_key(table, &key),
            Modal::Generate(input) => self.on_generate_key(input, key),
            Modal::Help => None,
        }
    }

    /// Repo dialog: tab or arrows between fields, confirm saves all three.
    fn on_repo_key(
        &mut self,
        mut fields: [Input; 3],
        focus: usize,
        key: KeyEvent,
    ) -> Option<Modal> {
        let focus = match key.code {
            _ if self.keys.confirm.matches(&key) => {
                let [owner, name, desc] = fields;
                self.owner = String::from(owner);
                self.name = String::from(name);
                self.desc = String::from(desc);
                self.say(Level::Done, "repo updated");
                return None;
            }
            KeyCode::Tab | KeyCode::Down => (focus + 1) % fields.len(),
            KeyCode::BackTab | KeyCode::Up => (focus + fields.len() - 1) % fields.len(),
            _ => {
                if let Some(field) = fields.get_mut(focus) {
                    field.handle_event(&Event::Key(key));
                }
                focus
            }
        };
        Some(Modal::Repo { fields, focus })
    }

    /// Preset picker: move, confirm applies.
    fn on_preset_key(&mut self, mut table: TableState, key: &KeyEvent) -> Option<Modal> {
        if self.keys.confirm.matches(key) {
            let preset = PRESETS.get(table.selected().unwrap_or(0))?;
            self.apply(preset);
            self.say(
                Level::Done,
                format!("preset {} applied, {} tools", preset.name, preset.count()),
            );
            return None;
        }
        match self.action(key) {
            Action::Up => step_in(&mut table, PRESETS.len(), false),
            Action::Down => step_in(&mut table, PRESETS.len(), true),
            _ => {}
        }
        Some(Modal::Preset(table))
    }

    /// Generate dialog: edit the destination, confirm reports what would be written.
    fn on_generate_key(&mut self, mut input: Input, key: KeyEvent) -> Option<Modal> {
        if !self.keys.confirm.matches(&key) {
            input.handle_event(&Event::Key(key));
            return Some(Modal::Generate(input));
        }
        let destination = input.value().trim();
        if destination.is_empty() {
            self.say(Level::Error, "the destination can't be empty");
            return Some(Modal::Generate(input));
        }
        let message = format!(
            "would write {} files to {destination} (placeholder, nothing written)",
            self.files().len()
        );
        self.say(Level::Done, message);
        None
    }
}

/// Moves `table`'s selection one row within `len` rows, without wrapping.
fn step_in(table: &mut TableState, len: usize, down: bool) {
    let at = table.selected().unwrap_or(0);
    let next = if down { at + 1 } else { at.saturating_sub(1) };
    table.select(Some(next.min(len.saturating_sub(1))));
}
