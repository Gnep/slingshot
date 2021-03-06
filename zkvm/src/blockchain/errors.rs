use crate::errors::VMError;

#[derive(Debug, Fail)]
pub enum BlockchainError {
    #[fail(display = "version reversion")]
    VersionReversion,

    #[fail(display = "illegal extension")]
    IllegalExtension,

    #[fail(display = "bad height")]
    BadHeight,

    #[fail(display = "mismatched previous-block ID")]
    MismatchedPrev,

    #[fail(display = "bad block timestamp")]
    BadBlockTimestamp,

    #[fail(display = "bad tx timestamp")]
    BadTxTimestamp,

    #[fail(display = "bad tx version")]
    BadTxVersion,

    #[fail(display = "txroot mismatch")]
    TxrootMismatch,

    #[fail(display = "tx validation")]
    TxValidation(VMError),
}
