use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(init)]
    pub nft_account: ProgramAccount<'info, NFTAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct NFTAccount {
    pub data: u64, // Example data
}
