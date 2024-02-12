use anchor_lang::prelude::*;
use crate::{errors::initializeerr::*, utils::*};
// use Response::*;

#[derive(Accounts)]
pub struct InitializeBlockride<'info> {
    #[account(init,
     payer = signer,
     seeds=[PROJECT_PDA],bump,
      space = 8 + std::mem::size_of::<ProjectData>())]
    pub ProjectPDA: Box<Account<'info, ProjectData>>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct ProjectData {
    pub kyc_wallet: Pubkey,
    pub authority_wallet: Pubkey,
    pub last_offering_id: u32,
}

impl ProjectData {
    pub fn is_wallet_kyc_wallet(&self, wallet: &Pubkey) -> bool {
        if &self.kyc_wallet != wallet {
            return false;
        }
        return true;
    }

    pub fn is_authority_wallet(&self, wallet: &Pubkey) -> bool {
        if &self.authority_wallet != wallet {
            return false;
        }
        return true;
    }
}

impl<'info> InitializeBlockride<'info> {
    pub fn set_or_update_kyc_wallet(
        &mut self,
        kyc_wallet: Pubkey,
        authority_wallet: Pubkey,
    ) -> Result<String> {
        let null_pubkey = Pubkey::default();
        if self.ProjectPDA.authority_wallet != null_pubkey
            && !self.ProjectPDA.is_authority_wallet(&self.signer.key())
        {
            return Err(InitializeError::Unauthorized.into());
        }

        self.ProjectPDA.kyc_wallet = kyc_wallet;
        self.ProjectPDA.authority_wallet = authority_wallet;
        msg!("Program initiated!!!"); // Message will show up in the tx logs
        Ok("Program inititated".to_owned())
    }
}
