use std::sync::{Arc, Mutex};
use eyre::Result;
use log::{error, info};
use super::IoEvent;
use crate::app::App;
use crate::disp_mgr::{disp::DispProp, DispMgr};

pub struct IoHandler {
    app: Arc<Mutex<App>>,
    disp_mgr: DispMgr,
}

impl IoHandler {
    pub fn new(app: Arc<Mutex<App>>) -> Self {
        let disp_mgr = DispMgr::new();
        Self {
            app,
            disp_mgr,
        }
    }

    ///Call different function depending on IoEvent
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

    ///Initialize the application
    fn do_initialize(&mut self) -> Result<()> {
        info!("Initialized");
        let mut app = self.app.lock().unwrap();
        app.initialize(self.disp_mgr.clone());
        info!("Application initialized");
        Ok(())
    }

    ///Increment a single DispProp for a single device, and reflect changes in the UI
    fn do_increment(&mut self, device_index: usize, prop: DispProp) -> Result<()> {
        if device_index == self.disp_mgr.get_num_disps() { return Ok(()); }
        self.disp_mgr.increment_value_by_index(device_index, prop);
        self.app.lock().unwrap().state.set_disp_mgr(self.disp_mgr.clone());
        Ok(())
    }

    ///Decrement a single DispProp for a single device, and reflect changes in the UI
    fn do_decrement(&mut self, device_index: usize, prop: DispProp) -> Result<()> {
        if device_index == self.disp_mgr.get_num_disps() { return Ok(()); }
        self.disp_mgr.decrement_value_by_index(device_index, prop);
        self.app.lock().unwrap().state.set_disp_mgr(self.disp_mgr.clone());
        Ok(())
    }
}
