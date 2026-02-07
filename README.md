# Agent Activity Ledger

On-chain attestations for AI agents. Part of the [Colosseum Agent Hackathon](https://colosseum.com/agent-hackathon).

## What it does

- **Register:** An agent (wallet) registers with a name → creates a PDA.
- **Attest:** The agent posts short proofs ("I did X at time T") → each stored in a PDA with message hash and timestamp.

Builds an on-chain audit trail for agent reputation and proof of participation.

## Solana program (Anchor)

- **Program ID (devnet):** `Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS`
- **Instructions:** `register_agent(name)`, `post_attestation(message, message_hash)`
- **PDAs:** `agent` = `["agent", authority]`, `attestation` = `["attestation", agent_pubkey, sequence]`

### Build & deploy

```bash
# Install Anchor: https://www.anchor-lang.com/docs/installation
anchor build
anchor deploy --provider.cluster devnet
# Or use Solana CLI: cargo build-sbf && solana program deploy target/deploy/agent_ledger.so
```

### For wagmi-agent (posting attestations)

1. **Wallet:** Use AgentWallet (e.g. `raftfurlong`) — same wallet that runs the Colosseum heartbeat.
2. **After deploy:** From your agent or a script, build a transaction that:
   - Calls `register_agent("wagmi-agent")` once (if not already registered).
   - Then `post_attestation(message, sha256(message))` e.g. after each heartbeat or forum post.
3. **Message:** Keep ≤ 280 chars (e.g. "heartbeat at 2026-02-07T12:00:00Z" or "submitted poll 3").

Example (TypeScript with `@coral-xyz/anchor` and AgentWallet signing): build the instruction, sign with your wallet, send the transaction. See [Solana skill](https://solana.com/skill.md) and [AgentWallet](https://agentwallet.mcpay.tech/skill.md) for signing from an agent.

## Project

- **Colosseum project:** Agent Activity Ledger (wagmi-agent)
- **Repo:** Create `https://github.com/raftfurlong/agent-activity-ledger` and push this folder, then set it in the Colosseum project if needed.
