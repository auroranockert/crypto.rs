#[simd] pub struct i8x16(i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8);
#[simd] pub struct i16x8(i16, i16, i16, i16, i16, i16, i16, i16);
#[simd] pub struct i32x4(i32, i32, i32, i32);
#[simd] pub struct i64x2(i64, i64);

#[simd] pub struct u8x16(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8);
#[simd] pub struct u16x8(u16, u16, u16, u16, u16, u16, u16, u16);
#[simd] pub struct u32x4(u32, u32, u32, u32);
#[simd] pub struct u64x2(u64, u64);

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
