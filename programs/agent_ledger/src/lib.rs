use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod agent_ledger {
    use super::*;

    /// Register an agent. Agent is identified by the wallet (authority) that signs.
    /// Creates a PDA to hold agent metadata and attestation count.
    pub fn register_agent(ctx: Context<RegisterAgent>, name: String) -> Result<()> {
        require!(name.len() <= 64, LedgerError::NameTooLong);
        let agent = &mut ctx.accounts.agent;
        agent.authority = ctx.accounts.authority.key();
        agent.name = name;
        agent.attestation_count = 0;
        agent.bump = ctx.bumps.agent;
        agent.created_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    /// Post an attestation: "I did X at time T". Message is hashed and stored.
    pub fn post_attestation(
        ctx: Context<PostAttestation>,
        message: String,
        message_hash: [u8; 32],
    ) -> Result<()> {
        require!(message.len() <= 280, LedgerError::MessageTooLong);
        let agent = &mut ctx.accounts.agent;
        agent.attestation_count = agent.attestation_count.checked_add(1).unwrap();

        let attestation = &mut ctx.accounts.attestation;
        attestation.agent = ctx.accounts.agent.key();
        attestation.authority = ctx.accounts.authority.key();
        attestation.message_hash = message_hash;
        attestation.sequence = agent.attestation_count;
        attestation.timestamp = Clock::get()?.unix_timestamp;
        attestation.bump = ctx.bumps.attestation;

        emit!(AttestationPosted {
            agent: ctx.accounts.agent.key(),
            sequence: agent.attestation_count,
            timestamp: attestation.timestamp,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct RegisterAgent<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Agent::INIT_SPACE,
        seeds = [b"agent", authority.key().as_ref()],
        bump
    )]
    pub agent: Account<'info, Agent>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PostAttestation<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"agent", authority.key().as_ref()],
        bump = agent.bump,
        has_one = authority
    )]
    pub agent: Account<'info, Agent>,

    /// Next sequence number for this attestation (agent.attestation_count + 1).
    #[account(
        init,
        payer = authority,
        space = 8 + Attestation::INIT_SPACE,
        seeds = [
            b"attestation",
            agent.key().as_ref(),
            &(agent.attestation_count + 1).to_le_bytes()
        ],
        bump
    )]
    pub attestation: Account<'info, Attestation>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Agent {
    pub authority: Pubkey,
    #[max_len(64)]
    pub name: String,
    pub attestation_count: u64,
    pub bump: u8,
    pub created_at: i64,
}

#[account]
#[derive(InitSpace)]
pub struct Attestation {
    pub agent: Pubkey,
    pub authority: Pubkey,
    pub message_hash: [u8; 32],
    pub sequence: u64,
    pub timestamp: i64,
    pub bump: u8,
}

#[event]
pub struct AttestationPosted {
    pub agent: Pubkey,
    pub sequence: u64,
    pub timestamp: i64,
}

#[error_code]
pub enum LedgerError {
    #[msg("Agent name must be 64 characters or less")]
    NameTooLong,
    #[msg("Attestation message must be 280 characters or less")]
    MessageTooLong,
}
