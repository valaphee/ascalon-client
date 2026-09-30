use ascalon_asset::packfile::{Guid, Ptr};

use super::{Item, Name};

#[derive(Debug)]
#[repr(C)]
pub struct CraftingRecipe {
    pub content_guid: Guid,
    pub content_type: u32,
    pub content_uid: u32,
    pub content_name: Ptr<Name>,
    pub content_full_name: Ptr<Name>,
    pub data_id: u32,
    pub _2c: u32,
    pub _30: Ptr<()>,
    pub _38: u32,
    pub _3c: u32,
    pub _40: Ptr<Item>,
    pub _48: u32,
    pub _4c: u32,
    pub _50: u32,
    pub _54: u32,
    pub _58: u32,
    pub _5c: u32,
    pub _60: Ptr<()>,
    pub _68: Ptr<()>,
    pub _70: u32,
    pub _74: u32,
    pub _78: Ptr<()>,
    pub _80: u32,
    pub _84: u32,
    pub _88: Ptr<Item>,
    pub _90: Ptr<()>,
    pub _98: u32,
    pub _9c: u32,
    pub _a0: u32,
    pub _a4: u32,
    pub _a8: Ptr<()>,
}
