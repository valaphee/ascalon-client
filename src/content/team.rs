use super::{Color, ContentType, Guid, Name, Ptr};

#[derive(Debug)]
#[repr(C)]
pub struct Team {
    pub contentGuid:     Guid,
    pub contentType:     u32,
    pub contentUid:      u32,
    pub contentName:     Ptr<Name>,
    pub contentFullName: Ptr<Name>,
    pub textName:        u32,
    _2c:                 u32,
    pub _30:             *const (),
    pub _38:             *const Color,
    pub _40:             *const Color,
    pub _48:             *const Color,
    pub _50:             *const Color,
    pub _58:             *const Color,
    pub _60:             u32,
    pub _64:             u32,
}

impl ContentType for Team {
    const ID: u32 = 401;
}
