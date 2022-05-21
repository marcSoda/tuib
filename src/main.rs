mod disp_mgr;
use disp_mgr::{ DispMgr, disp::DispProp };
use std::{thread, time::Duration};


fn main() {
    let mut disp_mgr = DispMgr::new();
    disp_mgr.set_value("eDP-1".to_string(), DispProp::R, 100);
    disp_mgr.set_value("eDP-1".to_string(), DispProp::G, 100);
    disp_mgr.set_value("eDP-1".to_string(), DispProp::B, 100);
    disp_mgr.set_value("eDP-1".to_string(), DispProp::Brightness, 100);
}
