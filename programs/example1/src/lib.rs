use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, TokenAccount, Transfer, MintTo};

declare_id!("4Y8PGKfY7q5hxDA17h6UHk5eACo6k9idy1chiHb7HKsp");

#[program]
mod example1 {
    use super::*;

    pub fn create(ctx: Context<Create>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count = 0;
        msg!("Creating account");
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        base_account.count += 1;
        msg!("Incrementing account");
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> ProgramResult{
        

        msg!("Entered deposit");
        msg!("Amount is {}", amount);
        
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.depositor.key(),
            &ctx.accounts.vault.key(),
            amount,
        );
       
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.depositor.to_account_info(),
                ctx.accounts.vault.to_account_info(),
            ],
        ).map_err(|err| println!("{:?}", err)).ok();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 16 + 16)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, has_one = owner)]
    depositor: Account<'info, TokenAccount>,

    #[account(mut, constraint = vault.mint_token == depositor.mint)]
    vault: Account<'info, Vault>,

    #[account(mut, constraint = vault_token.mint == vault.mint_token)]
    vault_token: Account<'info, TokenAccount>,

    #[account(mut)]
    vault_mint: Account<'info, Mint>,

    #[account(mut, constraint = user_vault.mint == vault.vault_mint)]
    user_vault: Account<'info, TokenAccount>,

    #[account(signer)]
    /// CHECK: xyz
    owner: AccountInfo<'info>,

    #[account(signer)]
    /// CHECK: xyz
    refferer: AccountInfo<'info>,
    // /// CHECK: xyz
    // token_program: AccountInfo<'info>,
}

#[account]
pub struct BaseAccount {
    pub count: u64,
}

#[account]
pub struct Vault {
    pub bump: u8,
    pub payer: Pubkey,
    pub mint_token: Pubkey,  // The token this vault keep
    pub vault_token: Pubkey, // PDA for this vault keep the token
    pub vault_mint: Pubkey,  // LP token mint
}