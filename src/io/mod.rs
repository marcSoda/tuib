use crate::disp_mgr::disp::DispProp;

pub mod handler;
#[derive(Debug, Clone)]
pub enum IoEvent {
    Initialize,
    DeviceIncrement(usize, DispProp),
    DeviceDecrement(usize, DispProp),
    DeviceScale(usize, DispProp, u8),
}
