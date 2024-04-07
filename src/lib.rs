use anchor_lang::prelude::*;
use anchor_lang::solana_program;

declare_id!("YOUR_PROGRAM_ID");

#[program]
mod nft_minter {
    use super::*;
    
    pub fn mint_nft(ctx: Context<MintNFT>, data: u64) -> ProgramResult {
        Ok(())
    }

}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(init)]
    pub nft_account: ProgramAccount<'info, NFTAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

// NFTAccount data structure
#[account]
pub struct NFTAccount {
    pub data: u64, // Example data
}
