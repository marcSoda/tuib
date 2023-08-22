use std::sync::Arc;
use parking_lot::FairMutex;
use eyre::Result;
use log::LevelFilter;
use tuib::app::App;
use tuib::io::handler::IoHandler;
use tuib::io::IoEvent;
use tuib::start_ui;

fn main() -> Result<()> {
    //channel for passing messages from UI to IO thread
    let (sync_io_tx, sync_io_rx) = std::sync::mpsc::channel::<IoEvent>();

    //create and clone uninitialzed app
    let app = Arc::new(FairMutex::new(App::new(sync_io_tx))); //for io thread
    let app_ui = Arc::clone(&app);                        //for ui(main) thread

    //init tui_logger. may remove later
    tui_logger::init_logger(LevelFilter::Debug).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Debug);

    // IO thread. just listen for instructions from UI thread
    std::thread::spawn(move || {
        let mut handler = IoHandler::new(app);
        while let Ok(io_event) = sync_io_rx.recv() {
            handler.handle_io_event(io_event);
        }
    });

    //ui(main) thread
    start_ui(&app_ui)?;

    Ok(())
}
