use super::{ContentType, Guid, Name, Progress, Ptr};

#[derive(Debug)]
#[repr(C)]
pub struct Emote {
    pub contentGuid:     Guid,
    pub contentType:     u32,
    pub contentUid:      u32,
    pub contentName:     Ptr<Name>,
    pub contentFullName: Ptr<Name>,
    pub dataId:          u32,
    pub _2c:             u32,
    pub _30:             u32,
    pub _34:             u32,
    pub _38:             u32,
    pub _3c:             u32,
    pub _40:             *const (),
    pub _48:             u32,
    pub _4c:             u32,
    pub _50:             u32,
    pub _54:             u32,
    pub _58:             u32,
    pub _5c:             u32,
    pub _60:             *const Progress,
    pub _68:             u32,
    pub _6c:             u32,
}

impl ContentType for Emote {
    const ID: u32 = 19;
}
