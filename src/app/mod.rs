use log::{debug, error, warn};
use self::{actions::Actions, state::AppState};
use crate::app::actions::Action;
use crate::inputs::key::Key;
use crate::io::IoEvent;
use crate::disp_mgr::DispMgr;

pub mod actions;
pub mod state;
pub mod ui;

#[derive(Debug, PartialEq, Eq)]
pub enum AppReturn {
    Exit,
    Continue,
}

pub struct App {
    io_tx: std::sync::mpsc::Sender<IoEvent>,
    actions: Actions,
    is_loading: bool,
    pub state: AppState,
}

impl App {
    pub fn new(io_tx: std::sync::mpsc::Sender<IoEvent>) -> Self {
        let actions = vec![Action::Quit].into();
        let is_loading = false;
        let state = AppState::default();

        Self {
            io_tx,
            actions,
            is_loading,
            state,
        }
    }

    pub fn do_action(&mut self, key: Key) -> AppReturn {
        if let Some(action) = self.actions.find(key) {
            debug!("Run action [{:?}]", action);
            match action {
                Action::Quit => AppReturn::Exit,
                Action::MoveUp => {
                    self.state.prev_prop();
                    AppReturn::Continue
                }
                Action::MoveDown => {
                    self.state.next_prop();
                    AppReturn::Continue
                }
                Action::TabRight => {
                    self.state.tab_right();
                    AppReturn::Continue
                }
                Action::TabLeft => {
                    self.state.tab_left();
                    AppReturn::Continue
                }
                Action::MoveRight => {
                    self.state.move_right();
                    if let Some(device_index) = self.state.tab_index() {
                        if let Some(focused_prop) = self.state.focused_prop() {
                            self.dispatch(IoEvent::DeviceIncrement(device_index, focused_prop));
                        }
                    }
                    AppReturn::Continue
                }
                Action::MoveLeft => {
                    self.state.move_left();
                    if let Some(device_index) = self.state.tab_index() {
                        if let Some(focused_prop) = self.state.focused_prop() {
                            self.dispatch(IoEvent::DeviceDecrement(device_index, focused_prop));
                        }
                    }
                    AppReturn::Continue
                }
            }
        } else {
            warn!("No action accociated to {}", key);
            AppReturn::Continue
        }
    }

    // This doesn't do anything, included for good measure
    pub fn update_on_tick(&mut self) -> AppReturn {
        AppReturn::Continue
    }

    pub fn dispatch(&mut self, action: IoEvent) {
        self.is_loading = true;
        if let Err(e) = self.io_tx.send(action) {
            self.is_loading = false;
            error!("Error from dispatch {}", e);
        };
    }

    pub fn actions(&self) -> &Actions {
        &self.actions
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }

    pub fn is_loading(&self) -> bool {
        self.is_loading
    }

    pub fn initialize(&mut self, disp_mgr: DispMgr) {
        self.actions = vec![
            Action::Quit,
            Action::MoveRight,
            Action::MoveLeft,
            Action::MoveUp,
            Action::MoveDown,
            Action::TabRight,
            Action::TabLeft,
        ]
        .into();
        self.state = AppState::initialize(disp_mgr);
    }

    pub fn loaded(&mut self) {
        self.is_loading = false;
    }
}
