use ascalon_asset::packfile::{Guid, Ptr};

use super::Name;

#[derive(Debug)]
#[repr(C)]
pub struct Color {
    pub content_guid: Guid,
    pub content_type: u32,
    pub content_uid: u32,
    pub content_name: Ptr<Name>,
    pub content_full_name: Ptr<Name>,
    pub data_id: u32,
    pub _2c: u32,
    pub _30: Ptr<Color_30>,
    pub _38: u32,
    pub _3c: u32,
    pub _40: u32,
    pub _44: u32,
    pub name: u32,
    pub _4c: u32,
}

#[derive(Debug)]
#[repr(C)]
pub struct Color_30 {
    pub brightness: f32,
    pub contrast: f32,
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
    pub material_type: MaterialType,
}

#[derive(Debug)]
#[repr(u32)]
pub enum MaterialType {
    _0 = 0,
    Cloth = 1,
    Leather = 2,
    Metal = 3,
    Fur = 4,
}
