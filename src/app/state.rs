use log::info;
use crate::disp_mgr::{disp::DispProp, DispMgr};

#[derive(Clone)]
pub enum AppState {
    Uninit,
    Initialized {
        tab_index: usize,
        focused_prop: DispProp,
        num_disps: usize,
        disp_mgr: DispMgr,
    },
}

impl AppState {
    pub fn initialize(disp_mgr: DispMgr) -> Self {
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

    ///Check if app has bee initialized
    pub fn is_initialized(&self) -> bool {
        matches!(self, &Self::Initialized { .. })
    }

    pub fn move_right(&mut self) {
        info!("Waiting for io thread to move right");
    }

    pub fn move_left(&mut self) {
        info!("Waiting for io thread to move left");
    }

    ///Increment tab_index or cycle around if 1
    pub fn tab_right(&mut self) {
        if let Self::Initialized { tab_index, num_disps, .. } = self {
            *tab_index = (*tab_index + 1) % (*num_disps + 1)
        }
    }

    ///Decrement tab_index or cycle around if 0
    pub fn tab_left(&mut self) {
        if let Self::Initialized { tab_index, num_disps, .. } = self {
            if *tab_index == 0 { *tab_index = *num_disps; }
            else { *tab_index -= 1; }
        }
    }

    ///Get tab_index
    pub fn tab_index(&self) -> Option<usize> {
        if let Self::Initialized { tab_index, .. } = self {
            Some(*tab_index)
        } else {
            None
        }
    }

    ///Get the disp_mgr. ONLY USED FOR CHECKING STATE, NOT CHANGING
    pub fn disp_mgr(&self) -> Option<DispMgr> {
        if let Self::Initialized { disp_mgr, .. } = self {
            Some(disp_mgr.clone())
        } else {
            None
        }
    }

    ///Get the DispProp of the currently selected slider
    pub fn focused_prop(&self) -> Option<DispProp> {
        if let Self::Initialized { focused_prop, .. } = self {
            Some(*focused_prop)
        } else {
            None
        }
    }

    ///Get the number of connected displays
    pub fn num_disps(&self) -> Option<usize> {
        if let Self::Initialized { num_disps, .. } = self {
            Some(*num_disps)
        } else {
            None
        }
    }

    ///Set the state of the disp_mgr. Set by the io thread.
    pub fn set_disp_mgr(&mut self, new_disp_mgr: DispMgr) {
        if let Self::Initialized { ref mut disp_mgr, .. } = self {
            *disp_mgr = new_disp_mgr;
        }
    }

    ///Move the selected ui slider to the next value
    pub fn next_prop(&mut self) {
        if let Self::Initialized { focused_prop, .. } = self {
            focused_prop.next();
        }
    }

    ///Move the selected ui slider to the previous value
    pub fn prev_prop(&mut self) {
        if let Self::Initialized { focused_prop, .. } = self {
            focused_prop.prev();
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::Uninit
    }
}
