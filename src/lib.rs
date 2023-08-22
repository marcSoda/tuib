use std::io::stdout;
use std::sync::Arc;
use parking_lot::FairMutex;
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

pub fn start_ui(app: &Arc<FairMutex<App>>) -> Result<()> {
    //setup tui
    let stdout = stdout();
    crossterm::terminal::enable_raw_mode()?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    // todo: when tick rate is raised (eg 200), weird stuff happens when holding keys. test it
    let tick_rate = Duration::from_millis(10);
    let mut events = Events::new(tick_rate);

    app.lock().dispatch(IoEvent::Initialize);

    //main UI loop
    loop {
        //need to sleep or lock is acquired too fast and IO thread blocks infinitely
        let mut app = app.lock();

        //draw the app
        terminal.draw(|rect| ui::draw(rect, &app))?;

        //get either a tick(occurrs every 200ms an Input is not detected) or an Input
        let result = match events.get_next() {
            InputEvent::Input(key) => app.do_action(key),
            InputEvent::Tick => app.update_on_tick(),
        };

        //break if we receive the signal to exit
        if result == AppReturn::Exit {
            events.close();
            break;
        }
    }

    terminal.show_cursor()?;
    terminal.clear()?;
    crossterm::terminal::disable_raw_mode()?;
    Ok(())
}
