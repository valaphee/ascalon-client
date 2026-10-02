use super::{Name, WcharPtr};

#[repr(C)]
pub struct Skill {
    pub contentGuid:     [u8; 16],
    pub contentType:     u32,
    pub contentUid:      u32,
    pub contentName:     *const Name,
    pub contentFullName: *const Name,
    pub dataId:          u32,
    pub _2c:             u32,
    pub _30:             u32,
    pub textName:        u32,
    pub flags:           SkillFlags,
    _3c:                 u32,
    pub _40:             *const [()],
    pub fileIcon:        WcharPtr,
    pub _58:             u32,
    pub _5c:             u32,
    pub _60:             *const (),
    pub _68:             *const [()],
    pub _78:             *const (),
    _80:                 u32,
    _84:                 u32,
    pub _88:             *const [()],
    pub _98:             u32,
    _9c:                 u32,
    pub _a0:             *const [()],
    pub _b0:             *const (),
}

bitflags::bitflags! {
    #[repr(transparent)]
    pub struct SkillFlags: u32 {
        const GROUND_TARGETED = 1 << 12;
    }
}
