use anchor_lang::prelude::*;

#[error_code]
pub enum WithdrawError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("KYC not verified.")]
    KYCnotverified,
    #[msg("Not allowed")]
    NotAllowed,
    #[msg("An error occured while attempting withdrawal from vault")]
    WithdrawalError,
    #[msg("An Error occured")]
    Error,
    #[msg("Provided token mint is not the token mint associated with this offering")]
    TokenMintMismatch,
}
