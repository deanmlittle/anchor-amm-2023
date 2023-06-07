use anchor_lang::{error_code, error::Error};
use constant_product_curve::CurveError;
use solana_program::program_error::ProgramError;

#[error_code]
pub enum AmmError {
    #[msg("DefaultError")]
    DefaultError,
    #[msg("Offer expired.")]
    OfferExpired,
    #[msg("This pool is locked.")]
    PoolLocked,
    #[msg("Slippage exceeded.")]
    SlippageExceeded,
    #[msg("Overflow detected.")]
    Overflow,
    #[msg("Underflow detected.")]
    Underflow,
    #[msg("Invalid token.")]
    InvalidToken,
    #[msg("Actual liquidity is less than minimum.")]
    LiquidityLessThanMinimum,
    #[msg("No liquidity in pool.")]
    NoLiquidityInPool,
    #[msg("Bump error.")]
    BumpError,
    #[msg("Curve error.")]
    CurveError,
    #[msg("Fee is greater than 100%. This is not a very good deal.")]
    InvalidFee,
    #[msg("Invalid update authority.")]
    InvalidAuthority,
    #[msg("No update authority set.")]
    NoAuthoritySet,
    #[msg("Invalid amount.")]
    InvalidAmount,
    #[msg("Invalid precision.")]
    InvalidPrecision,
    #[msg("Insufficient balance.")]
    InsufficientBalance,
    #[msg("Zero balance.")]
    ZeroBalance,
}

impl From<CurveError> for AmmError {
    fn from(error: CurveError) -> AmmError {
        match error {
            CurveError::InvalidPrecision => AmmError::InvalidPrecision,
            CurveError::Overflow => AmmError::Overflow,
            CurveError::Underflow => AmmError::Underflow,
            CurveError::InvalidFeeAmount => AmmError::InvalidFee,
            CurveError::InsufficientBalance => AmmError::InsufficientBalance,
            CurveError::ZeroBalance => AmmError::ZeroBalance,
            CurveError::SlippageLimitExceeded => AmmError::SlippageExceeded,
        }
    }
}