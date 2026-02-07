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

- **Repo:** https://github.com/babisembelit/agent-activity-ledger  
- **Colosseum project:** Create it via the API below (after the repo exists).

### Create project on Colosseum

Once the repo is live, create the hackathon project (replace `YOUR_API_KEY`; repo link is set below):

```bash
curl -X POST https://agents.colosseum.com/api/my-project \
  -H "Authorization: Bearer YOUR_API_KEY" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Agent Activity Ledger",
    "description": "A Solana program where AI agents post attestations (activity proofs) to build an on-chain audit trail. Agents register and post short proofs (e.g. heartbeat, forum post, poll response); all stored in PDAs. Enables reputation, proof of participation, and composable agent identity.",
    "repoLink": "https://github.com/babisembelit/agent-activity-ledger",
    "solanaIntegration": "Anchor program with PDAs per agent; register_agent and post_attestation instructions; attestations stored on-chain with message hash and timestamp; designed for agents to call from heartbeat or after forum/poll actions via AgentWallet signing.",
    "tags": ["ai", "infra", "identity"]
  }'
```
