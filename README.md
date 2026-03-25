# token-contracts

The IAM utility token. An SPL Token-2022 mint on Solana with the Confidential Balances extension, enabling private balance and transfer amounts while maintaining on-chain verifiability.

## Token

| Property | Value |
|----------|-------|
| Standard | SPL Token-2022 |
| Extension | Confidential Balances (ElGamal + Sigma proofs) |
| Decimals | 6 |
| Supply | Fixed at genesis |

Confidential Balances encrypt token amounts using twisted ElGamal encryption. The network verifies transfers via zero-knowledge range and equality proofs without learning the amounts. Balances remain private to the holder.

## Utility

**Validator staking.** Validators stake IAM tokens to join the Anonymity Ring and participate in verification attestations. Stake size determines selection weight in VRF-based validator assignment.

**Verification capacity.** Integrators stake IAM tokens for discounted or unlimited verifications. Staking replaces per-verification fees with a capacity tier model, reducing cost at scale.

**Governance.** IAM token holders vote on protocol parameters: minimum stake thresholds, trust score weights, challenge expiry, fee structure. One token, one vote. Staked tokens carry governance weight.

## Distribution

| Allocation | Share | Vesting |
|------------|-------|---------|
| Community | 40% | Unlocked at genesis |
| Ecosystem grants | 20% | 12-month linear |
| Treasury | 15% | Protocol-controlled |
| Team | 15% | 24-month linear, 6-month cliff |
| Initial liquidity | 10% | Unlocked at genesis |

Launch mechanism: MetaDAO or curated community sale. The first airdrop goes to IAM-verified humans only.

## Revenue Model

Integrators deposit SOL or USDC into an escrow account to fund verifications at ~$0.01 each. The protocol retains ~70% margin after Solana transaction costs (~$0.003). Revenue flows to the treasury, which buys IAM tokens on the open market and distributes them to active validators as staking rewards.

```
Integrator deposits $100
  → covers ~10,000 verifications
  → IAM retains ~$70 margin
  → treasury buys IAM tokens
  → validators earn staking rewards
```

## Architecture

The token program integrates with two other IAM Protocol programs:

- **iam-registry** (`protocol-core`): Reads validator stake amounts to determine Anonymity Ring eligibility and VRF selection weight. The registry's `register_validator` instruction will accept IAM token stakes alongside SOL.
- **executor-node**: Calculates integrator capacity tiers from staked IAM balances. Staking above threshold grants unlimited verification quota.

```
token-contracts/
└── programs/
    └── iam-token/
        └── src/
            └── lib.rs    # Token mint, Confidential Balances, vesting
```

## Status

Phase 7. The program scaffold is in place. Implementation begins after the external security audit of the core protocol (Phase 6).

## Setup

```bash
# Prerequisites: Solana CLI >= 2.2, Anchor 0.32.1, Rust

anchor build          # Compile the program
anchor test           # Run integration tests
anchor deploy         # Deploy to localnet/devnet
```

## License

MIT
