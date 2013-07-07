use std::cast;
use std::unstable::intrinsics;

#[simd] pub struct i8x16(i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8);
#[simd] pub struct i16x8(i16, i16, i16, i16, i16, i16, i16, i16);
#[simd] pub struct i32x4(i32, i32, i32, i32);
#[simd] pub struct i64x2(i64, i64);

#[simd] pub struct u8x16(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8);
#[simd] pub struct u16x8(u16, u16, u16, u16, u16, u16, u16, u16);
#[simd] pub struct u32x4(u32, u32, u32, u32);
#[simd] pub struct u64x2(u64, u64);

pub trait ByteSwap {
    pub fn to_big_endian(&self) -> Self;
    pub fn to_little_endian(&self) -> Self;
}

impl ByteSwap for i8x16 {
    pub fn to_big_endian(&self) -> i8x16 { return *self; }
    pub fn to_little_endian(&self) -> i8x16 { return *self; }
}

impl ByteSwap for i16x8 {
    pub fn to_big_endian(&self) -> i16x8 {
        return match self {
            &i16x8(s0, s1, s2, s3, s4, s5, s6, s7) => unsafe {
                i16x8(intrinsics::to_be16(s0), intrinsics::to_be16(s1), intrinsics::to_be16(s2), intrinsics::to_be16(s3), intrinsics::to_be16(s4), intrinsics::to_be16(s5), intrinsics::to_be16(s6), intrinsics::to_be16(s7))
            }
        };
    }

    pub fn to_little_endian(&self) -> i16x8 {
        return match self {
            &i16x8(s0, s1, s2, s3, s4, s5, s6, s7) => unsafe {
                i16x8(intrinsics::to_le16(s0), intrinsics::to_le16(s1), intrinsics::to_le16(s2), intrinsics::to_le16(s3), intrinsics::to_le16(s4), intrinsics::to_le16(s5), intrinsics::to_le16(s6), intrinsics::to_le16(s7))
            }
        };
    }
}

impl ByteSwap for i32x4 {
    pub fn to_big_endian(&self) -> i32x4 {
        return match self {
            &i32x4(s0, s1, s2, s3) => unsafe {
                i32x4(intrinsics::to_be32(s0), intrinsics::to_be32(s1), intrinsics::to_be32(s2), intrinsics::to_be32(s3))
            }
        };
    }

    pub fn to_little_endian(&self) -> i32x4 {
        return match self {
            &i32x4(s0, s1, s2, s3) => unsafe {
                i32x4(intrinsics::to_le32(s0), intrinsics::to_le32(s1), intrinsics::to_le32(s2), intrinsics::to_le32(s3))
            }
        };
    }
}

impl ByteSwap for i64x2 {
    pub fn to_big_endian(&self) -> i64x2 {
        return match self {
            &i64x2(s0, s1) => unsafe {
                i64x2(intrinsics::to_be64(s0), intrinsics::to_be64(s1))
            }
        };
    }

    pub fn to_little_endian(&self) -> i64x2 {
        return match self {
            &i64x2(s0, s1) => unsafe {
                i64x2(intrinsics::to_le64(s0), intrinsics::to_le64(s1))
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
        return unsafe { cast::transmute(cast::transmute::<&u16x8, &i16x8>(self).to_big_endian()) };
    }

    pub fn to_little_endian(&self) -> u16x8 {
        return unsafe { cast::transmute(cast::transmute::<&u16x8, &i16x8>(self).to_little_endian()) };
    }
}

impl ByteSwap for u32x4 {
    pub fn to_big_endian(&self) -> u32x4 {
        return unsafe { cast::transmute(cast::transmute::<&u32x4, &i32x4>(self).to_big_endian()) };
    }

    pub fn to_little_endian(&self) -> u32x4 {
        return unsafe { cast::transmute(cast::transmute::<&u32x4, &i32x4>(self).to_little_endian()) };
    }
}

impl ByteSwap for u64x2 {
    pub fn to_big_endian(&self) -> u64x2 {
        return unsafe { cast::transmute(cast::transmute::<&u64x2, &i64x2>(self).to_big_endian()) };
    }

    pub fn to_little_endian(&self) -> u64x2 {
        return unsafe { cast::transmute(cast::transmute::<&u64x2, &i64x2>(self).to_little_endian()) };
    }
}

impl BitXor<u8x16, u8x16> for u8x16 {
    pub fn bitxor(&self, rhs:&u8x16) -> u8x16 {
        return match (self, rhs) {
            (&u8x16(s0, s1, s2, s3, s4, s5, s6, s7, s8, s9, s10, s11, s12, s13, s14, s15), &u8x16(r0, r1, r2, r3, r4, r5, r6, r7, r8, r9, r10, r11, r12, r13, r14, r15)) => u8x16(s0 ^ r0, s1 ^ r1, s2 ^ r2, s3 ^ r3, s4 ^ r4, s5 ^ r5, s6 ^ r6, s7 ^ r7, s8 ^ r8, s9 ^ r9, s10 ^ r10, s11 ^ r11, s12 ^ r12, s13 ^ r13, s14 ^ r14, s15 ^ r15)
        };
    }
}

impl BitXor<u16x8, u16x8> for u16x8 {
    pub fn bitxor(&self, rhs:&u16x8) -> u16x8 {
        return match (self, rhs) {
            (&u16x8(s0, s1, s2, s3, s4, s5, s6, s7), &u16x8(r0, r1, r2, r3, r4, r5, r6, r7)) => u16x8(s0 ^ r0, s1 ^ r1, s2 ^ r2, s3 ^ r3, s4 ^ r4, s5 ^ r5, s6 ^ r6, s7 ^ r7)
        };
    }
}

impl BitXor<u32x4, u32x4> for u32x4 {
    pub fn bitxor(&self, rhs:&u32x4) -> u32x4 {
        return match (self, rhs) {
            (&u32x4(s0, s1, s2, s3), &u32x4(r0, r1, r2, r3)) => u32x4(s0 ^ r0, s1 ^ r1, s2 ^ r2, s3 ^ r3)
        };
    }
}

impl BitXor<u64x2, u64x2> for u64x2 {
    pub fn bitxor(&self, rhs:&u64x2) -> u64x2 {
        return match (self, rhs) {
            (&u64x2(s0, s1), &u64x2(r0, r1)) => u64x2(s0 ^ r0, s1 ^ r1)
        };
    }
}
