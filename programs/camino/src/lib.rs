use anchor_lang::prelude::*;

pub mod instructions;

use crate::instructions::*;

declare_id!("E3UTHzf4Dc1pcu5fBZGN9WashSnbrVK1ht9tzQL2m3pL");

#[program]
pub mod camino {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let _ = ctx.accounts.process();
        Ok(())
    }

    pub fn deposit_to_cb_btc(ctx: Context<DepositToCbBtc>, amount: u64) -> Result<()> {
        let _ = ctx.accounts.process(amount);
        Ok(())
    }

    pub fn deposit_cb_btc_to_camino(ctx: Context<DepositCbBtcToCamino>, amount: u64) -> Result<()> {
        let _ = ctx.accounts.process(amount, ctx.bumps.cb_btc_pool);
        Ok(())
    }

    pub fn withdraw_from_cb_btc(ctx: Context<WithdrawFromCbBtc>, amount: u64) -> Result<()> {
        let _ = ctx.accounts.process(amount, ctx.bumps.cb_btc_pool);
        Ok(())
    }

    pub fn withdraw_camino_to_cb_btc(
        ctx: Context<WithdrawCaminoToCbBtc>,
        amount: u64,
    ) -> Result<()> {
        let _ = ctx.accounts.process(amount, ctx.bumps.camino_pool);
        Ok(())
    }
}
