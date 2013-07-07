use blockcipher::BlockCipher;

pub struct CBCMode<B, C> {
    iv:B, key:C
}

impl<B, C> CBCMode<B, C> {
    pub fn create(iv:B, key:C) -> CBCMode<B, C> {
        return CBCMode { iv:iv, key:key };
    }
}

impl<B:BitXor<B, B> + Copy, C:BlockCipher<B>> BlockCipher<B> for CBCMode<B, C> {
    pub fn encrypt(&mut self, in:B) -> B {
        let result = self.key.encrypt(in ^ self.iv);

        self.iv = copy result;

        return result;
    }

    pub fn decrypt(&mut self, in:B) -> B {
        let result = self.key.decrypt(copy in) ^ self.iv;

        self.iv = in;

        return result;
    }
}
