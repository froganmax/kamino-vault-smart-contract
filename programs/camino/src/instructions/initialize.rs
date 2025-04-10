use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK:
    #[account(
        mut,
        seeds = [ b"pool_camino"],
        bump
    )]
    pub camino_pool: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = cb_btc,
        associated_token::authority = camino_pool
    )]
    pub camino_pool_token: Account<'info, TokenAccount>,

    pub cb_btc: Account<'info, Mint>,

    /// CHECK:
    #[account(
        mut,
        seeds = [ b"pool_cb_btc"],
        bump
    )]
    pub cb_btc_pool: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = cb_btc,
        associated_token::authority = cb_btc_pool
    )]
    pub cb_btc_pool_token: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn process(&mut self) -> Result<()> {
        msg!("cbBtc & Camino Pool Initialized ");

        Ok(())
    }
}
