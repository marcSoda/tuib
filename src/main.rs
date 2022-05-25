use std::sync::Arc;
use eyre::Result;
use log::LevelFilter;
use tuib::app::App;
use tuib::io::handler::IoAsyncHandler;
use tuib::io::IoEvent;
use tuib::disp_mgr::DispMgr;
use tuib::start_ui;

#[tokio::main]
async fn main() -> Result<()> {
    let (sync_io_tx, mut sync_io_rx) = tokio::sync::mpsc::channel::<IoEvent>(100);

    let app = Arc::new(tokio::sync::Mutex::new(App::new(sync_io_tx.clone()))); //for io thread
    let app_ui = Arc::clone(&app);                                             //for ui(main) thread

    tui_logger::init_logger(LevelFilter::Debug).unwrap();
    tui_logger::set_default_level(log::LevelFilter::Debug);

    // IO thread
    tokio::spawn(async move {
        let mut handler = IoAsyncHandler::new(app, DispMgr::new());
        while let Some(io_event) = sync_io_rx.recv().await {
            handler.handle_io_event(io_event).await;
        }
    });

    //ui(main) thread
    start_ui(&app_ui).await?;

    Ok(())
}
