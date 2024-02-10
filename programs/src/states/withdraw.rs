use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct WithdrawProfit<'info> {
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
pub struct Withdraw{
    pub withdrawer:Pubkey,
    pub 
}
