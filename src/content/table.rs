use super::{ContentType, Guid, Name, Ptr};

#[derive(Debug)]
#[repr(C)]
pub struct TableInt {
    pub contentGuid:     Guid,
    pub contentType:     u32,
    pub contentUid:      u32,
    pub contentName:     Ptr<Name>,
    pub contentFullName: Ptr<Name>,
    pub entries:         Ptr<[TableIntEntry]>,
}

impl ContentType for TableInt {
    const ID: u32 = 394;
}

#[derive(Debug)]
#[repr(C)]
pub struct TableIntEntry {
    value: *const (),
    key:   u32,
}
