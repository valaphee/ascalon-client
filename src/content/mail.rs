use super::{Guid, Name, Ptr, String};

#[derive(Debug)]
#[repr(C)]
pub struct Mail {
    pub contentGuid:     Guid,
    pub contentType:     u32,
    pub contentUid:      u32,
    pub contentName:     Ptr<Name>,
    pub contentFullName: Ptr<Name>,
    pub dataId:          u32,
    pub coins:           u32,
    _30:                 u32,
    pub _34:             u32,
    pub _38:             u32,
    pub _3c:             u32,
    pub _40:             *const (),
    pub _48:             *const (),
    pub _50:             Guid,
    _60:                 u32,
    _64:                 u32,
    _68:                 u32,
    _6c:                 u32,
    pub textMessage:     u32,
    pub textSender:      u32,
    pub textSubject:     u32,
    pub _7c:             u32,
    pub _80:             String,
    pub _90:             String,
    pub _a0:             u32,
    _a4:                 u32,
    pub _a8:             String,
}
