use anchor_lang::prelude::*;

pub mod errors;
pub mod states;
pub mod utils;
use crate::{states::initialize::*, states::offerings::*, utils::*};

// pub mod initialize;
// pub mod invest;
// pub mod offerings;

// use states;
// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("J4we6VGaF67HJt7v21Y63eQPaAUY57RnQwqnwHSgWufr");

#[program]
mod BlockrideProgram {
    use super::*;
    pub fn initialize(
        ctx: Context<InitializeBlockride>,
        kyc_wallet: Pubkey,
        authority_wallet: Pubkey,
    ) -> Result<String> {
        let init = ctx
            .accounts
            .set_or_update_kyc_wallet(kyc_wallet, authority_wallet);

        match init {
            Ok(t) => {
                msg!("Program inintiated!!!");
                return Ok(t);
            }
            Err(_e) => {
                msg!("An error occred while inititing this program!!!");
                // let errMsgId = Response::Err(())
                // return err!("An error occured");
                return Err(_e);
            }
        }

    }
    pub fn create_offering(
        ctx: Context<Offerings>,
        amount_to_raise: u64,
        earnings_duration: u64,
        yield_earnings_started: bool,
        automatically_start_yield_period_after_completed_fund_rase: bool,
        offering_name: String,
        authority_wallet: Pubkey,
        investment_Started: bool,
        token_mint: Pubkey,
        total_earning_perc: u32,
        withdrawal_length: u64,
    ) -> Result<String> {
        let init = ctx.accounts.initialize_offering(
            amount_to_raise,
            earnings_duration,
            yield_earnings_started,
            automatically_start_yield_period_after_completed_fund_rase,
            offering_name,
            authority_wallet,
            investment_Started,
            token_mint,
            total_earning_perc,
            withdrawal_length,
        );
   match init {
            Ok(t) => {
                msg!("Program inintiated!!!");
                return Ok(t);
            }
            Err(_e) => {
                msg!("An error occred while inititing this program!!!");
                // let errMsgId = Response::Err(())
                // return err!("An error occured");
                return Err(_e);
            }
        }
    }
}
