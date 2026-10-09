use super::{Guid, Name, Ptr};

#[derive(Debug)]
#[repr(C)]
pub struct Color {
    pub contentGuid:     Guid,
    pub contentType:     u32,
    pub contentUid:      u32,
    pub contentName:     Ptr<Name>,
    pub contentFullName: Ptr<Name>,
    pub dataId:          u32,
    _34:                 u32,
    pub materials:       Ptr<[ColorMaterial]>,
    _40:                 u32,
    _44:                 u32,
    pub textName:        u32,
    _4c:                 u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct ColorMaterial {
    pub brightness: f32,
    pub contrast:   f32,
    pub hue:        f32,
    pub saturation: f32,
    pub lightness:  f32,
    pub r#type:     ColorMaterialType,
}

#[derive(Debug)]
#[repr(u32)]
pub enum ColorMaterialType {
    Default = 0,
    Cloth   = 1,
    Leather = 2,
    Metal   = 3,
    Fur     = 4,
}
