use super::{Guid, Name, Ptr};

#[derive(Debug)]
#[repr(C)]
pub struct Progress {
    pub contentGuid:     Guid,
    pub contentType:     u32,
    pub contentUid:      u32,
    pub contentName:     Ptr<Name>,
    pub contentFullName: Ptr<Name>,
    pub dataId:          u32,
    pub r#type:          ProgressType,
    pub _30:             u32,
    _34:                 u32,
    pub _38:             *const (),
    pub _40:             u32,
    pub _44:             u32,
    _48:                 u32,
    _4c:                 u32,
    pub _50:             *const (),
    pub _58:             u32,
    _5c:                 u32,
    pub _60:             u32,
    pub _64:             u32,
    pub _68:             u32,
    pub _6c:             u32,
}

#[derive(Debug)]
#[repr(u32)]
pub enum ProgressType {
    Bit     = 0,
    Bitmask = 1,
    Counter = 2,
    Maximum = 3,
}
