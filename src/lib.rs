use anchor_lang::prelude::*;

#[program]
mod nft_minter {
    use super::*;

    #[derive(Accounts)]
    pub struct NFTMetadata {
        pub data: Account<Data>,
        pub authority: Signer,
    }

    #[derive(Accounts)]
    pub struct Data<'info> {
        #[account(init, payer = authority, space = 100 + 8 * name.len(), writable)]
        pub nft_account: Account<'info, NFTData>,
        pub authority: Signer<'info>,
        pub payer: Signer<'info>,
        pub system_program: AccountInfo<'info>,
    }

    #[derive(Accounts)]
    pub struct NFTData {
        pub authority: Signer,
        pub metadata: UncheckedAccount,
        pub mint: Account<Mint>,
        #[account(mut)]
        pub data: Account<Buffer>,
        pub rent: Sysvar<'info, Rent>,
        pub system_program: AccountInfo<'info>,
    }

    #[derive(Accounts)]
    pub struct Mint {
        pub mint: Account<'info, Mint>,
        pub authority: Signer<'info>,
    }

    impl<'info> Data<'info> {
        pub fn mint_nft(
            &self,
            ctx: Context<'_, '_, '_, 'info, MintNFT<'info>>,
            name: String,
            uri: String,
        ) -> ProgramResult {
            let mint = ctx.accounts.mint.clone();
            let seeds = &[b"nft", name.as_bytes()];
            let (_, bump) = Pubkey::find_program_address(seeds, &ctx.program_id);
            let nft_metadata_account = &mut ctx.accounts.nft_account;
            let cpi_ctx = CpiContext::new(ctx.accounts.metadata.clone(), (mint, bump));
            nft_metadata::mint(cpi_ctx, name, uri)?;
            nft_metadata_account.mint = *mint.to_account_info().key;
            Ok(())
        }
    }
}

#[derive(Accounts)]
pub struct Metadata<'info> {
    pub authority: Signer<'info>,
    pub payer: Signer<'info>,
    #[account(mut)]
    pub data: Account<'info, MetadataData>,
    pub system_program: AccountInfo<'info>,
}

#[account]
pub struct MetadataData {
    pub authority: Pubkey,
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Accounts)]
pub struct UpdateAuthority<'info> {
    #[account(mut, signer)]
    pub metadata: Account<'info, MetadataData>,
}

impl<'info> Metadata<'info> {
    pub fn mint(
        &self,
        ctx: Context<'_, '_, '_, 'info, Mint>,
        name: String,
        symbol: String,
        uri: String,
    ) -> ProgramResult {
        let data = &mut ctx.accounts.data;
        data.authority = *self.authority.to_account_info().key;
        data.name = name;
        data.symbol = symbol;
        data.uri = uri;
        Ok(())
    }

    pub fn update_authority(
        &self,
        ctx: Context<'_, '_, '_, 'info, UpdateAuthority<'info>>,
        new_authority: Pubkey,
    ) -> ProgramResult {
        let metadata = &mut ctx.accounts.metadata;
        metadata.authority = new_authority;
        Ok(())
    }
}