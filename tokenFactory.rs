use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use std::mem::size_of;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod token_factory {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let factory = &mut ctx.accounts.factory;
        factory.authority = *ctx.accounts.authority.key;
        factory.token_count = 0;
        Ok(())
    }

    pub fn create_token(
        ctx: Context<CreateToken>,
        name: String,
        symbol: String,
        decimals: u8,
    ) -> Result<()> {
        let factory = &mut ctx.accounts.factory;
        let token_info = &mut ctx.accounts.token_info;

        token_info.mint = ctx.accounts.mint.key();
        token_info.name = name;
        token_info.symbol = symbol;
        token_info.decimals = decimals;
        token_info.authority = *ctx.accounts.authority.key;

        factory.token_count += 1;

        Ok(())
    }

    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u64) -> Result<()> {
        token::mint_to(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                token::MintTo {
                    mint: ctx.accounts.mint.to_account_info(),
                    to: ctx.accounts.token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + size_of::<TokenFactory>())]
    pub factory: Account<'info, TokenFactory>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub factory: Account<'info, TokenFactory>,
    #[account(
        init,
        payer = authority,
        space = 8 + size_of::<TokenInfo>(),
        seeds = [b"token-info", factory.token_count.to_le_bytes().as_ref()],
        bump
    )]
    pub token_info: Account<'info, TokenInfo>,
    #[account(
        init,
        payer = authority,
        mint::decimals = 9,
        mint::authority = authority.key(),
    )]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct TokenFactory {
    pub authority: Pubkey,
    pub token_count: u64,
}

#[account]
pub struct TokenInfo {
    pub mint: Pubkey,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub authority: Pubkey,
}