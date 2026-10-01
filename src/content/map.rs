use super::{Name, Progress, String, WcharPtr};

#[repr(C)]
pub struct Map {
    pub contentGuid: [u8; 16],
    pub contentType: u32,
    pub contentUid: u32,
    pub contentName: *const Name,
    pub contentFullName: *const Name,
    pub dataId: u32,
    pub r#type: MapType,
    pub _030: u32,
    _034: u32,          // zero
    pub _038: WcharPtr, // file
    pub _040: WcharPtr, // file
    pub _048: WcharPtr, // file
    pub _050: WcharPtr, // file
    pub _058: WcharPtr, // file
    pub _060: WcharPtr, // file
    pub fileMap: WcharPtr,
    _070: u32, // zero
    _074: u32, // zero
    pub file078: WcharPtr,
    pub _080: *const [()],
    pub _090: WcharPtr, // file
    pub _098: MapFlags,
    pub _09c: u32,
    _0a0: u32, // zero
    _0a4: u32, // zero
    pub levelMin: u32,
    pub levelMax: u32,
    pub _0b0: String,
    pub _0c0: *const (), // type 0x02B
    pub _0c8: *const (), // type 0x001
    pub _0d0: *const (), // type 0x04A
    pub _0d8: String,
    _0e8: u32,           // zero
    _0ec: u32,           // zero
    _0f0: u32,           // zero
    _0f4: u32,           // zero
    pub _0f8: *const (), // type 0x0AD
    pub pvp: *const (),
    pub _108: *const (), // type 0x03C
    pub _110: String,
    _120: u32, // zero
    _124: u32, // zero
    _128: u32, // zero
    _12c: u32, // zero
    _130: u32, // zero
    _134: u32, // zero
    _138: u32, // zero
    _13c: u32, // zero
    _140: u32, // zero
    _144: u32, // zero
    _148: u32, // zero
    _14c: u32, // zero
    _150: u32, // zero
    _154: u32, // zero
    _158: u32, // zero
    _15c: u32, // zero
    _160: u32, // zero
    _164: u32, // zero
    pub textName: u32,
    pub textDescription: u32,
    pub _170: u32,
    _174: u32, // zero
    pub _178: *const [()],
    pub _188: u32,
    _18c: u32, // zero
    pub _190: *const [()],
    pub _1a0: [u8; 16],
    _1b0: u32,             // zero
    _1b4: u32,             // zero
    pub _1b8: *const [()], // type 0x109
    _1c8: u32,             // zero
    _1cc: u32,             // zero
    pub _1d0: *const (),
    pub _1d8: *const Progress,
    _1e0: u32,
    _1e4: u32,
    _1e8: u32,
    _1ec: u32,
    pub _1f0: *const Progress,
    pub _1f8: *const (),
    pub _200: *const [()],
    pub _210: *const [()],
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
    #[repr(transparent)]
    pub struct MapFlags: u32 {
        const INSTANCE_CHECKPOINT_OVERRIDE = 1 << 17;
    }
}
