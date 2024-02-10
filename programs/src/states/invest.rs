use anchor_lang::{prelude::*, solana_program::account_info::AccountInfo};
use anchor_spl::{
    associated_token::get_associated_token_address,
    token::{transfer, Token, TokenAccount, Transfer as SplTransfer},
};
use crate::{errors::investerr::*, states::initialize::*, states::offerings::*, utils::*};

#[derive(Accounts)]
#[instruction(offering_id:u32)]
pub struct Invest<'info> {
    #[account(init, payer = signer,seeds=[INVEST_SEED], space = 8 + 8,bump)]
    pub new_investment: Account<'info, InvestmentAccount>,
    #[account(mut,seeds=[PROJECT_PDA],bump=project_pda.bump)]
    pub project_pda: Account<'info, ProjectData>,
    #[account(mut,seeds=[OFFERINGS,&offering_id.to_le_bytes()],bump=offerings.offering_bump)]
    pub offerings: Account<'info, Offering>,
    #[account(mut,seeds=[INVESTMENT_TOKEN_MINT,offerings.key().as_ref()],bump=offerings.offering_token_account_bump)]
    pub offerings_vault_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub users_investment_token_tokenaccount: Account<'info, TokenAccount>,
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
    pub offering_id: u32,
    pub last_withdrawal_date: i64,
    pub investment_date: i64,
    pub bump: u8,
}

impl<'info> Invest<'info> {
    pub fn initiate_investment(
        &mut self,
        investment_amount: u64,
        token_decimals: u8,
        new_investment_bump: u8,
    ) -> Result<()> {
        if self.offerings.investment_ended {
            return Err(InvestError::InvestmentEnded.into());
        }
        if !self.offerings.investment_Started {
            return Err(InvestError::InvestmentHasNotStarted.into());
        }

        if !self
            .project_pda
            .is_wallet_kyc_wallet(&self.kyc_authorizer.key())
        {
            return Err(InvestError::KYCnotverified.into());
        }
        let signera_ata = get_associated_token_address(
            &self.signer.key(),
            &self.offerings.investment_vault_address,
        );
        //--------------Transfer investment amount from user to offering vault
        let cpi_accounts = SplTransfer {
            from: self.signer.to_account_info().clone(),
            to: self.offerings_vault_token_account.to_account_info().clone(),
            authority: self.signer.to_account_info().clone(),
        };
        let cpi_program = self.token_program.to_account_info();
        let send_token = transfer(
            CpiContext::new(cpi_program, cpi_accounts),
            investment_amount,
        );
        match send_token {
            Ok(T) => {
                let investData = InvestmentAccount {
                    amount_invested: investment_amount,
                    investment_date: Clock::get().unwrap().unix_timestamp,
                    investment_id: self.offerings.last_investment_id + 1,
                    investor: self.signer.key(),
                    last_withdrawal_date: Clock::get().unwrap().unix_timestamp,
                    offering_id: self.offerings.offering_id,
                    bump: new_investment_bump,
                };
                self.new_investment.set_inner(investData);
                Ok(())
            }
            Err(E) => Err(InvestError::Error.into()),
        }
        // Ok(())
    }
}
