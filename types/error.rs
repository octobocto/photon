use thiserror::Error;

#[derive(Debug, Error)]
#[error("Bitcoin amount overflow")]
pub struct AmountOverflow;

#[derive(Debug, Error)]
#[error("Bitcoin amount underflow")]
pub struct AmountUnderflow;

#[derive(Debug, Error)]
#[error("fips205 signing error: `{}`", .0)]
#[repr(transparent)]
pub struct Fips205Signing(pub(crate) &'static str);

#[derive(Debug, Error)]
#[error("failed to decode signing key from bytes: `{}`", .0)]
#[repr(transparent)]
pub struct DecodeSigningKey(pub(crate) &'static str);

#[derive(Debug, Error)]
pub enum Authorization {
    #[error("borsh serialization error")]
    BorshSerialize(#[from] borsh::io::Error),
    #[error("invalid signature")]
    InvalidSignature,
    #[error("not enough authorizations")]
    NotEnoughAuthorizations,
    #[error(transparent)]
    Signing(#[from] Fips205Signing),
    #[error("too many authorizations")]
    TooManyAuthorizations,
    #[error(
        "wrong key for address: address = {address},
             hash(verifying_key) = {hash_verifying_key}"
    )]
    WrongKeyForAddress {
        address: crate::Address,
        hash_verifying_key: crate::Address,
    },
}

#[derive(Debug, Error)]
pub enum ComputeFee {
    #[error("underfunded (value in < value out)")]
    Underfunded,
    #[error("value in overflow")]
    ValueInOverflow(#[source] AmountOverflow),
    #[error("value out overflow")]
    ValueOutOverflow(#[source] AmountOverflow),
}

#[derive(Debug, Error)]
pub enum ParseAddress {
    #[error("bs58 error")]
    Bs58(#[from] bitcoin::base58::InvalidCharacterError),
    #[error("wrong address length {0} != 20")]
    WrongLength(usize),
}

#[derive(Debug, Error)]
#[error("utreexo error ({0})")]
#[repr(transparent)]
pub struct Utreexo(pub(crate) String);
