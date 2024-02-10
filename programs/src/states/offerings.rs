use anchor_lang::prelude::*;
// pub mod initialize;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::{errors::offeringserr::*, states::initialize::*, utils::*};

// #[instructions(count)]
#[derive(Accounts)]
pub struct Offerings<'info> {
    #[account(init, payer = signer,
    seeds=[OFFERINGS,&(projectPDA.last_offering_id + 1).to_le_bytes()],bump,
     space = 8 + std::mem::size_of::<Offering>())]
    pub new_offerings: Box<Account<'info, Offering>>,
    #[account(init,seeds=[INVESTMENT_TOKEN_MINT,new_offerings.key().as_ref()],bump,payer=signer,token::mint=investment_token_mint,token::authority=new_offerings)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut,seeds=[PROJECT_PDA],bump=projectPDA.bump)]
    pub projectPDA: Box<Account<'info, ProjectData>>,
    pub investment_token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct Offering {
    pub offering_id: u32,
    pub total_amount_to_raise: u64, //Total amount expected to be raised in this round
    pub total_amount_to_raised: u64, // Total amount raised in this round
    pub yield_earnings_started: bool, // Has yield earning started?..boolean(true or false)
    pub automatically_start_yield_period_after_completed_fund_rase: bool, // To automatically start counting yield for users after period of fund raising is completed
    pub investment_ended: bool,   // Ended investment period for investors
    pub investment_Started: bool, // Investment period started for investors
    pub offering_name: String,    // The name of this particular offering
    pub authority_wallet: Pubkey, // The wallet that has authority to this offering
    pub investment_start_date: i64, //The date the offering started
    pub last_investment_id: u32,
    pub investment_token_mint: Pubkey, // Eg. USDC, SOL
    pub investment_vault_address: Pubkey,
    pub last_withdrawal_id: u32,
    pub offering_bump: u8,
    pub offering_token_account_bump: u8,
}

impl<'info> Offerings<'info> {
    pub fn initialize_offering(
        &mut self,
        amount_to_raise: u64,
        yield_earnings_started: bool,
        automatically_start_yield_period_after_completed_fund_rase: bool,
        offering_name: String,
        authority_wallet: Pubkey,
        investment_Started: bool,
        total_earning_perc: u32,
        withdrawal_length: u64,
        new_offerings_bumps:u8,
        token_account_bumps:u8
    ) -> Result<String> {
        //Confirm that the creator of offering has authority to create an offering
        if &self.signer.key() != &self.projectPDA.authority_wallet {
            return Err(OfferingError::Unauthorized.into());
        }
        if automatically_start_yield_period_after_completed_fund_rase == true
            && yield_earnings_started == true
        {
            return Err(OfferingError::IncorrectYieldStartPeriod.into());
        }
        if yield_earnings_started == true {
            return Err(OfferingError::YeildEarningCannotBeStarted.into());
        }

        let new_offering_create = &mut self.new_offerings;
        if investment_Started == true {
            new_offering_create.investment_start_date = Clock::get().unwrap().unix_timestamp;
        }
        new_offering_create.total_amount_to_raise = amount_to_raise;
        new_offering_create.yield_earnings_started = yield_earnings_started;
        new_offering_create.total_amount_to_raised = 0;
        new_offering_create.automatically_start_yield_period_after_completed_fund_rase =
            automatically_start_yield_period_after_completed_fund_rase;
        new_offering_create.investment_ended = false;
        new_offering_create.investment_Started = investment_Started;
        new_offering_create.offering_name = offering_name;
        new_offering_create.authority_wallet = authority_wallet;
        new_offering_create.investment_token_mint = self.investment_token_mint.key().clone();
        self.projectPDA.last_offering_id += 1;
        new_offering_create.offering_id = self.projectPDA.last_offering_id.clone();
        new_offering_create.investment_vault_address = self.token_account.key().clone();
        new_offering_create.offering_bump = new_offerings_bumps;
        new_offering_create.offering_token_account_bump = token_account_bumps;
        Ok("Offering created".to_owned())
    }
    pub fn toggle_start_investment(&mut self) -> Result<()> {
        self.new_offerings.investment_Started = !self.new_offerings.investment_Started;
        Ok(())
    }
    pub fn toggle_end_investment(&mut self) -> Result<()> {
        self.new_offerings.investment_ended = !self.new_offerings.investment_ended;
        Ok(())
    }
}
