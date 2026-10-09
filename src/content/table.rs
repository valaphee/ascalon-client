use super::{Guid, Name, Ptr};

#[derive(Debug)]
#[repr(C)]
pub struct Table {
    pub contentGuid:     Guid,
    pub contentType:     u32,
    pub contentUid:      u32,
    pub contentName:     Ptr<Name>,
    pub contentFullName: Ptr<Name>,
    pub entries:         Ptr<[TableEntry]>,
}

#[derive(Debug)]
#[repr(C)]
pub struct TableEntry {
    value: *const (),
    key:   u32,
}
