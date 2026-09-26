use ascalon_asset::packfile::{Ptr, WcharPtr};

use super::{Guid, Name};

#[derive(Debug)]
#[repr(C)]
pub struct Item {
    pub contentGuid: Guid,
    pub contentType: u32,
    pub contentUid: u32,
    pub contentName: Ptr<Name>,
    pub contentFullName: Ptr<Name>,
    pub dataId: u32,
    pub r#type: ItemType,
    pub flags: u32,
    pub _3c: u32,
    pub icon: WcharPtr,
    pub _48: u32,
    pub _level: u32,
    pub _50: Ptr<()>,
    pub _58: Ptr<()>,
    pub rarity: ItemRarity,
    pub _64: u32,
    pub _68: Ptr<()>,
    pub _70: u32,
    pub level: u32,
    pub _78: u32,
    pub _7c: u32,
    pub name: u32,
    pub description: u32,
    pub _88: u32,
    pub _8c: u32,
    pub _90: Ptr<()>,
    pub _98: u32,
    pub _9c: u32,
    pub _a0: u32,
    pub _a4: u32,
    pub _a8: u32,
    pub _ac: u32,
}

#[derive(Debug)]
#[repr(C, u32)]
pub enum ItemType {
    Armor(Ptr<ItemArmor>) = 0,
    Augment(Ptr<ItemAugment>) = 1,
    Back(Ptr<ItemBack>) = 2,
    Bag(Ptr<ItemBag>) = 3,
    Consumable(Ptr<ItemConsumable>) = 4,
    Container(Ptr<ItemContainer>) = 5,
    CraftingMaterial(Ptr<ItemCraftingMaterial>) = 6,
    Gathering(Ptr<ItemGathering>) = 9,
    Gizmo(Ptr<ItemGizmo>) = 10,
    JadeTechModule(Ptr<ItemJadeTechModule>) = 11,
    MiniPet(Ptr<ItemMiniPet>) = 15,
    PowerCore(Ptr<ItemPowerCore>) = 17,
    Relic(Ptr<ItemRelic>) = 18,
    Tool(Ptr<ItemTool>) = 19,
    TraitGuide(Ptr<ItemTraitGuide>) = 20,
    Trinket(Ptr<ItemTrinket>) = 21,
    Trophy(Ptr<ItemTrophy>) = 22,
    UpgradeComponent(Ptr<ItemUpgradeComponent>) = 23,
    Weapon(Ptr<ItemWeapon>) = 24,
}

#[derive(Debug)]
#[repr(u32)]
pub enum ItemRarity {
    Junk = 0,
    Basic = 1,
    Fine = 2,
    Masterwork = 3,
    Rare = 4,
    Exotic = 5,
    Ascended = 6,
    Legendary = 7,
}

#[derive(Debug)]
#[repr(C)]
pub struct ItemArmor {
    pub _00: Ptr<()>,
    pub r#type: ItemArmorType,
    pub defense: u32,
    pub scaleType: u32,
    pub _14: u32,
    pub _18: u32,
    pub _1c: u32,
    pub _20: u32,
    pub _24: u32,
    pub _28: u32,
    pub _2c: u32,
    pub _30: u32,
    pub _34: u32,
    pub _38: Ptr<()>,
    pub _40: u32,
    pub _44: u32,
    pub _48: u32,
    pub _4c: u32,
    pub _50: u32,
    pub _54: u32,
    pub _58: u32,
    pub _5c: u32,
    pub _60: u32,
    pub _64: u32,
    pub weightClass: ItemArmorWeightClass,
}

#[derive(Debug)]
#[repr(u32)]
pub enum ItemArmorType {
    _0 = 0,
    _1 = 1,
    _2 = 2,
    _3 = 3,
    _4 = 4,
    _5 = 5,
    _6 = 6,
    _7 = 7,
    _8 = 8,
}

#[derive(Debug)]
#[repr(u32)]
pub enum ItemArmorWeightClass {
    _0 = 0,
    _1 = 1,
    _2 = 2,
    _3 = 3,
    _4 = 4,
}

#[derive(Debug)]
#[repr(C)]
pub struct ItemAugment;

#[derive(Debug)]
#[repr(C)]
pub struct ItemBack;

#[derive(Debug)]
#[repr(C)]
pub struct ItemBag {
    pub _00: u32,
    pub _04: u32,
    pub _08: u32,
    pub _0c: u32,
    pub _10: u32,
    pub _14: u32,
    pub _18: u32,
    pub _1c: u32,
    pub _20: u32,
    pub _24: u32,
    pub size: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct ItemConsumable {
    pub r#type: ItemConsumableType,
}

#[derive(Debug)]
#[repr(u32)]
pub enum ItemConsumableType {
    _0 = 0,
    _1 = 1,
    _2 = 2,
    _3 = 3,
    _4 = 4,
    _5 = 5,
    _6 = 6,
    _7 = 7,
    _8 = 8,
    _9 = 9,
    _10 = 10,
    _11 = 11,
    _12 = 12,
    _13 = 13,
    _14 = 14,
    _15 = 15,
    _16 = 16,
}

#[derive(Debug)]
#[repr(C)]
pub struct ItemContainer {
    pub flags: ItemContainerFlags,
    pub _04: u32,
    pub _08: u32,
    pub _0c: u32,
    pub _10: u32,
    pub _14: u32,
    pub _18: u32,
    pub _1c: u32,
    pub _20: u32,
    pub _24: u32,
    pub _28: u32,
    pub _2c: u32,
    pub _30: u32,
    pub _34: u32,
    pub _38: u32,
    pub _3c: u32,
    pub _40: u32,
    pub _44: u32,
    pub _48: u32,
    pub _4c: u32,
    pub _50: u32,
    pub r#type: ItemContainerType,
}

bitflags::bitflags! {
    #[repr(transparent)]
    #[derive(Debug)]
    pub struct ItemContainerFlags: u32 {
        const SHOW_SPLASH = 1 << 1;
    }
}

#[derive(Debug)]
#[repr(u32)]
pub enum ItemContainerType {
    Default = 0,
    _1 = 1,
    _2 = 2,
    OpenUi = 3,
}

#[derive(Debug)]
#[repr(C)]
pub struct ItemCraftingMaterial;

#[derive(Debug)]
#[repr(C)]
pub struct ItemGathering {
    pub _00: Ptr<()>,
    pub _08: u32,
    pub _0c: u32,
    pub _10: Ptr<()>,
    pub r#type: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct ItemGizmo {
    pub _00: u32,
    pub r#type: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct ItemJadeTechModule;

#[derive(Debug)]
#[repr(C)]
pub struct ItemMiniPet;

#[derive(Debug)]
#[repr(C)]
pub struct ItemPowerCore;

#[derive(Debug)]
#[repr(C)]
pub struct ItemRelic;

#[derive(Debug)]
#[repr(C)]
pub struct ItemTool {
    pub _00: u32,
    pub r#type: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct ItemTraitGuide;

#[derive(Debug)]
#[repr(C)]
pub struct ItemTrinket {
    pub _00: u32,
    pub _04: u32,
    pub _08: u32,
    pub _0c: u32,
    pub _10: u32,
    pub _14: u32,
    pub _18: u32,
    pub _1c: u32,
    pub r#type: ItemTrinketType,
}

#[derive(Debug)]
#[repr(u32)]
pub enum ItemTrinketType {
    _0 = 0,
    _1 = 1,
    _2 = 2,
    _3 = 3,
}

#[derive(Debug)]
#[repr(C)]
pub struct ItemTrophy;

#[derive(Debug)]
#[repr(C)]
pub struct ItemUpgradeComponent {
    pub _00: u32,
    pub _04: u32,
    pub _08: u32,
    pub _0c: u32,
    pub _10: Ptr<()>,
    pub _18: u32,
    pub _1c: u32,
    pub _20: u32,
    pub r#type: ItemUpgradeComponentType,
}

#[derive(Debug)]
#[repr(u32)]
pub enum ItemUpgradeComponentType {
    _0 = 0,
    _1 = 1,
    _2 = 2,
    _3 = 3,
    _4 = 4,
}

#[derive(Debug)]
#[repr(C)]
pub struct ItemWeapon {
    pub _00: Ptr<()>,
    pub _08: u32,
    pub r#type: ItemWeaponType,
    pub _10: u32,
    pub _14: u32,
    pub _18: u32,
    pub defense: u32,
    pub scaleType: u32,
}

#[derive(Debug)]
#[repr(u32)]
pub enum ItemWeaponType {
    _0 = 0,
    _1 = 1,
    _2 = 2,
    _3 = 3,
    _4 = 4,
    _5 = 5,
    _6 = 6,
    _7 = 7,
    _8 = 8,
    _9 = 9,
    _10 = 10,
    _11 = 11,
    _12 = 12,
    _13 = 13,
    _14 = 14,
    _15 = 15,
    _16 = 16,
    _17 = 17,
    _18 = 18,
    _19 = 19,
    _20 = 20,
    _21 = 21,
    _22 = 22,
    _23 = 23,
    _24 = 24,
}
