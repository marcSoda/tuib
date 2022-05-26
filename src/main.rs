use std::sync::{Arc, Mutex};
use eyre::Result;
use log::LevelFilter;
use tuib::app::App;
use tuib::io::handler::IoAsyncHandler;
use tuib::io::IoEvent;
use tuib::start_ui;

fn main() -> Result<()> {
    let (sync_io_tx, sync_io_rx) = std::sync::mpsc::channel::<IoEvent>();

    let app = Arc::new(Mutex::new(App::new(sync_io_tx))); //for io thread
    let app_ui = Arc::clone(&app);                                           //for ui(main) thread

    tui_logger::init_logger(LevelFilter::Debug).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Debug);

    // IO thread
    std::thread::spawn(move || {
        let mut handler = IoAsyncHandler::new(app);
        while let Ok(io_event) = sync_io_rx.recv() {
            handler.handle_io_event(io_event);
        }
    });

    //ui(main) thread
    start_ui(&app_ui)?;

    Ok(())
}
