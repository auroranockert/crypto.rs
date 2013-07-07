use std::unstable::intrinsics;

use simd::{i8x16, i16x8, i32x4, i64x2};
use simd::{u8x16, u16x8, u32x4, u64x2};

pub trait ByteSwap {
    pub fn to_big_endian(&self) -> Self;
    pub fn to_little_endian(&self) -> Self;
}

impl ByteSwap for i8 {
    pub fn to_big_endian(&self) -> i8 { return *self; }
    pub fn to_little_endian(&self) -> i8 { return *self; }
}

impl ByteSwap for i16 {
    pub fn to_big_endian(&self) -> i16 { return intrinsics::to_be16(*self); }
    pub fn to_little_endian(&self) -> i16 { return intrinsics::to_le16(*self); }
}

impl ByteSwap for i32 {
    pub fn to_big_endian(&self) -> i32 { return intrinsics::to_be32(*self); }
    pub fn to_little_endian(&self) -> i32 { return intrinsics::to_le32(*self); }
}


impl ByteSwap for i64 {
    pub fn to_big_endian(&self) -> i64 { return intrinsics::to_be64(*self); }
    pub fn to_little_endian(&self) -> i64 { return intrinsics::to_le64(*self); }
}

impl ByteSwap for u8 {
    pub fn to_big_endian(&self) -> u8 { return *self; }
    pub fn to_little_endian(&self) -> u8 { return *self; }
}

impl ByteSwap for u16 {
    pub fn to_big_endian(&self) -> u16 { return (*self as i16).to_big_endian() as u16; }
    pub fn to_little_endian(&self) -> u16 { return (*self as i16).to_big_endian() as u16; }
}

impl ByteSwap for u32 {
    pub fn to_big_endian(&self) -> u32 { return (*self as i32).to_big_endian() as u32; }
    pub fn to_little_endian(&self) -> u32 { return (*self as i32).to_big_endian() as u32; }
}

impl ByteSwap for u64 {
    pub fn to_big_endian(&self) -> u64 { return (*self as i64).to_big_endian() as u64; }
    pub fn to_little_endian(&self) -> u64 { return (*self as i64).to_big_endian() as u64; }
}

impl ByteSwap for i8x16 {
    pub fn to_big_endian(&self) -> i8x16 {
        return *self;
    }

    pub fn to_little_endian(&self) -> i8x16 {
        return *self;
    }
}

impl ByteSwap for i16x8 {
    pub fn to_big_endian(&self) -> i16x8 {
        return match self {
            &i16x8(s0, s1, s2, s3, s4, s5, s6, s7) => {
                i16x8(s0.to_big_endian(), s1.to_big_endian(), s2.to_big_endian(), s3.to_big_endian(), s4.to_big_endian(), s5.to_big_endian(), s6.to_big_endian(), s7.to_big_endian())
            }
        };
    }

    pub fn to_little_endian(&self) -> i16x8 {
        return match self {
            &i16x8(s0, s1, s2, s3, s4, s5, s6, s7) => {
                i16x8(s0.to_little_endian(), s1.to_little_endian(), s2.to_little_endian(), s3.to_little_endian(), s4.to_little_endian(), s5.to_little_endian(), s6.to_little_endian(), s7.to_little_endian())
            }
        };
    }
}

impl ByteSwap for i32x4 {
    pub fn to_big_endian(&self) -> i32x4 {
        return match self {
            &i32x4(s0, s1, s2, s3) => {
                i32x4(s0.to_big_endian(), s1.to_big_endian(), s2.to_big_endian(), s3.to_big_endian())
            }
        };
    }

    pub fn to_little_endian(&self) -> i32x4 {
        return match self {
            &i32x4(s0, s1, s2, s3) => {
                i32x4(s0.to_little_endian(), s1.to_little_endian(), s2.to_little_endian(), s3.to_little_endian())
            }
        };
    }
}

impl ByteSwap for i64x2 {
    pub fn to_big_endian(&self) -> i64x2 {
        return match self {
            &i64x2(s0, s1) => {
                i64x2(s0.to_big_endian(), s1.to_big_endian())
            }
        };
    }

    pub fn to_little_endian(&self) -> i64x2 {
        return match self {
            &i64x2(s0, s1) => {
                i64x2(s0.to_little_endian(), s1.to_little_endian())
            }
        };
    }
}

impl ByteSwap for u8x16 {
    pub fn to_big_endian(&self) -> u8x16 {
        return *self;
    }

    pub fn to_little_endian(&self) -> u8x16 {
        return *self;
    }
}

impl ByteSwap for u16x8 {
    pub fn to_big_endian(&self) -> u16x8 {
        return match self {
            &u16x8(s0, s1, s2, s3, s4, s5, s6, s7) => {
                u16x8(s0.to_big_endian(), s1.to_big_endian(), s2.to_big_endian(), s3.to_big_endian(), s4.to_big_endian(), s5.to_big_endian(), s6.to_big_endian(), s7.to_big_endian())
            }
        };
    }

    pub fn to_little_endian(&self) -> u16x8 {
        return match self {
            &u16x8(s0, s1, s2, s3, s4, s5, s6, s7) => {
                u16x8(s0.to_little_endian(), s1.to_little_endian(), s2.to_little_endian(), s3.to_little_endian(), s4.to_little_endian(), s5.to_little_endian(), s6.to_little_endian(), s7.to_little_endian())
            }
        };
    }
}

impl ByteSwap for u32x4 {
    pub fn to_big_endian(&self) -> u32x4 {
        return match self {
            &u32x4(s0, s1, s2, s3) => {
                u32x4(s0.to_big_endian(), s1.to_big_endian(), s2.to_big_endian(), s3.to_big_endian())
            }
        };
    }

    pub fn to_little_endian(&self) -> u32x4 {
        return match self {
            &u32x4(s0, s1, s2, s3) => {
                u32x4(s0.to_little_endian(), s1.to_little_endian(), s2.to_little_endian(), s3.to_little_endian())
            }
        };
    }
}

impl ByteSwap for u64x2 {
    pub fn to_big_endian(&self) -> u64x2 {
        return match self {
            &u64x2(s0, s1) => {
                u64x2(s0.to_big_endian(), s1.to_big_endian())
            }
        };
    }

    pub fn to_little_endian(&self) -> u64x2 {
        return match self {
            &u64x2(s0, s1) => {
                u64x2(s0.to_little_endian(), s1.to_little_endian())
            }
        };
    }
}
