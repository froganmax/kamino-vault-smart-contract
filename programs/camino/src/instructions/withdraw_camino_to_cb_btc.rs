use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct WithdrawCaminoToCbBtc<'info> {
    /// CHECK:
    #[account(
        mut,
        seeds = [ b"pool_camino"],
        bump
    )]
    pub camino_pool: AccountInfo<'info>,

    #[account(
        mut,
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
        mut,
        associated_token::mint = cb_btc,
        associated_token::authority = cb_btc_pool
    )]
    pub cb_btc_pool_token: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,

    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawCaminoToCbBtc<'info> {
    pub fn process(&mut self, amount: u64, bump: u8) -> Result<()> {
        let transfer_instruction = Transfer {
            from: self.camino_pool_token.to_account_info(),
            to: self.cb_btc_pool_token.to_account_info(),
            authority: self.camino_pool.to_account_info(),
        };

        token::transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                transfer_instruction,
                &[&[
                    b"pool_camino",
                    &[bump], // Constant seed
                ]],
            ),
            amount, // The amount to transfer (in tokens, not lamports),
        )?;

        Ok(())
    }
}
