// Copyright 2020 - developers of the `grammers` project.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::fmt;

/// Telegram's [Authorization Key](https://core.telegram.org/mtproto/auth_key).
///
/// This library does not provide the means to generate a valid key,
/// because doing so relies on (de-)serializing Telegram types.
#[derive(Clone, Eq)]
pub struct AuthKey {
    pub(crate) data: [u8; 256],

    pub(crate) aux_hash: [u8; 8],
    pub(crate) id: [u8; 8],
}

impl PartialEq for AuthKey {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl AuthKey {
    /// Creates a new authorization key.
    pub fn from_bytes(data: [u8; 256]) -> Self {
        let sha1 = crate::sha1!(&data);

        let mut aux_hash = [0; 8];
        aux_hash.copy_from_slice(&sha1[0..8]);

        let mut id = [0; 8];
        id.copy_from_slice(&sha1[12..12 + 8]);

        Self { data, aux_hash, id }
    }

    /// Returns a shared reference to the underlying data.
    pub fn as_bytes(&self) -> &[u8; 256] {
        &self.data
    }

    /// Consumes the `AuthKey`, returning underlying data.
    pub fn into_bytes(self) -> [u8; 256] {
        self.data
    }

    /// The 64 lower-order bits of the SHA1 hash of the authorization key.
    /// https://core.telegram.org/mtproto/description#key-identifier-auth-key-id
    pub fn id(&self) -> &[u8; 8] {
        &self.id
    }

    /// Calculates the new nonce hash based on the current attributes.
    pub fn calc_new_nonce_hash(&self, new_nonce: &[u8; 32], number: u8) -> [u8; 16] {
        let mut buffer = [0; 32 + 1 + 8];

        buffer[..32].copy_from_slice(new_nonce);
        buffer[32] = number;
        buffer[33..].copy_from_slice(&self.aux_hash);

        crate::sha1!(buffer)[4..].try_into().unwrap()
    }
}

impl fmt::Debug for AuthKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let id = u64::from_le_bytes(self.id);

        f.debug_struct("AuthKey")
            .field("id", &format_args!("0x{:016x}", id))
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_auth_key() -> AuthKey {
        let mut buffer = [0u8; 256];
        buffer
            .iter_mut()
            .enumerate()
            .for_each(|(i, x)| *x = i as u8);

        AuthKey::from_bytes(buffer)
    }

    fn get_test_new_nonce() -> [u8; 32] {
        let mut buffer = [0u8; 32];
        buffer
            .iter_mut()
            .enumerate()
            .for_each(|(i, x)| *x = i as u8);

        buffer
    }

    #[test]
    fn auth_key_aux_hash() {
        let auth_key = get_test_auth_key();
        let expected = [73, 22, 214, 189, 183, 247, 142, 104];

        assert_eq!(auth_key.aux_hash, expected);
    }

    #[test]
    fn auth_key_id() {
        let auth_key = get_test_auth_key();
        let expected = [50, 209, 88, 110, 164, 87, 223, 200];

        assert_eq!(auth_key.id, expected);
    }

    #[test]
    fn calc_new_nonce_hash1() {
        let auth_key = get_test_auth_key();
        let new_nonce = get_test_new_nonce();
        assert_eq!(
            auth_key.calc_new_nonce_hash(&new_nonce, 1),
            [
                194, 206, 210, 179, 62, 89, 58, 85, 210, 127, 74, 93, 171, 238, 124, 103
            ]
        );
    }

    #[test]
    fn calc_new_nonce_hash2() {
        let auth_key = get_test_auth_key();
        let new_nonce = get_test_new_nonce();
        assert_eq!(
            auth_key.calc_new_nonce_hash(&new_nonce, 2),
            [
                244, 49, 142, 133, 189, 47, 243, 190, 132, 217, 254, 252, 227, 220, 227, 159
            ]
        );
    }

    #[test]
    fn calc_new_nonce_hash3() {
        let auth_key = get_test_auth_key();
        let new_nonce = get_test_new_nonce();
        assert_eq!(
            auth_key.calc_new_nonce_hash(&new_nonce, 3),
            [
                75, 249, 215, 179, 125, 180, 19, 238, 67, 29, 40, 81, 118, 49, 203, 61
            ]
        );
    }
}
