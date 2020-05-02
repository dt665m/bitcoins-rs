//! The `bitcoin::nets` module cotains Bitcoin network definitions. These are the main interface
//! for accessing the library.
//!
//! Expected user flow is to import the network and access the transaction builder through it.
//! This gives the user immediate access to the full bitcoin toolchain via a single import.
//!
//! ```
//! use rmn_btc::{BitcoinMainnet, enc::Address, types::Outpoint};
//! use riemann_core::{
//!     nets::Network,
//!     builder::TxBuilder,
//!     ser::Ser,
//! };
//!
//! let address = BitcoinMainnet::string_to_address("bc1qvyyvsdcd0t9863stt7u9rf37wx443lzasg0usy").unwrap();
//!
//! let b = BitcoinMainnet::tx_builder();
//! b.version(2)
//!  .spend(Outpoint::default(), 0xaabbccdd)
//!  .pay(0x8888_8888_8888_8888, &address).unwrap()
//!  .pay(0x7777_7777_7777_7777, &Address::SH("377mKFYsaJPsxYSB5aFfx8SW3RaN5BzZVh".to_owned())).unwrap()
//!  .build()
//!  .serialize_hex();
//!
//! let script = BitcoinMainnet::decode_address(&address).unwrap();
//! let re_encoded = BitcoinMainnet::encode_address(&script).unwrap();
//! assert_eq!(address, re_encoded);
//! ```
use std::marker::PhantomData;

use riemann_core::{enc::AddressEncoder, nets::Network};

use crate::{
    builder::LegacyBuilder,
    enc::{
        bases::EncodingError,
        encoder::{Address, MainnetEncoder, SignetEncoder, TestnetEncoder},
    },
    types::{
        script::ScriptPubkey,
        transactions::{LegacyTx, WitnessTransaction, WitnessTx},
        txin::BitcoinTxIn,
        txout::TxOut,
    },
};

/// A trait for a Bitcoin network. Specifies that Witness Txns must use the same Input and Output
/// format as Legacy transactions.
pub trait BitcoinNetwork<'a>: Network<'a> {
    /// An associated witness transaction type.
    type WTx: WitnessTransaction<'a, TxIn = Self::TxIn, TxOut = Self::TxOut>;
}

/// A newtype for Bitcoin networks, parameterized by an encoder. We change the encoder to
/// differentiate between main, test, and signet.
#[derive(Debug)]
pub struct Bitcoin<T: AddressEncoder>(PhantomData<*const T>);

impl<'a, T> Network<'a> for Bitcoin<T>
where
    T: AddressEncoder<Address = Address, Error = EncodingError, RecipientIdentifier = ScriptPubkey>,
{
    type Address = Address;
    type Error = EncodingError;
    type RecipientIdentifier = ScriptPubkey;
    type Encoder = T;
    type TxIn = BitcoinTxIn;
    type TxOut = TxOut;
    type Tx = LegacyTx;
    type Builder = LegacyBuilder<T>;
}

impl<'a, T> BitcoinNetwork<'a> for Bitcoin<T>
where
    T: AddressEncoder<Address = Address, Error = EncodingError, RecipientIdentifier = ScriptPubkey>,
{
    type WTx = WitnessTx;
}

/// A fully-parameterized BitcoinMainnet. This is the main interface for accessing the library.
pub type BitcoinMainnet<'a> = Bitcoin<MainnetEncoder>;

/// A fully-parameterized BitcoinTestnet. This is the main interface for accessing the library.
pub type BitcoinTestnet<'a> = Bitcoin<TestnetEncoder>;

/// A fully-parameterized BitcoinSignet. This is the main interface for accessing the library.
pub type BitcoinSignet<'a> = Bitcoin<SignetEncoder>;

#[cfg(test)]
mod test {
    use super::*;
    use crate::types::txin::BitcoinOutpoint;
    use riemann_core::{builder::TxBuilder, ser::Ser};

    #[test]
    fn it_has_sensible_syntax() {
        let tx_hex = BitcoinMainnet::tx_builder()
            .version(2)
            .spend(BitcoinOutpoint::default(), 0xaabbccdd)
            .pay(
                0x8888_8888_8888_8888,
                &Address::WPKH("bc1qvyyvsdcd0t9863stt7u9rf37wx443lzasg0usy".to_owned()),
            )
            .unwrap()
            .pay(
                0x7777_7777_7777_7777,
                &Address::SH("377mKFYsaJPsxYSB5aFfx8SW3RaN5BzZVh".to_owned()),
            )
            .unwrap()
            .build()
            .serialize_hex()
            .unwrap();
        BitcoinMainnet::builder_from_hex(&tx_hex).unwrap();
        // println!("{:?}", b);
    }

    #[test]
    fn it_exposes_encoder_interface() {
        let addr_string = "bc1qvyyvsdcd0t9863stt7u9rf37wx443lzasg0usy".to_owned();
        let address = Address::WPKH(addr_string.clone());
        assert_eq!(&address, &BitcoinMainnet::string_to_address(&addr_string).unwrap());
        let u = BitcoinMainnet::decode_address(&address).unwrap();
        assert_eq!(&address, &BitcoinMainnet::encode_address(&u).unwrap())
    }
}
