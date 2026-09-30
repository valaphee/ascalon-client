use super::{Name, Progress};

#[repr(C)]
pub struct Map {
    pub content_guid: [u8; 16],
    pub content_type: u32,
    pub content_uid: u32,
    pub content_name: *const Name,
    pub content_full_name: *const Name,
    pub data_id: u32,
    pub r#type: MapType,
    pub _030: u32,
    pub _034: u32,
    pub _038: *const u16, // file
    pub _040: *const u16, // file
    pub _048: *const u16, // file
    pub _050: *const u16, // file
    pub _058: *const u16, // file
    pub _060: *const u16, // file
    pub file: *const u16, // file
    pub _070: u32,
    pub _074: u32,
    pub _078: *const u16, // file
    pub _080: *const (),
    pub _088: u32,
    pub _08c: u32,
    pub _090: *const u16, // file
    pub _098: MapFlags,
    pub _09c: u32,
    pub _0a0: u32,
    pub _0a4: u32,
    pub min_level: u32,
    pub max_level: u32,
    pub _0b0: *const u16,
    pub _0b8: u32,
    pub _0bc: u32,
    pub _0c0: *const (), // type 0x02B
    pub _0c8: *const (), // type 0x001
    pub _0d0: *const (), // type 0x04A
    pub _0d8: *const u16,
    pub _0e0: u32,
    pub _0e4: u32,
    pub _0e8: u32,
    pub _0ec: u32,
    pub _0f0: u32,
    pub _0f4: u32,
    pub _0f8: *const (), // type 0x0AD
    pub pvp: *const (),
    pub _108: *const (), // type 0x03C
    pub _110: *const u16,
    pub _118: u32,
    pub _11c: u32,
    pub _120: u32,
    pub _124: u32,
    pub _128: u32,
    pub _12c: u32,
    pub _130: u32,
    pub _134: u32,
    pub _138: u32,
    pub _13c: u32,
    pub _140: u32,
    pub _144: u32,
    pub _148: u32,
    pub _14c: u32,
    pub _150: u32,
    pub _154: u32,
    pub _158: u32,
    pub _15c: u32,
    pub _160: u32,
    pub _164: u32,
    pub name: u32,
    pub description: u32,
    pub _170: u32,
    pub _174: u32,
    pub _178: *const (),
    pub _180: u32,
    pub _184: u32,
    pub _188: u32,
    pub _18c: u32,
    pub _190: *const (),
    pub _198: u32,
    pub _19c: u32,
    pub _1a0: [u8; 16],
    pub _1b0: u32,
    pub _1b4: u32,
    pub _1b8: *const (), // type 0x109
    pub _1c0: u32,
    pub _1c4: u32,
    pub _1c8: u32,
    pub _1cc: u32,
    pub _1d0: *const (),
    pub _1d8: *const Progress,
    pub _1e0: u32,
    pub _1e4: u32,
    pub _1e8: u32,
    pub _1ec: u32,
    pub _1f0: *const Progress,
    pub _1f8: *const (),
    pub _200: *const (),
    pub _208: u32,
    pub _20c: u32,
    pub _210: *const (),
    pub _218: u32,
    pub _21c: u32,
    pub _220: *const (), // type 0x0A8
}

#[repr(u32)]
pub enum MapType {
    _0 = 0,
    _1 = 1,
    Pvp = 2,
    Instance = 4,
    _5 = 5,
    Tutorial = 7,
    Center = 9,
    BlueHome = 10,
    GreenHome = 11,
    RedHome = 12,
    JumpPuzzle = 14,
    EdgeOfTheMists = 15,
    _16 = 16,
    Unknown = 18,
    _19 = 19,
}

bitflags::bitflags! {
    #[derive(Debug)]
    #[repr(transparent)]
    pub struct MapFlags: u32 {
        const INSTANCE_CHECKPOINT_OVERRIDE = 1 << 17;
    }
}
