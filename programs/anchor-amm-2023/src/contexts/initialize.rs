use std::collections::BTreeMap;

use anchor_lang::{prelude::*};
use anchor_spl::token::{Mint, Token, TokenAccount};
use anchor_spl::associated_token::AssociatedToken;
use crate::errors::AmmError;
use crate::state::config::Config;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
        init,
        seeds = [b"lp", config.key.as_ref()],
        payer = initializer,
        bump,
        mint::decimals = 6,
        mint::authority = auth
    )]
    pub mint_lp: Account<'info, Mint>,
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_x,
        associated_token::authority = auth,
    )]
    pub vault_x: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_y,
        associated_token::authority = auth,
    )]
    pub vault_y: Account<'info, TokenAccount>,
    /// CHECK: This is safe because it's just used to sign
    #[account(seeds = [b"auth"], bump)]
    pub auth: UncheckedAccount<'info>,
    #[account(
        init, 
        payer = initializer, 
        seeds = [b"config", seed.to_le_bytes().as_ref()], 
        bump,
        space = Config::LEN
    )]
    pub config: Account<'info, Config>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>
}

impl<'info> Initialize<'info> {
    pub fn init(
        &mut self,
        bumps: &BTreeMap<String, u8>,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>        
    ) -> Result<()> {
        // Don't charge >100.00% as a fee
        require!(fee <= 10000, AmmError::InvalidFee);

        let (auth_bump, config_bump, lp_bump) = (
            *bumps.get("auth").ok_or(AmmError::BumpError)?,
            *bumps.get("config").ok_or(AmmError::BumpError)?,
            *bumps.get("mint_lp").ok_or(AmmError::BumpError)?
        );
        self.config.init(
            seed,
            authority,
            self.mint_x.key(),
            self.mint_y.key(),
            fee,
            auth_bump,
            config_bump,
            lp_bump
        );
        Ok(())
    }
}