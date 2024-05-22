use anchor_lang::prelude::*;

declare_id!("5pwNcvridRdpj5UNpB2YW8YjfSN5Et2ZCvQ7EZLEdeqT");

#[program]
pub mod vote {
    
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _url: String) -> Result<()> {
        ctx.accounts.vote.score = 0;
        ctx.accounts.vote.last_vote = None;
        ctx.accounts.vote.bump = ctx.bumps.vote;
        Ok(())
    }

    pub fn upvote(ctx: Context<Vote>, _url: String) -> Result<()> {
        ctx.accounts.vote.score += 1;
        ctx.accounts.vote.last_vote = Some(ctx.accounts.signer.key());
        Ok(())
    }

    pub fn downvote(ctx: Context<Vote>, _url: String) -> Result<()> {
        ctx.accounts.vote.score -= 1;
        ctx.accounts.vote.last_vote = Some(ctx.accounts.signer.key());
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(_url: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = VoteState::INIT_SPACE,
        seeds = [_url.as_bytes().as_ref()],
        bump
    )]
    vote: Account<'info, VoteState>,
    system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(_url: String)]
pub struct Vote<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        mut,
        seeds = [_url.as_bytes().as_ref()],
        bump = vote.bump
    )]
    vote: Account<'info, VoteState>,
    system_program: Program<'info, System>
}

#[account]
pub struct VoteState {
    score: i64,
    last_vote: Option<Pubkey>,
    bump: u8
}

impl Space for VoteState {
    const INIT_SPACE: usize = 8 + 8 + 33 + 1;
}