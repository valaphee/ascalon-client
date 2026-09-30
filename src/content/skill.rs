use super::Name;

#[repr(C)]
pub struct Skill {
    pub content_guid: [u8; 16],
    pub content_type: u32,
    pub content_uid: u32,
    pub content_name: *const Name,
    pub content_full_name: *const Name,
    pub data_id: u32,
    pub _2c: u32,
    pub _30: u32,
    pub name: u32,
    pub flags: SkillFlags,
    pub _3c: u32,
    pub _40: *const (),
    pub _48: u32,
    pub _4c: u32,
    pub icon: *const u16,
    pub _58: u32,
    pub _5c: u32,
    pub _60: *const (),
    pub _68: *const (),
    pub _70: u32,
    pub _74: u32,
    pub _78: *const (), // type 0x09B
    pub _80: u32,
    pub _84: u32,
    pub _88: *const (),
    pub _90: u32,
    pub _94: u32,
    pub _98: u32,
    pub _9c: u32,
    pub _a0: *const (),
    pub _a8: u32,
    pub _ac: u32,
    pub _b0: *const (),
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct SkillFlags: u32 {
        const GROUND_TARGETED = 1 << 12;
    }
}
