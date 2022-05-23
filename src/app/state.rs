use log::info;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use crate::disp_mgr::{DispMgr, disp::DispProp};

#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized {
        disp_mgr: Arc<Mutex<DispMgr>>,
        duration: Duration,
        counter_sleep: u32,
        counter_tick: u64,
        tab_index: usize,
        focused_prop: DispProp,
        num_disps: usize,
    },
}

impl AppState {
    pub fn initialized(disp_mgr: Arc<Mutex<DispMgr>>) -> Self {
        let duration = Duration::from_secs(1);
        let counter_sleep = 0;
        let counter_tick = 0;
        let tab_index = 0;
        let focused_prop = DispProp::Brightness;
        let num_disps = disp_mgr.lock().unwrap().get_num_disps();
        Self::Initialized {
            disp_mgr,
            duration,
            counter_sleep,
            counter_tick,
            tab_index,
            focused_prop,
            num_disps,
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    //todo: create a config file where you can specify the increment magnitude instead of it always being 1
    pub fn move_right(&mut self) {
        info!("Waiting for io thread to move right");
        // if let Self::Initialized { disp_mgr, focused_prop, tab_index, num_disps,  .. } = self {
        //     if tab_index == num_disps { return; }
        //     let mut dm = disp_mgr.lock().unwrap();
        //     match *focused_prop {
        //         DispProp::Brightness => dm.disps[*tab_index].brightness = (dm.disps[*tab_index].brightness + 1).clamp(1, 100),
        //         DispProp::R => dm.disps[*tab_index].gamma.r = (dm.disps[*tab_index].gamma.r + 1).clamp(1, 100),
        //         DispProp::G => dm.disps[*tab_index].gamma.g = (dm.disps[*tab_index].gamma.g + 1).clamp(1, 100),
        //         DispProp::B => dm.disps[*tab_index].gamma.b = (dm.disps[*tab_index].gamma.b + 1).clamp(1, 100),
        //     }
        // }
    }

    pub fn move_left(&mut self) {
        info!("Waiting for io thread to move left");
        // if let Self::Initialized { disp_mgr, focused_prop, tab_index, num_disps,  .. } = self {
        //     if tab_index == num_disps { return; }
        //     let mut dm = disp_mgr.lock().unwrap();
        //     match *focused_prop {
        //         DispProp::Brightness => dm.disps[*tab_index].brightness = (dm.disps[*tab_index].brightness - 1).clamp(1, 100),
        //         DispProp::R => dm.disps[*tab_index].gamma.r = (dm.disps[*tab_index].gamma.r - 1).clamp(1, 100),
        //         DispProp::G => dm.disps[*tab_index].gamma.g = (dm.disps[*tab_index].gamma.g - 1).clamp(1, 100),
        //         DispProp::B => dm.disps[*tab_index].gamma.b = (dm.disps[*tab_index].gamma.b - 1).clamp(1, 100),
        //     }
        // }
    }

    pub fn tab_right(&mut self) {
        if let Self::Initialized { tab_index, num_disps, .. } = self {
            *tab_index = (*tab_index + 1) % (*num_disps + 1)
        }
    }

    pub fn tab_left(&mut self) {
        if let Self::Initialized { tab_index, num_disps, .. } = self {
            if *tab_index == 0 { *tab_index = *num_disps; }
            else { *tab_index -= 1; }
        }
    }

    pub fn duration(&self) -> Option<&Duration> {
        if let Self::Initialized { duration, .. } = self {
            Some(duration)
        } else {
            None
        }
    }

    pub fn tab_index(&self) -> Option<usize> {
        if let Self::Initialized { tab_index, .. } = self {
            Some(*tab_index)
        } else {
            None
        }
    }

    pub fn focused_prop(&self) -> Option<DispProp> {
        if let Self::Initialized { focused_prop, .. } = self {
            Some(*focused_prop)
        } else {
            None
        }
    }

    pub fn num_disps(&self) -> Option<&usize> {
        if let Self::Initialized { num_disps, .. } = self {
            Some(num_disps)
        } else {
            None
        }
    }

    pub fn next_prop(&mut self) {
        if let Self::Initialized { focused_prop, .. } = self {
            focused_prop.next();
        }
    }

    pub fn prev_prop(&mut self) {
        if let Self::Initialized { focused_prop, .. } = self {
            focused_prop.prev();
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Init
    }
}
