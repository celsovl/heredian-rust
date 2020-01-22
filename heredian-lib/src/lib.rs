use std::mem;
use std::convert::{TryInto};

pub mod net;
pub mod allegro_safe;
pub mod file_manager;

pub const ACTION_IDLE: i32 = 0;
pub const ACTION_WALK: i32 = 1;
pub const ACTION_RUN: i32 = 2;
pub const ACTION_ATTACK: i32 = 3;

pub const DIRECTION_LEFT: i32 = 1;
pub const DIRECTION_RIGHT: i32 = 2;
pub const DIRECTION_UP: i32 = 4;
pub const DIRECTION_DOWN: i32 = 8;
pub const DIRECTION_LEFTUP: i32 = DIRECTION_LEFT | DIRECTION_UP;
pub const DIRECTION_LEFTDOWN: i32 = DIRECTION_LEFT | DIRECTION_DOWN;
pub const DIRECTION_RIGHTUP: i32 = DIRECTION_RIGHT | DIRECTION_UP;
pub const DIRECTION_RIGHTDOWN: i32 = DIRECTION_RIGHT | DIRECTION_DOWN;
pub const DIRECTIONS: usize = 4;
pub const CHARS: usize =  30;
pub const LIFELESS: usize =  20;
pub const MAXCHARLIFELESS: usize =  5;

pub trait FromBytes {
    fn from_bytes(buf: &[u8]) -> Self;
}

pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

#[derive(Default, Debug, PartialEq, Clone)]
#[repr(C)]
pub struct PacketLifelessInfo {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
    pub d: i16,
    pub damage: i16,
}

#[derive(Default, Debug, PartialEq, Clone)]
#[repr(C)]
pub struct PacketCharInfo {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
    pub a: i16,
    pub d: i16,
    pub d2: i16,
    pub dhit: i16,
    pub numchar: i16,
    pub idchar: i16,
    pub totchar: i16,
    pub totenemies: i16,
    pub exit: bool,
    pub healt: i16,
    pub stamina: i16,
    pub damage: i16,
    pub idmap: i16,
    pub totlifeless: i16,
	pub step: i16,
    pub vision: i16,
    pub listlifeless: [Option<PacketLifelessInfo>; MAXCHARLIFELESS],
}

impl FromBytes for PacketCharInfo {
    fn from_bytes(buf: &[u8]) -> Self {
        let mut lifeless: [Option<PacketLifelessInfo>; MAXCHARLIFELESS] = [None,None,None,None,None];

        for i in 0..MAXCHARLIFELESS {
            let j = 39 + i * mem::size_of::<PacketLifelessInfo>();

            let life = PacketLifelessInfo {
                x: i16::from_le_bytes(buf[j..j+2].try_into().unwrap()),
                y: i16::from_le_bytes(buf[j+2..j+4].try_into().unwrap()),
                w: i16::from_le_bytes(buf[j+4..j+6].try_into().unwrap()),
                h: i16::from_le_bytes(buf[j+6..j+8].try_into().unwrap()),
                d: i16::from_le_bytes(buf[j+8..j+10].try_into().unwrap()),
                damage: i16::from_le_bytes(buf[j+10..j+12].try_into().unwrap()),
            };

            if life != Default::default() {
                lifeless[i] = Some(life);
            }
        }

        Self {
            x: i16::from_le_bytes(buf[0..2].try_into().unwrap()),
            y: i16::from_le_bytes(buf[2..4].try_into().unwrap()),
            w: i16::from_le_bytes(buf[4..6].try_into().unwrap()),
            h: i16::from_le_bytes(buf[6..8].try_into().unwrap()),
            a: i16::from_le_bytes(buf[8..10].try_into().unwrap()),
            d: i16::from_le_bytes(buf[10..12].try_into().unwrap()),
            d2: i16::from_le_bytes(buf[12..14].try_into().unwrap()),
            dhit: i16::from_le_bytes(buf[14..16].try_into().unwrap()),
            numchar: i16::from_le_bytes(buf[16..18].try_into().unwrap()),
            idchar: i16::from_le_bytes(buf[18..20].try_into().unwrap()),
            totchar: i16::from_le_bytes(buf[20..22].try_into().unwrap()),
            totenemies: i16::from_le_bytes(buf[22..24].try_into().unwrap()),
            exit: buf[24] == 1,
            healt: i16::from_le_bytes(buf[25..27].try_into().unwrap()),
            stamina: i16::from_le_bytes(buf[27..29].try_into().unwrap()),
            damage: i16::from_le_bytes(buf[29..31].try_into().unwrap()),
            idmap: i16::from_le_bytes(buf[31..33].try_into().unwrap()),
            totlifeless: i16::from_le_bytes(buf[33..35].try_into().unwrap()),
            step: i16::from_le_bytes(buf[35..37].try_into().unwrap()),
            vision: i16::from_le_bytes(buf[37..39].try_into().unwrap()),
            listlifeless: lifeless,
        }
    }
}

impl ToBytes for PacketCharInfo {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buf = vec![0u8; mem::size_of_val(self)];

        buf[0..2].copy_from_slice(&self.x.to_le_bytes());
        buf[2..4].copy_from_slice(&self.y.to_le_bytes());
        buf[4..6].copy_from_slice(&self.w.to_le_bytes());
        buf[6..8].copy_from_slice(&self.h.to_le_bytes());
        buf[8..10].copy_from_slice(&self.a.to_le_bytes());
        buf[10..12].copy_from_slice(&self.d.to_le_bytes());
        buf[12..14].copy_from_slice(&self.d2.to_le_bytes());
        buf[14..16].copy_from_slice(&self.dhit.to_le_bytes());
        buf[16..18].copy_from_slice(&self.numchar.to_le_bytes());
        buf[18..20].copy_from_slice(&self.idchar.to_le_bytes());
        buf[20..22].copy_from_slice(&self.totchar.to_le_bytes());
        buf[22..24].copy_from_slice(&self.totenemies.to_le_bytes());
        buf[24] = self.exit as u8;
        buf[25..27].copy_from_slice(&self.healt.to_le_bytes());
        buf[27..29].copy_from_slice(&self.stamina.to_le_bytes());
        buf[29..31].copy_from_slice(&self.damage.to_le_bytes());
        buf[31..33].copy_from_slice(&self.idmap.to_le_bytes());
        buf[33..35].copy_from_slice(&self.totlifeless.to_le_bytes());
        buf[35..37].copy_from_slice(&self.step.to_le_bytes());
        buf[37..39].copy_from_slice(&self.vision.to_le_bytes());

        for (i, lifeless) in self.listlifeless.iter().enumerate() {
            if let Some(lifeless) = lifeless {
                let j = 39 + i * mem::size_of_val(lifeless);
                buf[j..j+2].copy_from_slice(&lifeless.x.to_le_bytes());
                buf[j+2..j+4].copy_from_slice(&lifeless.y.to_le_bytes());
                buf[j+4..j+6].copy_from_slice(&lifeless.w.to_le_bytes());
                buf[j+6..j+8].copy_from_slice(&lifeless.h.to_le_bytes());
                buf[j+8..j+10].copy_from_slice(&lifeless.d.to_le_bytes());
                buf[j+10..j+12].copy_from_slice(&lifeless.damage.to_le_bytes());
            }
        }
    
        buf
    }
}
