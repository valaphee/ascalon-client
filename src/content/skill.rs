use ascalon_asset::packfile::{Ptr, WcharPtr};

use super::{Guid, Name};

#[derive(Debug)]
#[repr(C)]
pub struct Skill {
    pub content_guid: Guid,
    pub content_type: u32,
    pub content_uid: u32,
    pub content_name: Ptr<Name>,
    pub content_full_name: Ptr<Name>,
    pub data_id: u32,
    pub _2c: u32,
    pub _30: u32,
    pub name: u32,
    pub flags: SkillFlags,
    pub _3c: u32,
    pub _40: Ptr<()>,
    pub _48: u32,
    pub _4c: u32,
    pub icon: WcharPtr,
    pub _58: u32,
    pub _5c: u32,
    pub _60: Ptr<()>,
    pub _68: Ptr<()>,
    pub _70: u32,
    pub _74: u32,
    pub _78: Ptr<()>,
    pub _80: u32,
    pub _84: u32,
    pub _88: Ptr<()>,
    pub _90: u32,
    pub _94: u32,
    pub _98: u32,
    pub _9c: u32,
    pub _a0: Ptr<()>,
    pub _a8: u32,
    pub _ac: u32,
    pub _b0: Ptr<()>,
}

bitflags::bitflags! {
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct SkillFlags: u32 {
        const GROUND_TARGETED = 1 << 12;
    }
}
