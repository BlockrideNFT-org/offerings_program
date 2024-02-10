use anchor_lang::prelude::*;

#[error_code]
pub enum InvestError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("KYC not verified.")]
    KYCnotverified,
    #[msg("Not allowed")]
    NotAllowed,
    #[msg("An error occured while transfering invesment funds from user")]
    DepositError,
    #[msg("Error")]
    NoError,
    #[msg("Provided token mint is not the token mint associated with this offering")]
    TokenMintMismatch,
}
