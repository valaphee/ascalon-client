use super::{Guid, Name, Ptr};

#[derive(Debug)]
#[repr(C)]
pub struct Sector {
    pub contentGuid:     Guid,
    pub contentType:     u32,
    pub contentUid:      u32,
    pub contentName:     Ptr<Name>,
    pub contentFullName: Ptr<Name>,
}
