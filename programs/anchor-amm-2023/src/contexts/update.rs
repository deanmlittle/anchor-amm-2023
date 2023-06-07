use anchor_lang::prelude::*;
use crate::has_update_authority;
use crate::state::config::Config;
use crate::errors::AmmError;

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        seeds = [
            b"config",
            config.seed.to_le_bytes().as_ref()
        ],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,
    pub system_program: Program<'info, System>
}

impl<'info> Update<'info> {
    pub fn lock(
        &mut self,
    ) -> Result<()> {
        has_update_authority!(self);
        self.config.locked = true;
        Ok(())
    }

    pub fn unlock(
        &mut self,
    ) -> Result<()> {
        has_update_authority!(self);
        self.config.locked = false;
        Ok(())
    }
}



