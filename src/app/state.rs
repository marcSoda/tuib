use log::info;
use crate::disp_mgr::{disp::DispProp, DispMgr};

#[derive(Clone)]
pub enum AppState {
    Init,
    Initialized {
        tab_index: usize,
        focused_prop: DispProp,
        num_disps: usize,
        disp_mgr: DispMgr,
    },
}

impl AppState {
    pub fn initialized(disp_mgr: DispMgr) -> Self {
        let tab_index = 0;
        let focused_prop = DispProp::Brightness;
        let num_disps = disp_mgr.get_num_disps();
        Self::Initialized {
            tab_index,
            focused_prop,
            num_disps,
            disp_mgr,
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    //todo: create a config file where you can specify the increment magnitude instead of it always being 1
    pub fn move_right(&mut self) {
        info!("Waiting for io thread to move right");
    }

    pub fn move_left(&mut self) {
        info!("Waiting for io thread to move left");
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

    pub fn tab_index(&self) -> Option<usize> {
        if let Self::Initialized { tab_index, .. } = self {
            Some(*tab_index)
        } else {
            None
        }
    }

    pub fn disp_mgr(&self) -> Option<DispMgr> {
        if let Self::Initialized { disp_mgr, .. } = self {
            Some(disp_mgr.clone())
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

    pub fn num_disps(&self) -> Option<usize> {
        if let Self::Initialized { num_disps, .. } = self {
            Some(*num_disps)
        } else {
            None
        }
    }

    pub fn set_disp_mgr(&mut self, new_disp_mgr: DispMgr) {
        if let Self::Initialized { ref mut disp_mgr, .. } = self {
            *disp_mgr = new_disp_mgr;
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
