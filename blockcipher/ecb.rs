use blockcipher::BlockCipher;

pub struct ECBMode<C> {
    key: C
}

impl<C> ECBMode<C> {
    pub fn create(key:C) -> ECBMode<C> {
        return ECBMode { key:key };
    }
}

impl<B, C:BlockCipher<B>> BlockCipher<B> for ECBMode<C> {
    pub fn encrypt(&mut self, in:B) -> B {
        return self.key.encrypt(in);
    }

    pub fn decrypt(&mut self, in:B) -> B {
        return self.key.decrypt(in);
    }
}
