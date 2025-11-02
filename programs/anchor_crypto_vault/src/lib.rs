use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("22222222222222222222222222222222222222222222");

#[program]
pub mod anchor_crypto_vault {
    use super::*;

    pub fn deposit(ctx: Context<VaultAction>, amount: u64) -> Result<()> {
        //check if the vault is empty

        require!(
            ctx.accounts.vault.lamports() == 0,
            VaultError::VaultAlreadyExists
        );

        // Ensure amount exceeds rent-exempt minimum
        require!(
            amount > Rent::get()?.minimum_balance(0),
            VaultError::InvalidAmount
        );

        //CPI to transfer the amount from the signer to the vault
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.signer.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount)?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<VaultAction>) -> Result<()> {
        //check if the vault is not empty
        require!(
            ctx.accounts.vault.lamports() > 0,
            VaultError::InsufficientBalance
        );

        //Create PDA signer seeds
        let signer_key = ctx.accounts.signer.key();
        let signer_seeds = &[b"vault", signer_key.as_ref(), &[ctx.bumps.vault]];

        //Transfer the amount from the vault to the signer
        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.signer.to_account_info(),
        };
        let binding = &[&signer_seeds[..]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, binding);

        transfer(cpi_ctx, ctx.accounts.vault.lamports())?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct VaultAction<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum VaultError {
    #[msg("Insufficient balance")]
    InsufficientBalance,

    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Vault already exists")]
    VaultAlreadyExists,
}
