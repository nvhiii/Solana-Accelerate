#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("HsBtyhQe2fdyWL3ebYWznRtjKe31ueNt3GMRzCRmTenU");

#[program]
pub mod counter {
    use super::*;

    // instruction handlers
    pub fn creaste_journal_entry(ctx: Context<CreateEntry>, ) -> Result<()> {

    }

}

#[derive(Accounts)]
pub struct CreateEntry<'info> {

  #[account(
    init,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    // always include the bump when defining the seed
    bump,
    space = 8 + JournalEntryState::INIT_SPACE, // 8 bytes is alloc for the anchor discrim size always
    payer = owner,
  )]

  pub journal_entry: Account<'info, JournalEntryState>,

  // since the owner is the one paying, the state of acc changes, hence this is mutable
  #[account(mut)]
  pub owner: Signer<'info>,

  pub system_program: Program<'info, System>,


}

#[account]
#[derive(InitSpace)]
pub struct JournalEntryState {

  pub owner: Pubkey,

  #[max_len(50)]
  pub title: String,

  #[max_len(1000)]
  pub message: String,


}
