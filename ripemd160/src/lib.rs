//! An implementation of the RIPEMD-160 cryptographic hash.
//!
//! # Usage
//!
//! ```rust
//! # #[macro_use] extern crate hex_literal;
//! # extern crate ripemd160;
//! # fn main() {
//! use ripemd160::{Ripemd160, Digest};
//!
//! // create a hasher object, to use it do not forget to import `Digest` trait
//! let mut hasher = Ripemd160::new();
//! // write input message
//! hasher.input(b"Hello world!");
//! // read hash digest (it will consume hasher)
//! let result = hasher.result();
//!
//! assert_eq!(result[..], hex!("7f772647d88750add82d8e1a7a3e5c0902a346a3"));
//! # }
//! ```
//!
//! Also see [RustCrypto/hashes](https://github.com/RustCrypto/hashes) readme.
#![no_std]
extern crate block_buffer;
#[macro_use] extern crate opaque_debug;
#[macro_use] pub extern crate digest;
#[cfg(feature = "std")]
extern crate std;

pub use digest::Digest;
use digest::{Input, BlockInput, FixedOutput, Reset};
use block_buffer::BlockBuffer;
use block_buffer::byteorder::{LE, ByteOrder};
use digest::generic_array::GenericArray;
use digest::generic_array::typenum::{U20, U64};

mod block;
use block::{process_msg_block, DIGEST_BUF_LEN, H0};

/// Structure representing the state of a Ripemd160 computation
#[derive(Clone)]
pub struct Ripemd160 {
    h: [u32; DIGEST_BUF_LEN],
    len: u64,
    buffer: BlockBuffer<U64>,
}

impl Default for Ripemd160 {
    fn default() -> Self {
        Ripemd160 {
            h: H0,
            len: 0,
            buffer: Default::default(),
        }
    }
}

impl BlockInput for Ripemd160 {
    type BlockSize = U64;
}

impl Input for Ripemd160 {
    fn process(&mut self, input: &[u8]) {
        // Assumes that input.len() can be converted to u64 without overflow
        self.len += input.len() as u64;
        let h = &mut self.h;
        self.buffer.input(input, |b| process_msg_block(h, b));
    }
}

impl FixedOutput for Ripemd160 {
    type OutputSize = U20;

    fn fixed_result(mut self) -> GenericArray<u8, Self::OutputSize> {
        {
            let h = &mut self.h;
            let l = self.len << 3;
            self.buffer.len64_padding::<LE, _>(l, |b| process_msg_block(h, b));
        }

        let mut out = GenericArray::default();
        LE::write_u32_into(&self.h, &mut out[..]);
        out
    }
}

impl Reset for Ripemd160 {
    fn reset(&mut self) -> Self {
        let temp = self.clone();
        self.buffer.reset();
        self.len = 0;
        self.h = H0;
        temp
    }
}

impl_opaque_debug!(Ripemd160);
impl_write!(Ripemd160);
