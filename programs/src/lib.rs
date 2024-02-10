use anchor_lang::prelude::*;

pub mod errors;
pub mod states;
pub mod utils;

use crate::{
    states::{initialize::*, invest::*, offerings::*, withdraw::*},
    utils::*,
};

// pub mod initialize;
// pub mod invest;
// pub mod offerings;

// use states;
// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("4dS2nKMvJ2rCidNSZXbx4et3UR3B1TS8r6x5tr9SETbR");

#[program]
mod BlockrideProgram {
    use super::*;
    pub fn initialize(
        ctx: Context<InitializeBlockride>,
        kyc_wallet: Pubkey,
        authority_wallet: Pubkey,
    ) -> Result<String> {
        let ProjectPDA_bump = ctx.bumps.ProjectPDA.clone();
        let init = ctx
            .accounts
            .set_or_update_kyc_wallet(kyc_wallet, authority_wallet, ProjectPDA_bump);

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
        yield_earnings_started: bool,
        automatically_start_yield_period_after_completed_fund_rase: bool,
        offering_name: String,
        authority_wallet: Pubkey,
        investment_Started: bool,
        total_earning_perc: u32,
        withdrawal_length: u64,
    ) -> Result<String> {
        let new_offerings_bumps = ctx.bumps.new_offerings.clone();
        let token_account_bumps= ctx.bumps.token_account.clone();
        let init = ctx.accounts.initialize_offering(
            amount_to_raise,
            yield_earnings_started,
            automatically_start_yield_period_after_completed_fund_rase,
            offering_name,
            authority_wallet,
            investment_Started,
            total_earning_perc,
            withdrawal_length,
            new_offerings_bumps,
            token_account_bumps
        );
        match init {
            Result::Ok(t) => {
                msg!("Program inintiated!!!");
                return Ok(t);
            }
            Result::Err(_e) => {
                msg!("An error occred while inititing this program!!!");
                // let errMsgId = Response::Err(())
                // return err!("An error occured");
                return Err(_e);
            }
        }
    }

    pub fn toggle_start_investment(ctx: Context<Offerings>) -> Result<()> {
        let kickstart = ctx.accounts.toggle_start_investment();
        match kickstart {
            Result::Ok(t) => {
                msg!("investment start toggled!!!");
                return Ok(t);
            }
            Result::Err(_e) => {
                msg!("An error occred while toggling start investment!!!");
                // let errMsgId = Response::Err(())
                // return err!("An error occured");
                return Err(_e);
            }
        }
    }
    pub fn toggle_end_investment(ctx: Context<Offerings>) -> Result<()> {
        let endinvesment = ctx.accounts.toggle_end_investment();
        match endinvesment {
            Result::Ok(t) => {
                msg!("investment end toggled!!!");
                return Ok(t);
            }
            Result::Err(_e) => {
                msg!("An error occred while toggling end investment!!!");
                return Err(_e);
            }
        }
    }

    pub fn withdraw_from_pool(
        ctx: Context<WithdrawProfit>,
        withdrawer: WithdrawerRole,
        amount_withdrawn: u64,
    ) -> Result<()> {
        let withdrawal_bump = ctx.bumps.withdraw.clone();
        let offering_bump = ctx.accounts.offering_pda.offering_bump.clone();
        let make_withdrawal = ctx
            .accounts
            .make_withdrawal(withdrawer, amount_withdrawn, withdrawal_bump,offering_bump);
        match make_withdrawal {
            Result::Ok(t) => {
                msg!("Withdrawal completed!!!");
                return Ok(t);
            }
            Result::Err(_e) => {
                msg!("An error occred while making withdrawal!!!");
                // let errMsgId = Response::Err(())
                // return err!("An error occured");
                return Err(_e);
            }
        }
    }

    pub fn make_investment(
        ctx:  Context<Invest>,
        investment_amount: u64,
        token_decimals: u8,
    ) -> Result<()> {
        let new_investment_bump = ctx.bumps.new_investment.clone();
        let make_investment =
            ctx.accounts
                .initiate_investment(investment_amount, token_decimals, new_investment_bump);
        match make_investment {
            Result::Ok(t) => {
                msg!("investment withdrawal!!!");
                return Ok(t);
            }
            Result::Err(_e) => {
                msg!("An error occred while making investment!!!");
                // let errMsgId = Response::Err(())
                // return err!("An error occured");
                return Err(_e);
            }
        }
    }
}
