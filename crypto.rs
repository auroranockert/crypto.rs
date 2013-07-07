#[link(name = "crypto", vers = "0.1", uuid = "6ea1f5fc-9e23-46df-b03b-f730fea6e432")];

#[license = "MIT"];
#[crate_type = "lib"];

#[author = "Jens Nockert"];

#[comment = "A library of Cryptographic primitives"];
#[desc = "A library of Cryptographic primitives"];

pub mod simd;

pub mod blockcipher {
    pub trait BlockCipher<T> {
        pub fn encrypt(&mut self, in:T) -> T;
        pub fn decrypt(&mut self, in:T) -> T;
    }

    pub mod aes;
    pub mod ecb;
    pub mod cbc;
    pub mod ctr;
}

enum Direction { Encrypt, Decrypt }