use super::{Name, Progress, WcharPtr};

#[repr(C)]
pub struct Item {
    pub contentGuid:     [u8; 16],
    pub contentType:     u32,
    pub contentUid:      u32,
    pub contentName:     *const Name,
    pub contentFullName: *const Name,
    pub dataId:          u32,
    pub r#type:          ItemType,
    pub item:            _Item,
    pub flags:           ItemFlags,
    _3c:                 u32,
    pub fileIcon:        WcharPtr,
    pub _48:             u32,
    pub _level:          u32,
    pub _50:             *const Progress,
    pub _58:             *const Progress,
    pub rarity:          ItemRarity,
    _64:                 u32,
    pub _68:             *const (),
    pub _70:             u32,
    pub level:           u32,
    _78:                 u32,
    _7c:                 u32,
    pub textName:        u32,
    pub textDescription: u32,
    pub _88:             u32,
    _8c:                 u32,
    pub _90:             *const [()],
    pub _a0:             u32,
}

#[repr(u32)]
pub enum ItemType {
    Armor            = 0,
    Augment          = 1,
    Back             = 2,
    Bag              = 3,
    Consumable       = 4,
    Container        = 5,
    CraftingMaterial = 6,
    Gathering        = 9,
    Gizmo            = 10,
    JadeTechModule   = 11,
    Key              = 12,
    MiniPet          = 15,
    PowerCore        = 17,
    Relic            = 18,
    Tool             = 19,
    TraitGuide       = 20,
    Trinket          = 21,
    Trophy           = 22,
    UpgradeComponent = 23,
    Weapon           = 24,
}

pub union _Item {
    armor:             *const ItemArmor,
    augment:           *const ItemAugment,
    back:              *const ItemBack,
    bag:               *const ItemBag,
    consumable:        *const ItemConsumable,
    container:         *const ItemContainer,
    crafting_material: *const ItemCraftingMaterial,
    gathering:         *const ItemGathering,
    gizmo:             *const ItemGizmo,
    jade_tech_module:  *const ItemJadeTechModule,
    mini_pet:          *const ItemMiniPet,
    power_core:        *const ItemPowerCore,
    relic:             *const ItemRelic,
    tool:              *const ItemTool,
    trait_guide:       *const ItemTraitGuide,
    trinket:           *const ItemTrinket,
    trophy:            *const ItemTrophy,
    upgrade_component: *const ItemUpgradeComponent,
}

#[repr(C)]
pub struct ItemArmor {
    pub _00:         *const (),
    pub r#type:      u32,
    pub _0c:         u32,
    pub _10:         u32,
    pub _14:         u32,
    pub _18:         u32,
    pub _1c:         u32,
    pub _20:         u32,
    pub _24:         u32,
    pub _28:         u32,
    pub _2c:         u32,
    pub _30:         u32,
    pub _34:         u32,
    pub _38:         *const (),
    pub _40:         u32,
    pub _44:         u32,
    pub _48:         u32,
    pub _4c:         u32,
    pub _50:         u32,
    pub _54:         u32,
    pub _58:         u32,
    pub _5c:         u32,
    pub _60:         u32,
    pub _64:         u32,
    pub weightClass: ItemArmorWeightClass,
}

#[repr(u32)]
pub enum ItemArmorType {
    Coat        = 0,
    Leggings    = 1,
    Gloves      = 2,
    Helm        = 3,
    HelmAquatic = 4,
    Boots       = 5,
    Shoulders   = 6,
}

#[repr(u32)]
pub enum ItemArmorWeightClass {
    Clothing = 0,
    Light    = 1,
    Medium   = 2,
    Heavy    = 3,
}

#[repr(C)]
pub struct ItemAugment;

#[repr(C)]
pub struct ItemBack;

#[repr(C)]
pub struct ItemBag {
    pub _00:  u32,
    pub _04:  u32,
    pub _08:  u32,
    pub _0c:  u32,
    pub _10:  u32,
    pub _14:  u32,
    pub _18:  u32,
    pub _1c:  u32,
    pub _20:  u32,
    pub _24:  u32,
    pub size: u32,
}

#[repr(C)]
pub struct ItemConsumable {
    pub r#type: ItemConsumableType,
}

#[repr(u32)]
pub enum ItemConsumableType {
    AppearanceChange  = 0,
    Booze             = 1,
    ContractNpc       = 2,
    Food              = 3,
    Generic           = 4,
    Halloween         = 5,
    Immediate         = 6,
    TeleportToFriend  = 8,
    Transmutation     = 9,
    Unlock            = 10,
    RandomUnlock      = 11,
    UpgradeRemoval    = 13,
    Utility           = 14,
    MountRandomUnlock = 15,
    Currency          = 16,
}

#[repr(C)]
pub struct ItemContainer {
    pub flags:  ItemContainerFlags,
    pub _04:    u32,
    pub _08:    u32,
    pub _0c:    u32,
    pub _10:    u32,
    pub _14:    u32,
    pub _18:    u32,
    pub _1c:    u32,
    pub _20:    u32,
    pub _24:    u32,
    pub _28:    u32,
    pub _2c:    u32,
    pub _30:    u32,
    pub _34:    u32,
    pub _38:    u32,
    pub _3c:    u32,
    pub _40:    u32,
    pub _44:    u32,
    pub _48:    u32,
    pub _4c:    u32,
    pub _50:    u32,
    pub r#type: ItemContainerType,
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct ItemContainerFlags: u32 {
        const SHOW_SPLASH = 1 << 1;
    }
}

#[repr(u32)]
pub enum ItemContainerType {
    Default   = 0,
    GiftBox   = 1,
    Immediate = 2,
    OpenUi    = 3,
}

#[repr(C)]
pub struct ItemCraftingMaterial;

#[repr(C)]
pub struct ItemGathering {
    pub _00:    *const (),
    pub uses:   u32,
    pub _0c:    u32,
    pub _10:    *const (),
    pub r#type: ItemGatheringType,
}

#[repr(u32)]
pub enum ItemGatheringType {
    Foraging = 0,
    Logging  = 1,
    Mining   = 2,
    Fishing  = 3,
    Bait     = 4,
    Lure     = 5,
}

#[repr(C)]
pub struct ItemGizmo {
    pub _00:    u32,
    pub r#type: ItemGizmoType,
}

#[repr(u32)]
pub enum ItemGizmoType {
    Default             = 0,
    ContainerKey        = 1,
    RentableContractNpc = 2,
    UnlimitedConsumable = 4,
}

#[repr(C)]
pub struct ItemJadeTechModule;

#[repr(C)]
pub struct ItemMiniPet;

#[repr(C)]
pub struct ItemPowerCore;

#[repr(C)]
pub struct ItemRelic;

#[repr(C)]
pub struct ItemTool {
    pub uses:   u32,
    pub r#type: ItemToolType,
}

#[repr(u32)]
pub enum ItemToolType {
    Salvage = 2,
}

#[repr(C)]
pub struct ItemTraitGuide;

#[repr(C)]
pub struct ItemTrinket {
    pub _00:    u32,
    pub _04:    u32,
    pub _08:    u32,
    pub _0c:    u32,
    pub _10:    u32,
    pub _14:    u32,
    pub _18:    u32,
    pub _1c:    u32,
    pub r#type: ItemTrinketType,
}

#[repr(u32)]
pub enum ItemTrinketType {
    Accessory = 0,
    Amulet    = 1,
    Ring      = 2,
}

#[repr(C)]
pub struct ItemTrophy;

#[repr(C)]
pub struct ItemUpgradeComponent {
    pub _00:    u32,
    pub _04:    u32,
    pub _08:    u32,
    pub _0c:    u32,
    pub _10:    *const (),
    pub _18:    u32,
    pub _1c:    u32,
    pub _20:    u32,
    pub r#type: ItemUpgradeComponentType,
}

#[repr(u32)]
pub enum ItemUpgradeComponentType {
    Default = 0,
    Gem     = 1,
    Rune    = 2,
    Sigil   = 3,
}

#[repr(C)]
pub struct ItemWeapon {
    pub _00:    *const (),
    pub _08:    u32,
    pub r#type: ItemWeaponType,
    pub _10:    u32,
    pub _14:    u32,
    pub _18:    u32,
    pub _1c:    u32,
    pub _20:    u32,
}

#[repr(u32)]
pub enum ItemWeaponType {
    Sword        = 0,
    Hammer       = 1,
    LongBow      = 2,
    ShortBow     = 3,
    Axe          = 4,
    Dagger       = 5,
    Greatsword   = 6,
    Mace         = 7,
    Pistol       = 8,
    Rifle        = 10,
    Scepter      = 11,
    Staff        = 12,
    Focus        = 13,
    Torch        = 14,
    Warhorn      = 15,
    Shield       = 16,
    SmallBundle  = 17,
    LargeBundle  = 18,
    Harpoon      = 19,
    Speargun     = 20,
    Trident      = 21,
    Toy          = 22,
    ToyTwoHanded = 23,
    _25          = 25,
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct ItemFlags: u32 {
        const NO_MOVE = 1 << 28;
    }
}

#[repr(u32)]
pub enum ItemRarity {
    Junk       = 0,
    Basic      = 1,
    Fine       = 2,
    Masterwork = 3,
    Rare       = 4,
    Exotic     = 5,
    Ascended   = 6,
    Legendary  = 7,
}
