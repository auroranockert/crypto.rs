use std::cast;
use std::unstable::intrinsics;

use simd;
use blockcipher::BlockCipher;

pub trait CTRIncrement {
    pub fn ctr_increment(&self) -> Self;
}

impl CTRIncrement for simd::u8x16 {
    pub fn ctr_increment(&self) -> simd::u8x16 {
        let mut result:[i64, ..2] = unsafe { cast::transmute(*self) };

        result[1] = intrinsics::to_be64(intrinsics::to_be64(result[1]) + 1);

        return unsafe { cast::transmute(result) }
    }
}

pub struct CTRMode<B, C> {
    iv:B, key:C
}

impl<B, C> CTRMode<B, C> {
    pub fn create(iv:B, key:C) -> CTRMode<B, C> {
        return CTRMode { iv:iv, key:key };
    }
}

impl<B:CTRIncrement + BitXor<B, B> + Copy, C:BlockCipher<B>> BlockCipher<B> for CTRMode<B, C> {
    pub fn encrypt(&mut self, in:B) -> B {
        let result = self.key.encrypt(self.iv) ^ in;

        self.iv = self.iv.ctr_increment();

        return result;
    }

    pub fn decrypt(&mut self, in:B) -> B {
        let result = self.key.encrypt(self.iv) ^ in;

        self.iv = self.iv.ctr_increment();

        return result;
    }
}
