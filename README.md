# Title
Stellar Private Voting DApp (voting-contract)

# Project Description
Traditional voting systems (based on the one-person-one-vote mechanism) often fail to reflect the true intensity of voters’ preferences toward a specific option. This project was developed to address that limitation, making it particularly suitable for university clubs, student organizations, or classroom communities.

By implementing a private voting mechanism on the Stellar blockchain network (using Rust and the Soroban SDK), this smart contract allows voters not only to express what they want, but also how strongly they desire it. The system ensures transparency while preventing individuals or small groups from easily manipulating the final results.

# Core Features
## Secure Initialization
The contract tightly binds administrative authority to a designated Admin account (e.g., the executive board) immediately upon deployment, preventing unauthorized ownership takeover attempts.

## Voting Credit Allocation (`mint_credits`)
Administrators can transparently distribute "Voting Credits" to eligible members within the system.

## Non-Linear Voting Cost (`vote`)
The system applies a quadratic cost formula:

```math
Cost = (Number\ of\ Votes)^2
```

This means allocating multiple votes to a single option becomes exponentially more expensive, serving as a natural anti-whale and anti-manipulation mechanism.

## Transparent Tracking
Read-only functions (`get_credits`, `get_poll_votes`) allow anyone to publicly check members’ credit balances and current voting results without paying network gas fees.

## Strict Authentication
The contract leverages Soroban’s `require_auth()` function to ensure that every state-changing action is cryptographically signed and verified by the legitimate wallet owner.

# Contract Link
Contract on Stellar Expert (Testnet):  
https://stellar.expert/explorer/testnet/contract/CDOES5GHCFZGMXHIYORMRWOFQIOMNXEK3SUD2BSENNRSCZNSUGFGWKDL

## Interaction Screenshots
- Initialization and Credit Minting: `screenshot_deploy.png`
- Successful Voting Execution: `screenshot_Invoke contract.png`
# Future Scope

## Front-end Integration
Develop a user-friendly web interface using React.js and integrate it with the Stellar Freighter Wallet, enabling members to vote easily without relying on Terminal commands or IDE platforms.

## Real Token Integration
Upgrade the current virtual "Voting Credits" to comply with the Soroban Token Interface standard, allowing real crypto assets (such as USDC or XLM) to be used as voting costs for large-scale DAO governance models.

## Time-Bound Polls
Extend the contract logic to support multiple simultaneous voting events with strict start and end deadlines, based on Stellar ledger sequence data.

# Author Profile
- Full Name: Ngô Phạm Khánh Ngọc
- Role: Student
- Contact/GitHub: https://github.com/idpgem81
