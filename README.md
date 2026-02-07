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

**Recommended: GitHub Actions (no local toolchain needed)**

Push to `main` and the [workflow](.github/workflows/build-and-deploy.yml) will build the program. Artifacts (`.so` + IDL) are uploaded.

To **deploy to devnet** from CI:
1. Create a Solana keypair (or use AgentWallet’s): `solana-keygen new -o keypair.json`
2. Base64-encode it: `cat keypair.json | base64 -w0` (Linux) or `base64 -i keypair.json` (macOS)
3. In GitHub: repo → Settings → Secrets and variables → Actions → New repository secret: name `SOLANA_DEPLOY_KEYPAIR`, value = the base64 string
4. Run the workflow manually (Actions → Build and Deploy → Run workflow) or set repo variable `DEPLOY_DEVNET` = `true` to deploy on every push to main

**Local (if you have Anchor + Solana 1.18):**
```bash
anchor build
anchor deploy --provider.cluster devnet
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
