# Project Title

CafePoints

# Project Description

CafePoints is a Soroban smart contract on the Stellar network designed for a cafe loyalty program. It enables the shop owner (`admin`) to initialize the contract and award points to customers after purchases, while allowing customers to redeem their points for rewards and enabling anyone to query customer point balances and total points issued.

# Project Vision

To replace easily lost paper loyalty cards and make point tracking transparent and verifiable on-chain for cafes and their customers using Stellar smart contracts.

# Key Features

- One-time contract initialization (`init`) that sets the `admin` address and initializes total issued points to `0`.
- Admin-restricted point issuance (`add_points`) that credits positive `i128` points to a customer, updates `last_updated` with the ledger timestamp, and increments `TotalIssued`.
- Customer point redemption (`redeem_points`) that deducts points from the customer's balance for a `reward` string and updates `last_updated`, preventing redemptions exceeding the current balance or non-positive amounts.
- Public read methods (`get_points` and `get_total_issued`) to inspect a customer's current point balance and the cumulative points issued.
- Explicit contract error codes (`AlreadyInitialized`, `NotInitialized`, `Unauthorized`, `InvalidAmount`, `InsufficientPoints`) enforced via `Result` and `require_auth()`.

# Usage Instructions

Build the contract from the workspace:

```powershell
stellar contract build