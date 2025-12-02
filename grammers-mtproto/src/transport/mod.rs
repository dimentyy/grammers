// Copyright 2020 - developers of the `grammers` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Implementation of the several [MTProto transports].
//!
//! This layer is responsible for taking serialized messages from the MTP and
//! packing them in a format that can be sent over a protocol, such as TCP, HTTP or UDP.
//!
//! [MTProto transports]: https://core.telegram.org/mtproto#mtproto-transport
mod abridged;
mod error;
mod full;
mod intermediate;
mod obfuscated;

use std::ops::Range;

use grammers_crypto::DequeBuffer;

pub use abridged::Abridged;
pub use error::Error;
pub use full::Full;
pub use intermediate::Intermediate;
pub use obfuscated::Obfuscated;

/// Result from calling [`Transport::unpack`].
#[derive(Clone, Debug, PartialEq)]
pub struct UnpackedOffset {
    /// Range of the inner data contained within the unpacked transport message.
    pub data_range: Range<usize>,
    /// Offset where the next transport message will start within the buffer.
    pub next_offset: usize,
}

/// The trait used by the transports to create instances of themselves.
pub trait Transport {
    /// Packs the input buffer in-place.
    ///
    /// Panics if `buffer.len()` is not divisible by 4.
    fn pack(&mut self, buffer: &mut DequeBuffer<u8>);

    /// Unpacks the input buffer in-place.
    ///
    /// Subsequent calls to `unpack` should be made with the same buffer,
    /// with the data on the ranges from previous `UnpackedOffset` removed.
    /// Failing to do so will cause transports such as [`Obfuscated`] to misbehave.
    fn unpack(&mut self, buffer: &mut [u8]) -> Result<UnpackedOffset, Error>;
}

/// The trait used by the obfuscated transport to get the transport tags.
pub trait Tagged {
    /// Gets the transport tag for use in the obfuscated transport and
    /// changes the internal state to avoid sending the tag again.
    fn init_tag(&mut self) -> [u8; 4];
}
