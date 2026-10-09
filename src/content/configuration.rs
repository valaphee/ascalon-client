use super::{Guid, Name};

#[repr(C)]
pub struct Configuration {
    pub contentGuid:     Guid,
    pub contentType:     u32,
    pub contentUid:      u32,
    pub contentName:     *const Name,
    pub contentFullName: *const Name,
    pub r#type:          u32,
    pub _2c:             u32,
    pub _type:           u32,
    pub _34:             u32,
    pub value:           u32,
}
