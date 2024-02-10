use anchor_lang::prelude::*;
// pub mod errors;
// use crate::{errors::initializeerr::*, errors::offeringserr::*};
#[constant]
pub const PROJECT_PDA: &[u8] = b"BLOCKRIDE_SYSTEM";

#[constant]
pub const OFFERINGS: &[u8] = b"BLOCKRIDE_OFFERING_SYSTEM";

#[constant]
pub const WITHDRAW: &[u8] = b"BLOCKRIDE_WITHDRAW_SYSTEM";

#[constant]
pub const INVESTMENT_TOKEN_MINT: &[u8] = b"INVESTMENT_TOKEN_MINT";

#[constant]
pub const INVEST_SEED: &[u8] = b"INVEST_SEED";

// #[derive(Copy, Clone)]
// pub enum Response {
//     Ok(String),
//     Err(u32),
// }
// impl Response {
//     pub fn get_error(&self) -> Error {
//         match *self {
//             Response::Err(err_id) => match err_id {
//                 0 => OfferingError::Unauthorized.into(),
//                 1 => OfferingError::IncorrectYieldStartPeriod.into(),
//                 2 => OfferingError::YeildEarningCannotBeStarted.into(),
//                 _ => OfferingError::Unauthorized.into(),
//             },
//             _ => OfferingError::NoError.into(),
//         }
//     }
// }
// pub enum OfferingResponse {
//     Ok(String),
//     Err(Error),
// }
