#[link(name = "crypto", vers = "0.1", uuid = "6ea1f5fc-9e23-46df-b03b-f730fea6e432")];

#[license = "MIT"];
#[crate_type = "lib"];

#[author = "Jens Nockert"];

#[comment = "A library of Cryptographic primitives"];
#[desc = "A library of Cryptographic primitives"];

pub mod simd;
pub mod rotate;
pub mod byteswap;

pub mod blockcipher {
    pub trait BlockCipher<Block> {
        pub fn encrypt(&mut self, in:Block) -> Block;
        pub fn decrypt(&mut self, in:Block) -> Block;
    }

    pub mod aes;
    pub mod ecb;
    pub mod cbc;
    pub mod ctr;
}

pub mod hash {
    pub trait Hash<Block, Result> {
        pub fn update(&mut self, in:Block);
        pub fn finish(&mut self) -> Result;
    }

    pub mod sha256;
}

enum Direction { Encrypt, Decrypt }