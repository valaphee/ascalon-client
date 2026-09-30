use super::{Item, Name, Progress};

#[repr(C)]
pub struct Achievement {
    pub content_guid: [u8; 16],
    pub content_type: u32,
    pub content_uid: u32,
    pub content_name: *const Name,
    pub content_full_name: *const Name,
    pub data_id: u32,
    _2c: u32, // pad
    pub icon: *const u16,
    pub _38: u32,
    pub _3c: u32,
    pub _40: *const Achievement,
    pub _48: *const Progress,
    pub _50: *const Progress,
    pub _58: u32,
    pub _5c: u32,
    pub _60: *const Item,
    pub name: u32,
    pub description: u32,
    pub _70: u32,
    pub requirement: u32,
    pub tiers: *const [AchievementTier],
    pub _88: *const (), // type 0x49
    pub _90: *const (),
    pub _98: u32,
    pub _9c: u32,
    pub r#type: u32,
    pub _a4: u32,
    pub _a8: *const (),
    pub _b0: u32,
    pub _b4: u32,
    pub _b8: u32,
    pub _bc: u32,
    pub _c0: *const Item,
    pub _c8: u32,
    pub _cc: u32,
    pub _d0: *const (), // type 0x4C
    pub _d8: u32,
    pub _dc: u32,
    pub _e0: *const (),
    pub _e8: u32,
    pub _ec: u32,
    pub _f0: *const (),
    pub _f8: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct AchievementTier {}
