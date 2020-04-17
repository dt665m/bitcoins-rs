//! This module holds `MarkedDigest` types used by Bitcoin transactions. Currently we represent
//! only `TXID`s and `WTXID`s. In the future we may also represent sighash digests this way.

use std::io::{Read, Write};
use bitcoin_spv::types::{Hash256Digest};

use riemann_core::{
    hashes::marked::{MarkedDigest},
    ser::{Ser, SerError, SerResult},
};

mark_hash256!(
    /// A marked Hash256Digest representing transaction IDs
    TXID
);
mark_hash256!(
    /// A marked Hash256Digest representing witness transaction IDs
    WTXID
);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_serializes_and_derializes_hash256digests() {
        let cases = [
            (TXID::default(), "0000000000000000000000000000000000000000000000000000000000000000"),
        ];
        for case in cases.iter() {
            let digest = TXID::deserialize_hex(case.1.to_owned()).unwrap();
            assert_eq!(digest.serialized_length(), 32);
            assert_eq!(digest, case.0);
            assert_eq!(digest.serialize_hex().unwrap(), case.1);
            assert_eq!(case.0.serialize_hex().unwrap(), case.1);
        }
    }
}
