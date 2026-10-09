use super::{Guid, Name};

#[repr(C)]
pub struct Color {
    pub contentGuid:     Guid,
    pub contentType:     u32,
    pub contentUid:      u32,
    pub contentName:     *const Name,
    pub contentFullName: *const Name,
    pub dataId:          u32,
    _34:                 u32,
    pub _30:             *const [Color_30],
    _40:                 u32,
    _44:                 u32,
    pub textName:        u32,
}

#[repr(C)]
pub struct Color_30 {
    pub brightness:   f32,
    pub contrast:     f32,
    pub hue:          f32,
    pub saturation:   f32,
    pub lightness:    f32,
    pub materialType: MaterialType,
}

#[repr(u32)]
pub enum MaterialType {
    Default = 0,
    Cloth   = 1,
    Leather = 2,
    Metal   = 3,
    Fur     = 4,
}
