use anchor_lang::prelude::*;

declare_id!("CwycY9HgbcMHiEsQ4QirrzmM4nJtzQSWK8QBFwFC7dY2");

#[program]
pub mod anchor_crypto_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
