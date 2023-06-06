use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer, transfer};
use anchor_spl::associated_token::AssociatedToken;
use constant_product_curve::{ConstantProduct, LiquidityPair};
use crate::state::config::Config;
use crate::errors::AmmError;

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    pub mint_x: Box<Account<'info, Mint>>,
    pub mint_y: Box<Account<'info, Mint>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user,
    )]
    pub user_x: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = user,
    )]
    pub user_y: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = auth,
    )]
    pub vault_x: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = auth,
    )]
    pub vault_y: Box<Account<'info, TokenAccount>>,
    /// CHECK: just a pda for signing
    #[account(seeds = [b"auth"], bump)]
    pub auth: UncheckedAccount<'info>,
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [
            b"config",
            config.seed.to_le_bytes().as_ref()
        ],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Swap<'info> {
    pub fn swap(
        &self,
        is_x: bool,
        amount: u64, // Amount of tokens to deposit
        min: u64, // Minimum expected amount in return
        expiration: i64,
    ) -> Result<()> {
        require!(!self.config.locked, AmmError::PoolLocked);
        require!(Clock::get()?.unix_timestamp > expiration, AmmError::OfferExpired);

        let fee_amount = (amount as u128)
            .checked_mul(self.config.fee as u128).ok_or(AmmError::Overflow)?
            .checked_div(10_000).ok_or(AmmError::InvalidFee)? as u64;

        let amount = (amount as u128).checked_sub(fee_amount as u128).ok_or(AmmError::InvalidFee)? as u64;

        let mut curve = match ConstantProduct::init(
            self.vault_x.amount, 
            self.vault_y.amount,
            self.vault_x.amount,
            0, 
            None
        ) {
            Ok(c) => c,
            Err(_) => return err!(AmmError::CurveError)
        };
        
        let token = match is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y
        };

        let res = match curve.swap(token, amount, min) {
            Ok(r) => r,
            Err(_) => return err!(AmmError::CurveError)
        };

        self.deposit_token(is_x, res.deposit)?;
        self.withdraw_token(is_x, res.withdraw)
    }
    
    pub fn deposit_token(
        &self,
        is_x: bool,
        amount: u64
    ) -> Result<()> {
        let (from, to) =  match is_x {
            true => (self.user_x.to_account_info(), self.vault_x.to_account_info()),
            false => (self.user_y.to_account_info(), self.vault_y.to_account_info())
        };

        let accounts = Transfer {
            from,
            to,
            authority: self.user.to_account_info() 
        };

        let ctx = CpiContext::new(
            self.token_program.to_account_info(),
            accounts
        );

        transfer(ctx, amount)
    }
    
    pub fn withdraw_token(
        &self,
        is_x: bool,
        amount: u64
    ) -> Result<()> {
        let (from, to) =  match is_x {
            true => (self.vault_x.to_account_info(), self.user_x.to_account_info()),
            false => (self.vault_y.to_account_info(), self.user_y.to_account_info())
        };

        let accounts = Transfer {
            from,
            to,
            authority: self.auth.to_account_info()
        };

        let seeds = &[
            &b"auth"[..],
            &[self.config.auth_bump],
        ];

        let signer_seeds = &[&seeds[..]];        

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds
        );

        transfer(ctx, amount)
    }
}