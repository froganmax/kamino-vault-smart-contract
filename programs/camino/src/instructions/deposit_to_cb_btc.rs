use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

#[derive(Accounts)]
pub struct DepositToCbBtc<'info> {
    pub cb_btc: Account<'info, Mint>,
    /// CHECK:
    #[account(
        mut,
        seeds = [ b"pool_cb_btc"],
        bump
    )]
    pub cb_btc_pool: AccountInfo<'info>,

    #[account(
        mut,
        associated_token::mint = cb_btc,
        associated_token::authority = cb_btc_pool
    )]
    pub cb_btc_pool_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    #[account(
        mut,
        associated_token::mint = cb_btc,
        associated_token::authority = payer
    )]
    pub payer_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> DepositToCbBtc<'info> {
    pub fn process(&mut self, amount: u64) -> Result<()> {
        let transfer_instruction = Transfer {
            from: self.payer_ata.to_account_info(),
            to: self.cb_btc_pool_token.to_account_info(),
            authority: self.payer.to_account_info(),
        };
        // Execute the transfer
        token::transfer(
            CpiContext::new(self.token_program.to_account_info(), transfer_instruction),
            amount, // The amount to transfer (in tokens, not lamports),
        )?;

        Ok(())
    }
}
