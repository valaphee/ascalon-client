use super::Name;

#[repr(C)]
pub struct Progress {
    pub content_guid: [u8; 16],
    pub content_type: u32,
    pub content_uid: u32,
    pub content_name: *const Name,
    pub content_full_name: *const Name,
    pub data_id: u32,
    pub r#type: ProgressType,
    pub _30: u32,
    pub _34: u32,
    pub _38: *const (),
    pub _40: u32,
    pub _44: u32,
    pub _48: u32,
    pub _4c: u32,
    pub _50: *const (), // type 0x196
    pub _58: u32,
    pub _5c: u32,
    pub _60: u32,
    pub _64: u32,
    pub _68: u32,
    pub _6c: u32,
}

#[repr(u32)]
pub enum ProgressType {
    Bit = 0,
    Bitmask = 1,
    Counter = 2,
    Maximum = 3,
}
