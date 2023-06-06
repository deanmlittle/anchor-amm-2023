use crate::constants::*;
use anchor_lang::prelude::*;

#[account]
pub struct Config {
    pub seed:u64,
    pub authority: Option<Pubkey>,
    pub mint_x: Pubkey,           // Token X Mint
    pub mint_y: Pubkey,           // Token Y Mint
    pub fee: u16,                 // Swap fee in basis points
    pub locked: bool,
    pub auth_bump: u8,
    pub config_bump: u8,
    pub lp_bump: u8
}

impl Config {
    pub fn init(
        &mut self, 
        seed: u64, 
        authority: Option<Pubkey>, 
        mint_x: Pubkey,
        mint_y: Pubkey,
        fee: u16,
        auth_bump: u8,
        config_bump: u8,
        lp_bump: u8
    ) -> Result<()> {
        self.seed = seed;
        self.authority = authority;
        self.mint_x = mint_x;
        self.mint_y = mint_y;
        self.fee = fee;
        self.locked = false;
        self.auth_bump = auth_bump;
        self.config_bump = config_bump;
        self.lp_bump = lp_bump;
        Ok(())
    }

    pub const LEN: usize = 8 + U64_L + OPTION_L + PUBKEY_L * 3 + U16_L + BOOL_L + U8_L * 3;
}