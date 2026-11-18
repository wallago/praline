use std::io::Write;

use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event, KeyEventKind},
};

use crate::{app::App, config::Config, prelude::*};

/// Application state handler.
mod state;
/// Colour palette shared by every widget.
mod theme;
/// Widget renderer.
mod view;

/// UI types the rest of the module uses often.
mod prelude {
    pub(super) use super::state::*;
    pub(super) use super::theme::*;
}

/// Runs the UI until the user quits, restoring the terminal either way.
pub(crate) fn run(config: Config, app: App) -> Result<()> {
    let mut terminal = ratatui::try_init()?;
    let mut state = state::State::new(config, app)?;
    let result = event_loop(&mut terminal, &mut state);
    ratatui::restore();
    result?;
    if let Some(target) = &state.created {
        // The screen is gone with the TUI: say where the repo went.
        writeln!(std::io::stdout(), "created `{}`", target.display())?;
    }
    Ok(())
}

/// Draws, waits for a key press, applies it, until the state says stop.
fn event_loop(terminal: &mut DefaultTerminal, state: &mut state::State) -> Result<()> {
    while state.running {
        terminal.draw(|frame| view::render(state, frame))?;
        if let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            state.on_key(key);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use ratatui::{
        Terminal,
        backend::TestBackend,
        crossterm::event::{KeyCode, KeyEvent},
    };

    use super::*;

    /// A TUI started with no flags, as `praline` alone does.
    fn state() -> state::State {
        let app = App::new(Args::default()).unwrap();
        state::State::new(Config::default(), app).unwrap()
    }

    /// The screen `state` draws on a 150×40 terminal, as text.
    fn screen(state: &mut state::State) -> String {
        let mut terminal = Terminal::new(TestBackend::new(150, 40)).unwrap();
        terminal.draw(|frame| view::render(state, frame)).unwrap();
        let buffer = terminal.backend().buffer();
        (0..buffer.area.height)
            .map(|y| {
                (0..buffer.area.width)
                    .map(|x| buffer[(x, y)].symbol())
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Presses each key of `keys` in turn.
    fn press(state: &mut state::State, keys: &[KeyCode]) {
        for &key in keys {
            state.on_key(KeyEvent::from(key));
        }
    }

    #[test]
    fn the_build_page_shows_apps_options_and_the_repo() {
        let screen = screen(&mut state());
        for text in [
            "apps",
            "+ add app",
            "options · app",
            "summary",
            "about",
            "owner",
        ] {
            assert!(screen.contains(text), "no `{text}` on\n{screen}");
        }
    }

    #[test]
    fn adding_an_app_from_the_keyboard_moves_apps_into_their_folders() {
        let mut state = state();
        press(&mut state, &[KeyCode::Char('a')]);
        press(
            &mut state,
            &"site".chars().map(KeyCode::Char).collect::<Vec<_>>(),
        );
        press(&mut state, &[KeyCode::Enter]);
        assert_eq!(state.app.apps.len(), 2);
        let screen = screen(&mut state);
        assert!(screen.contains("apps/site"), "{screen}");
        assert!(screen.contains("2 apps in apps/"), "{screen}");
    }

    #[test]
    fn generate_asks_yes_or_no_then_writes_the_repo_and_quits() {
        let dest = std::env::temp_dir().join(format!("praline-ui-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dest);
        std::fs::create_dir_all(&dest).unwrap();
        let mut state = state();
        state.app.dest.clone_from(&dest);
        press(&mut state, &[KeyCode::Char('g')]);
        assert!(
            state.confirm && state.running,
            "nothing is written before the dialog"
        );
        let screen = screen(&mut state);
        assert!(
            screen.contains("Generate demo-name?") && screen.contains("y yes"),
            "{screen}"
        );
        press(&mut state, &[KeyCode::Esc]);
        assert!(!state.confirm && state.running);
        // No selection to move: Enter does nothing, `n` closes, `y` writes
        // the repo and quits.
        press(&mut state, &[KeyCode::Char('g'), KeyCode::Enter]);
        assert!(state.confirm && state.running);
        press(&mut state, &[KeyCode::Char('n')]);
        assert!(!state.confirm && state.running);
        press(&mut state, &[KeyCode::Char('g'), KeyCode::Char('y')]);
        assert!(!state.running);
        assert!(dest.join("demo-name").join("justfile").exists());
        let _ = std::fs::remove_dir_all(&dest);
    }

    #[test]
    fn the_dir_picker_shows_its_own_keys() {
        let mut state = state();
        // Tab to the repo form, down to `dest`, open the picker.
        press(
            &mut state,
            &[
                KeyCode::Tab,
                KeyCode::Down,
                KeyCode::Down,
                KeyCode::Down,
                KeyCode::Enter,
            ],
        );
        assert!(state.dashboard.picker.is_some());
        let screen = screen(&mut state);
        for hint in ["pick this dir", "parent", "open dir", "cancel"] {
            assert!(screen.contains(hint), "no `{hint}` on\n{screen}");
        }
        assert!(!screen.contains("add app"), "{screen}");
    }
}
