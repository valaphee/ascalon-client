use super::{Guid, Name, WcharPtr};

#[repr(C)]
pub struct Currency {
    pub contentGuid:     Guid,
    pub contentType:     u32,
    pub contentUid:      u32,
    pub contentName:     *const Name,
    pub contentFullName: *const Name,
    pub dataId:          u32,
    pub _2c:             u32,
    pub _30:             u32,
    pub _34:             u32,
    pub _38:             u32,
    pub _3c:             u32,
    pub textName:        u32,
    pub textDescription: u32,
    pub fileIcon:        WcharPtr,
    pub _50:             u32,
    pub order:           u32,
    _58:                 u32,
    _5c:                 u32,
    pub _60:             *const (),
    pub _68:             *const (),
    pub _70:             *const (),
    pub _78:             *const (),
}
