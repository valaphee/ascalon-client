use super::{Item, Name, Progress};

#[repr(C)]
pub struct CraftingRecipe {
    pub content_guid: [u8; 16],
    pub content_type: u32,
    pub content_uid: u32,
    pub content_name: *const Name,
    pub content_full_name: *const Name,
    pub data_id: u32,
    pub _2c: u32,
    pub _30: *const (),
    pub _38: u32,
    pub _3c: u32,
    pub _40: *const Item,
    pub _48: u32,
    pub _4c: u32,
    pub _50: u32,
    pub _54: u32,
    pub _58: u32,
    pub _5c: u32,
    pub _60: *const Progress,
    pub _68: *const (), // type 0x1B
    pub _70: u32,
    pub _74: u32,
    pub _78: *const (),
    pub _80: u32,
    pub _84: u32,
    pub _88: *const Item,
    pub _90: *const (), // type 0x98
    pub _98: u32,
    pub _9c: u32,
    pub _a0: u32,
    pub _a4: u32,
    pub _a8: *const (), // type 0x1E
}
