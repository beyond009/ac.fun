# AC.fun - Community-Driven Meme Token Auction Platform

AC.fun is a decentralized, community-driven platform for launching and pricing Meme tokens using a dynamic auction mechanism. The platform allows both individual and unionized bids through guild governance, ensuring fair and transparent pricing for each token. AC.fun aims to redefine the Meme token market by using flexible bidding curves and smart contract-based governance.

## Features

- **Dynamic Auction Mechanism**: AC.fun uses a progressive English auction system with a segmented bidding curve that adapts to different bidding stages, ensuring optimal pricing for each token.
- **Guild-Based Bidding**: Users can join guilds or bid individually. Guilds are community-driven entities that pool resources and strategies to increase their chances of winning auctions.
- **Fair and Transparent**: All bidding processes are governed by smart contracts, ensuring transparency and preventing malicious manipulation.
- **Flexible Bidding Curve**: The pricing curve dynamically adjusts across different phases, encouraging early participation and preventing late-stage price manipulation.
- **Smart Contract Governance**: Joining guilds and managing auctions are all handled through decentralized smart contracts, ensuring trust and fairness.

## How It Works

1. **Auction Launch**: Each Meme token auction is launched with an initial price set by the smart contract. The auction follows a dynamic pricing curve.
   
2. **Bidding**: Participants can place bids either individually or through guilds. Each bid must be higher than the current highest bid, following the curve's pricing mechanism.
   
3. **Dynamic Pricing Curve**:
   - **Phase 1**: When the bid count is below 50, the price increases slowly at a rate of 0.1 per bid.
   - **Phase 2**: Between 50 and 150 bids, the price increases at a faster rate of 0.5 per bid.
   - **Phase 3**: After 150 bids, the price grows exponentially, deterring frequent small bids and encouraging final price determination.

4. **Auction Closure**: The auction ends when the preset time or price cap is reached. The highest bidder wins the auction, and the funds are used to provide liquidity for the token's market entry.

## Bidding Curve Formula

The pricing curve is defined by the following formula:

```math
f(x) = 
\begin{cases} 
0.1 \cdot x & \text{if } x < 50 \\
0.1 \cdot 50 + 0.5 \cdot (x - 50) & \text{if } 50 \leq x < 150 \\
10 \cdot 2^{(x - 150)} + 45 & \text{if } x \geq 150
\end{cases}
