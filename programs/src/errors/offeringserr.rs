use anchor_lang::prelude::*;

#[error_code]
pub enum OfferingError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Not allowed")]
    NotAllowed,
    #[msg("automatic yield cannot be set to true while yield earning started is set to true")]
    IncorrectYieldStartPeriod,
    #[msg("Yield earnings cannot start at offering creation")]
    YeildEarningCannotBeStarted,
    #[msg("Error")]
    NoError,
}
