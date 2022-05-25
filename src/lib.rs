use std::io::stdout;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use app::{App, AppReturn};
use eyre::Result;
use inputs::events::Events;
use inputs::InputEvent;
use io::IoEvent;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use crate::app::ui;
pub mod app;
pub mod inputs;
pub mod io;
pub mod disp_mgr;

pub fn start_ui(app: &Arc<Mutex<App>>) -> Result<()> {
    let stdout = stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let tick_rate = Duration::from_millis(200);
    let mut events = Events::new(tick_rate);

    {
        let mut app = app.lock().unwrap();
        app.dispatch(IoEvent::Initialize);
    }

    loop {
        let mut app = app.lock().unwrap();

        terminal.draw(|rect| ui::draw(rect, &app))?;

        let result = match events.next() {
            InputEvent::Input(key) => app.do_action(key),
            InputEvent::Tick => app.update_on_tick(),
        };

        if result == AppReturn::Exit {
            events.close();
            break;
        }
    }

    terminal.clear()?;
    terminal.show_cursor()?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
