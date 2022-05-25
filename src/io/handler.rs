use std::sync::{Arc, Mutex};
use eyre::Result;
use log::{error, info};
use super::IoEvent;
use crate::app::App;
use crate::disp_mgr::{DispMgr, disp::DispProp};

pub struct IoAsyncHandler {
    app: Arc<Mutex<App>>,
    disp_mgr: DispMgr,
}

impl IoAsyncHandler {
    pub fn new(app: Arc<Mutex<App>>, disp_mgr: DispMgr) -> Self {
        Self {
            app,
            disp_mgr,
        }
    }

    pub fn handle_io_event(&mut self, io_event: IoEvent) {
        let result = match io_event {
            IoEvent::Initialize => self.do_initialize(),
            IoEvent::DeviceIncrement(device_index, prop) => self.do_increment(device_index, prop),
            IoEvent::DeviceDecrement(device_index, prop) => self.do_decrement(device_index, prop),
        };

        if let Err(err) = result {
            error!("Error in io::handler::handle_io_event: {:?}", err);
        }

        let mut app = self.app.lock().unwrap();
        app.loaded();
    }

    fn do_initialize(&mut self) -> Result<()> {
        info!("Initialized");
        let mut app = self.app.lock().unwrap();
        app.initialized();
        info!("Application initialized");
        Ok(())
    }

    fn do_increment(&mut self, device_index: usize, prop: DispProp) -> Result<()> {
        info!(">>>>>>>>>>>>>>>>>>>>>>>>>");
        // if device_index == self.disp_mgr.get_num_disps() { return Ok(()); }
        self.disp_mgr.increment_value_by_index(0, prop);
        let app = self.app.lock().unwrap();
        Ok(())
    }

    fn do_decrement(&mut self, device_index: usize, prop: DispProp) -> Result<()> {
        info!("<<<<<<<<<<<<<<<<<<<");
        // if device_index == self.disp_mgr.get_num_disps() { return Ok(()); }
        self.disp_mgr.decrement_value_by_index(0, prop);
        let app = self.app.lock().unwrap();
        Ok(())
    }
}
