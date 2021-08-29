use anchor_lang::prelude::*;

#[program]
pub mod solvent_contracts {
    use super::*;

    /*
        This should initialize a bucket. Every bucket will have a program account
        storing the bucket details.
    */

    pub fn initializeBucket(ctx: Context<InitializeBucket>) -> ProgramResult {
        ctx.accounts.bucket.creator = *ctx.accounts.creator.key;
        ctx.accounts.bucket.isFinalized = false;
        Ok(())
    }

    pub fn finalizeBucket(ctx: Context<FinalizeBucket>) -> ProgramResult {
        ctx.accounts.bucket.isFinalized = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeBucket<'info> {
    bucket: ProgramAccount<'info, Bucket>,
    #[account(signer)]
    creator: AccountInfo<'info>
}

#[derive(Accounts)]
pub struct FinalizeBucket<'info> {
    bucket: ProgramAccount<'info, Bucket>,
    #[account(signer, address = bucket.creator)]
    creator: AccountInfo<'info>
}

#[account]
pub struct Bucket {
    creator: Pubkey,
    isFinalized: bool,

}
