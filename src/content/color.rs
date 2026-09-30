use super::Name;

#[repr(C)]
pub struct Color {
    pub content_guid: [u8; 16],
    pub content_type: u32,
    pub content_uid: u32,
    pub content_name: *const Name,
    pub content_full_name: *const Name,
    pub data_id: u32,
    _34: u32, // pad
    pub _30: *const [Color_30],
    _40: u32, // pad
    _44: u32, // pad
    pub name: u32,
}

#[repr(C)]
pub struct Color_30 {
    pub brightness: f32,
    pub contrast: f32,
    pub hue: f32,
    pub saturation: f32,
    pub lightness: f32,
    pub material_type: MaterialType,
}

#[repr(u32)]
pub enum MaterialType {
    _0 = 0,
    Cloth = 1,
    Leather = 2,
    Metal = 3,
    Fur = 4,
}
