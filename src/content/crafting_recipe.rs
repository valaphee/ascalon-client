use super::{ContentType, Guid, Item, Name, Progress, Ptr};

#[derive(Debug)]
#[repr(C)]
pub struct CraftingRecipe {
    pub contentGuid:      Guid,
    pub contentType:      u32,
    pub contentUid:       u32,
    pub contentName:      Ptr<Name>,
    pub contentFullName:  Ptr<Name>,
    pub dataId:           u32,
    pub _2c:              u32,
    pub ingredients:      *const [()],
    pub outputItem:       *const Item,
    pub outputItemCount:  u32,
    pub rating:           u32,
    pub _50:              u32,
    pub time:             u32,
    pub _58:              u32,
    _5c:                  u32,
    pub _60:              *const Progress,
    pub _68:              *const (),
    _70:                  u32,
    _74:                  u32,
    pub guildIngredients: *const [()],
    pub item:             *const Item,
    pub _90:              *const (),
    _98:                  u32,
    _9c:                  u32,
    _a0:                  u32,
    _a4:                  u32,
    pub _a8:              *const (),
}

impl ContentType for CraftingRecipe {
    const ID: u32 = 12;
}
