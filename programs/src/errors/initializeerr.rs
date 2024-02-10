use anchor_lang::prelude::*;

#[error_code]
pub enum InitializeError {
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
}
