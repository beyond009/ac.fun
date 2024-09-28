use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod auction {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, end_time: i64, min_bid: u64) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        auction.authority = *ctx.accounts.authority.key;
        auction.end_time = end_time;
        auction.highest_bid = min_bid;
        auction.highest_bidder = Pubkey::default();
        auction.ended = false;
        Ok(())
    }

    pub fn bid(ctx: Context<Bid>, amount: u64) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        let bidder = &ctx.accounts.bidder;

        require!(!auction.ended, ErrorCode::AuctionEnded);
        require!(Clock::get()?.unix_timestamp < auction.end_time, ErrorCode::AuctionEnded);
        require!(amount > auction.highest_bid, ErrorCode::BidTooLow);

        // 退还之前的最高出价
        if auction.highest_bidder != Pubkey::default() {
            // 这里应该实现退还逻辑，为简化示例，这里省略
        }

        // 更新最高出价信息
        auction.highest_bid = amount;
        auction.highest_bidder = *bidder.key;

        Ok(())
    }

    pub fn end_auction(ctx: Context<EndAuction>) -> Result<()> {
        let auction = &mut ctx.accounts.auction;
        let authority = &ctx.accounts.authority;

        require!(!auction.ended, ErrorCode::AuctionAlreadyEnded);
        require!(Clock::get()?.unix_timestamp >= auction.end_time, ErrorCode::AuctionNotEnded);

        auction.ended = true;

        // 这里应该实现将最高出价转给拍卖发起人的逻辑，为简化示例，这里省略

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 8 + 8 + 32 + 1)]
    pub auction: Account<'info, Auction>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Bid<'info> {
    #[account(mut)]
    pub auction: Account<'info, Auction>,
    pub bidder: Signer<'info>,
}

#[derive(Accounts)]
pub struct EndAuction<'info> {
    #[account(mut, has_one = authority)]
    pub auction: Account<'info, Auction>,
    pub authority: Signer<'info>,
}

#[account]
pub struct Auction {
    pub authority: Pubkey,
    pub end_time: i64,
    pub highest_bid: u64,
    pub highest_bidder: Pubkey,
    pub ended: bool,
}

#[error_code]
pub enum ErrorCode {
    #[msg("The auction has already ended")]
    AuctionEnded,
    #[msg("The auction has not ended yet")]
    AuctionNotEnded,
    #[msg("The bid is too low")]
    BidTooLow,
    #[msg("The auction has already been ended")]
    Auct