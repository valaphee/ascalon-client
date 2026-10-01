use super::{Item, Name, Progress};

#[repr(C)]
pub struct CraftingRecipe {
    pub contentGuid: [u8; 16],
    pub contentType: u32,
    pub contentUid: u32,
    pub contentName: *const Name,
    pub contentFullName: *const Name,
    pub dataId: u32,
    pub _2c: u32,
    pub ingredients: *const [()],
    pub outputItem: *const Item,
    pub outputItemCount: u32,
    pub rating: u32,
    pub _50: u32,
    pub time: u32,
    pub _58: u32,
    _5c: u32, // zero
    pub _60: *const Progress,
    pub _68: *const (), // type 0x1B
    _70: u32,           // zero
    _74: u32,           // zero
    pub guildIngredients: *const [()],
    pub item: *const Item,
    pub _90: *const (), // type 0x98
    _98: u32,           // zero
    _9c: u32,           // zero
    _a0: u32,           // zero
    _a4: u32,           // zero
    pub _a8: *const (), // type 0x1E
}
