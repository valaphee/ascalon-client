use super::{Content, ContentType, Guid, Name, Ptr};

#[derive(Debug)]
#[repr(C)]
pub struct Configuration {
    pub contentGuid:     Guid,
    pub contentType:     u32,
    pub contentUid:      u32,
    pub contentName:     Ptr<Name>,
    pub contentFullName: Ptr<Name>,
    pub r#type:          u32,
    pub value:           Content,
}

impl ContentType for Configuration {
    const ID: u32 = 150;
}
