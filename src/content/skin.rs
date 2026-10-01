use super::{Name, WcharPtr};

#[repr(C)]
pub struct Skin {
    pub contentGuid: [u8; 16],
    pub contentType: u32,
    pub contentUid: u32,
    pub contentName: *const Name,
    pub contentFullName: *const Name,
    pub dataId: u32,
    _2c: u32,          // zero
    pub _30: WcharPtr, // file
    pub _38: *const [()],
    pub _48: WcharPtr, // file
    pub _50: u32,
    pub _54: u32,
    pub fileIcon: WcharPtr,
    _60: u32, // zero
    _64: u32, // zero
    pub textName: u32,
    pub textDescription: u32,
    pub _70: u32,       // text
    _74: u32,           // zero
    pub _78: *const (), // type 0x17A
    pub _80: u32,
    _84: u32, // zero
    pub _88: *const (),
    pub _90: u32,
    _94: u32, // zero
    pub _98: u32,
    _9c: u32, // zero
    pub _a0: u32,
    _a4: u32, // zero
    pub _a8: *const [()],
    pub _b8: u32,
}
