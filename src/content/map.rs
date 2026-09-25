use ascalon_asset::packfile::{Ptr, WcharPtr};
use zerocopy::little_endian::U32;

use super::{Guid, Name};

#[derive(Debug)]
#[repr(C)]
pub struct Map {
    pub contentGuid: Guid,
    pub contentType: U32,
    pub contentUid: U32,
    pub contentName: Ptr<Name>,
    pub contentFullName: Ptr<Name>,
    pub dataId: U32,
    pub r#type: MapType,
    pub _30: U32,
    pub _34: U32,
    pub _38: U32,
    pub _3c: U32,
    pub _40: WcharPtr,
    pub _48: U32,
    pub _4c: U32,
    pub _50: U32,
    pub _54: U32,
    pub _58: WcharPtr,
    pub _60: WcharPtr,
    pub _68: WcharPtr,
    pub _70: U32,
    pub _74: U32,
    pub _78: WcharPtr,
    pub _80: U32,
    pub _84: U32,
    pub _88: U32,
    pub _8c: U32,
    pub _90: U32,
    pub _94: U32,
    pub flags: U32,
}

#[derive(Debug)]
#[repr(u32)]
pub enum MapType {
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
}
