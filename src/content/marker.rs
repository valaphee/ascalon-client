use super::{Guid, Name, Ptr};

#[derive(Debug)]
#[repr(C)]
pub struct Marker {
    pub contentGuid:     Guid,
    pub contentType:     u32,
    pub contentUid:      u32,
    pub contentName:     Ptr<Name>,
    pub contentFullName: Ptr<Name>,
}
