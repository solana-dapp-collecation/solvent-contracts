use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount};
use anchor_lang::Key;
use anchor_lang::solana_program::system_program;

const AUTHORITY_SEED : &str = "authorityseed";

#[program]
pub mod solvent_contracts {
    use super::*;

    /*
        This should initialize a bucket. Every bucket will have a program account
        storing the bucket details.
    */

    pub fn initializeBucket(ctx: Context<InitializeBucket>, _mint_bump: u8, _bucket_authority_bump : u8) -> ProgramResult {
        ctx.accounts.bucket.creator = *ctx.accounts.creator.key;
        ctx.accounts.bucket.isFinalized = false;
        Ok(())
    }

    pub fn finalizeBucket(ctx: Context<FinalizeBucket>) -> ProgramResult {
        ctx.accounts.bucket.isFinalized = true;
        Ok(())
    }

    pub fn whitelistToken(ctx: Context<WhitelistToken>) -> ProgramResult {
        ctx.accounts.mint_state.whitelisted = true;
        Ok(())
    }

    pub fn addToBucket(ctx: Context<AddToBucket>, _bucket_authority_bump: u8) -> ProgramResult {

        // Transfer NFT to bucket
        let nft_transfer_ctx = CpiContext::new(
            ctx.accounts.token_program.clone(),
            token::Transfer {
                from: ctx.accounts.user_nft_wallet.to_account_info().clone(),
                to: ctx.accounts.nft_bucket_vault.to_account_info().clone(),
                authority: ctx.accounts.authority.clone(),
            },
        );
        token::transfer(nft_transfer_ctx, 1)?;

        // TODO - Add sign with seeds
        // Mint the droplets into the user's token account
        let seeds = &[AUTHORITY_SEED.as_bytes(), &[_bucket_authority_bump]];
        let signer_seeds = &[&seeds[..]];

        let mint_to_user_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_program.clone(),
            token::MintTo {
                mint: ctx.accounts.droplets_mint.to_account_info().clone(),
                to: ctx.accounts.user_droplet_account.to_account_info().clone(),
                authority: ctx.accounts.bucket_authority.to_account_info().clone(),
            },
            signer_seeds
        );
        token::mint_to(mint_to_user_ctx, 1)?;
        Ok(())
    }

    pub fn redeem(ctx: Context<Redeem>, _bucket_authority_bump: u8) -> ProgramResult {
        // TODO - check how to burn the droplets
        // token::burn();

        let seeds = &[AUTHORITY_SEED.as_bytes(), &[_bucket_authority_bump]];
        let signer_seeds = &[&seeds[..]];

        // Transfer coin tokens to vault
        let nft_transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.clone(),
            token::Transfer {
                from: ctx.accounts.nft_bucket_vault.to_account_info().clone(),
                to: ctx.accounts.user_nft_wallet.to_account_info().clone(),
                authority: ctx.accounts.bucket_authority.clone(),
            },
            signer_seeds
        );
        token::transfer(nft_transfer_ctx, 1)?;
        
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction( _mint_bump: u8, _bucket_authority_bump: u8)]
pub struct InitializeBucket<'info> {
    #[account(init)]
    bucket: ProgramAccount<'info, Bucket>,
    #[account(signer)]
    creator: AccountInfo<'info>,

    // Common bucket authority for all markets, no namespacing
    #[account(
        seeds = [AUTHORITY_SEED.as_bytes()],
        bump = _bucket_authority_bump,
    )]
    pub bucket_authority: AccountInfo<'info>,

    // #[account(
    //     init,
    //     mint::decimals = 8,
    //     mint::authority = bucket_authority,
    //     seeds = [
    //         // Namespace by market
    //         bucket.key().as_ref(),
    //     ],
    //     bump = _mint_bump,
    //     payer = creator,
    //     space = Mint::LEN
    // )]
    pub droplets_mint: CpiAccount<'info, Mint>,

    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,

    #[account(address = system_program::ID)]
    pub system_program: AccountInfo<'info>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct FinalizeBucket<'info> {
    #[account(mut)]
    bucket: ProgramAccount<'info, Bucket>,
    #[account(signer, address = bucket.creator)]
    creator: AccountInfo<'info>
}

#[derive(Accounts)]
pub struct WhitelistToken<'info> {
    bucket: ProgramAccount<'info, Bucket>,
    #[account(signer, address = bucket.creator)]
    creator: AccountInfo<'info>,
    nft_mint: CpiAccount<'info, Mint>,

    // Associated with the bucket and NFT mint
    #[account(init)]
    mint_state: ProgramAccount<'info, MintState>
}

#[derive(Accounts)]
pub struct AddToBucket<'info> {
    pub bucket: ProgramAccount<'info, Bucket>,
    pub nft_mint: CpiAccount<'info, Mint>,
    pub mint_state: ProgramAccount<'info, MintState>,

    // Account representing user's NFT account
    pub user_nft_wallet: CpiAccount<'info, TokenAccount>,

    // Account representing a bucket for the collection
    pub nft_bucket_vault: CpiAccount<'info, TokenAccount>,

    #[account(signer)]
    pub authority: AccountInfo<'info>,

    // The Droplet token account associated with a bucket
    pub droplets_mint: CpiAccount<'info, Mint>,

    // The user's Droplet account where it's droplets will be minted in
    // exchange of NFT deposits.
    pub user_droplet_account: CpiAccount<'info, TokenAccount>,
    pub bucket_authority: AccountInfo<'info>,

    #[account(address = token::ID)]
    pub token_program: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Redeem<'info> {
    user_droplet_account: CpiAccount<'info, TokenAccount>,
    pub user_nft_wallet: CpiAccount<'info, TokenAccount>,
    pub nft_bucket_vault: CpiAccount<'info, TokenAccount>,
    pub bucket_authority: AccountInfo<'info>,
    pub token_program: AccountInfo<'info>,
    #[account(signer)]
    pub user: AccountInfo<'info>,

}

#[account]
pub struct Bucket {
    creator: Pubkey,
    isFinalized: bool,

}

#[account]
pub struct MintState {
    whitelisted: bool,
}