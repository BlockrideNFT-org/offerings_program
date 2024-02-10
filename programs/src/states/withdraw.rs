use anchor_lang::prelude::*;
use anchor_lang::{
    prelude::*,
    solana_program::{account_info::AccountInfo, program::invoke_signed},
};
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::{errors::withdrawalerr::*, states::initialize::*, states::offerings::*, utils::*};
use spl_token::instruction::transfer;
// use std::result::Result;
#[derive(Accounts)]
#[instruction(offering_id:u32)]
pub struct WithdrawProfit<'info> {
    #[account(mut,
     seeds=[PROJECT_PDA],bump)]
    pub project_pda: Box<Account<'info, ProjectData>>,
    #[account(mut,seeds=[OFFERINGS,&offering_id.to_le_bytes()],bump=offering_pda.offering_bump)]
    pub offering_pda: Box<Account<'info, Offering>>,
    #[account(init,seeds=[WITHDRAW,&(offering_pda.last_withdrawal_id + 1).to_le_bytes()],bump,
    space = 8 + std::mem::size_of::<Withdraw>(),payer=signer)]
    pub withdraw: Account<'info, Withdraw>,
    #[account(mut,seeds=[INVESTMENT_TOKEN_MINT,offering_pda.key().as_ref()],bump=offering_pda.offering_token_account_bump)]
    pub offerings_vault_token_account: Account<'info, TokenAccount>,
    pub vault_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub signer_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub kyc_wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Default)]
pub enum WithdrawerRole {
    #[default]
    Investor,
    Admin,
}

#[account]
pub struct Withdraw {
    pub withdrawer: Pubkey,
    pub role: WithdrawerRole,
    pub withdrawal_date: i64,
    pub withdrawn_amount: u64,
    pub bump: u8,
}

impl<'info> WithdrawProfit<'info> {
    pub fn make_withdrawal(
        &mut self,
        withdrawer: WithdrawerRole,
        amount_withdrawn: u64,
        withdrawal_bump:u8,
        offering_bump:u8
    ) -> Result<()> {
        if !self
            .project_pda
            .is_wallet_kyc_wallet(&self.kyc_wallet.key())
        {
            return Err(WithdrawError::KYCnotverified.into());
        }

        // make trransfer from offering to user
        pub enum Res<T> {
            Ok(T),
            Err(ProgramError),
        }
        let send_token = transfer(
            &self.token_program.key(),
            &self.vault_ata.key(),
            &self.signer_ata.key(),
            &self.offering_pda.key(),
            &[&self.offering_pda.key()],
            amount_withdrawn,
        );
        let seeds = &[OFFERINGS, &[offering_bump]];
        let pda_signer = [&seeds[..]];
        match send_token {
            Ok(SendInstruction) => {
                let make_the_transfer = invoke_signed(
                    &SendInstruction,
                    &[
                        self.signer_ata.to_account_info().clone(),
                        self.vault_ata.to_account_info().clone(),
                        self.offering_pda.to_account_info().clone(),
                        self.token_program.to_account_info().clone(),
                    ],
                    &pda_signer,
                );
                match make_the_transfer {
                    Ok(T) => {
                        let withdrawData = Withdraw {
                            withdrawer: self.signer.key(),
                            role: withdrawer,
                            withdrawal_date: Clock::get().unwrap().unix_timestamp,
                            withdrawn_amount: amount_withdrawn,
                            bump: withdrawal_bump,
                        };
                        self.withdraw.set_inner(withdrawData);
                        self.offering_pda.last_withdrawal_id += 1;
                        msg!("Transfer succeeded!");
                        Ok(())
                    }
                    Err(_E) => Err(WithdrawError::Error.into()),
                }
            }
            Err(_E) => Err(WithdrawError::Error.into()),
        }
    }
}
