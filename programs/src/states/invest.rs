use anchor_lang::{
    prelude::*,
    solana_program::{account_info::AccountInfo, program::invoke_signed},
};
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer as SplTransfer};
use crate::{errors::investerr::*, states::initialize::*, states::offerings::*, utils::*};
use spl_token_2022::{
    instruction::{mint_to, transfer, transfer_checked},
    ID as Token2022ProgramId,
};
use std::ops::Div;

#[derive(Accounts)]
#[instruction(offering_id:u32)]
pub struct Invest<'info> {
    #[account(init, payer = signer, space = 8 + 8)]
    pub new_investment: Account<'info, InvestmentAccount>,
    #[account(mut,seeds=[PROJECT_PDA,&offering_id.to_le_bytes()],bump)]
    pub project_pda: Account<'info, ProjectData>,
    #[account(mut,seeds=[OFFERINGS],bump)]
    pub offerings: Account<'info, Offering>,
    #[account(mut,seeds=[INVESTMENT_TOKEN_MINT,offerings.key().as_ref()],bump)]
    pub offerings_vault_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub users_sft_tokenaccount: Account<'info, TokenAccount>,
    #[account(mut)]
    pub users_investment_token_tokenaccount: Account<'info, TokenAccount>,
    #[account(mut)]
    pub offering_sft_token_mint: Account<'info, Mint>,
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub kyc_authorizer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct InvestmentAccount {
    pub investment_id: u32,
    pub investor: Pubkey,
    pub amount_invested: u64,
    pub last_cash_out: u64,
    pub offering_id: u32,
    pub last_withdrawal_date: u64,
    pub investment_date: u64,
}

impl<'info> Invest<'info> {
    pub fn initiate_investment(
        &mut self,
        investment_amount: u64,
        token_decimals: u8,
        ctx: Context<Invest>,
    ) -> Result<()> {
        if self.offering_sft_token_mint.key() != self.offerings.token_mint {
            return Err(InvestError::TokenMintMismatch.into());
        }
        if !self
            .project_pda
            .is_wallet_kyc_wallet(&self.kyc_authorizer.key())
        {
            return Err(InvestError::KYCnotverified.into());
        }
        let receive_investment = transfer_checked(
            &self.token_program.key(),
            &self.signer.key(),
            &self.offerings.investment_token_mint,
            &self.offerings.investment_vault_address,
            &self.offerings.key(),
            &[&self.signer.key()],
            investment_amount,
            token_decimals,
        );
        let mint_sft_to_investor = mint_to(
            &Token2022ProgramId,
            &self.offerings.token_mint,
            &self.users_sft_tokenaccount.key(),
            &self.signer.key(),
            &[&self.signer.key()],
            investment_amount.div(u64::pow(10, token_decimals.into())),
        );

        match receive_investment {
            Ok(TransferInstruction) => match mint_sft_to_investor {
                Ok(MintSFTInstruction) => {
                    invoke_signed(
                        &TransferInstruction,
                        &[
                            self.signer.to_account_info(),
                            self.offerings.to_account_info(),
                            self.system_program.to_account_info(),
                            self.token_program.to_account_info(),
                            self.offerings_vault_token_account.to_account_info().clone(),
                            self.users_investment_token_tokenaccount.to_account_info(),
                        ],
                        &[],
                    );
                    let seeds = &[OFFERINGS, &[*ctx.bumps.get("offerings").unwrap()]];
                    let pda_signer = [&seeds[..]];
                    invoke_signed(
                        &MintSFTInstruction,
                        &[
                            self.signer.to_account_info(),
                            self.offerings.to_account_info(),
                            self.token_program.to_account_info(),
                            self.system_program.to_account_info(),
                            self.offering_sft_token_mint.to_account_info(),
                        ],
                        &pda_signer,
                    );
                    self.offerings.last_investment_id +=1;
                    self.new_investment.investment_id = self.offerings.last_investment_id.clone();
                    self.new_investment.investor = self.signer.key();
                    self.new_investment.amount_invested = investment_amount;
                    self.new_investment.investment_date = Clock::get().unwrap().unix_timestamp;
                    Ok(())
                }
                _ => return Err(InvestError::DepositError.into()),
            },
            _ => return Err(InvestError::DepositError.into()),
        }

        // Ok(())
    }
}
