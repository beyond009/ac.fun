use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod price_calculator {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn calculate_price(ctx: Context<CalculatePrice>, x: u64) -> Result<()> {
        let price_account = &mut ctx.accounts.price_account;
        price_account.price = calculate_price_internal(x);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct CalculatePrice<'info> {
    #[account(init_if_needed, payer = user, space = 8 + 8)]
    pub price_account: Account<'info, PriceAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct PriceAccount {
    pub price: u64,
}

fn calculate_price_internal(x: u64) -> u64 {
    if x < 50 {
        (x as f64 * 0.1) as u64
    } else if x < 150 {
        (50.0 * 0.1 + (x as f64 - 50.0) * 0.5) as u64
    } else {
        (10.0 * 2.0f64.powf((x as f64 - 150.0) / 100.0) + 45.0) as u64
    }
}