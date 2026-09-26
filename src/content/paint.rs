use ascalon_asset::packfile::Ptr;

use super::{Guid, Name};

#[derive(Debug)]
#[repr(C)]
pub struct Paint {
    pub contentGuid: Guid,
    pub contentType: u32,
    pub contentUid: u32,
    pub contentName: Ptr<Name>,
    pub contentFullName: Ptr<Name>,
    pub dataId: u32,
    pub _2c: u32,
    pub _30: Ptr<Paint_30>,
    pub _38: u32,
    pub _3c: u32,
    pub _40: u32,
    pub _44: u32,
    pub name: u32,
    pub _4c: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct Paint_30 {
    pub brightness: f32,
    pub contrast: f32,
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
    pub _14: u32,
}
