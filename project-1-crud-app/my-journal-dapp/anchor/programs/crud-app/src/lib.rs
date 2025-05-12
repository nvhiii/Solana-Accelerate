#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("9CzdMFuaTAchP6KyM1hAuzgnxBAqXMywKy28Jtfei7yo");

#[program]
pub mod counter {
    use super::*;


    // instruction for C(reate)RUD app
    // instruction handlers
    pub fn create_journal_entry(ctx: Context<CreateEntry>, title: String, message: String ) -> Result<()> {

      let journal_entry = &mut ctx.accounts.journal_entry;
      journal_entry.owner = *ctx.accounts.owner.key;
      journal_entry.title = title;
      journal_entry.message = message;
      Ok(())

    }

    // CR(ead)UD app section
    // no explicit instruction here, as we can simply query the blockchain lol

    // instruction for CRU(pdate)D
    pub fn update_journal_entry(ctx: Context<UpdateEntry>, _title: String, message: String) -> Result<()> {

      let journal_entry = &mut ctx.accounts.journal_entry;
      journal_entry.message = message;

      Ok(())

    }

    // instruction for CRUD(elete)
    pub fn delete_journal_entry(_ctx: Context<DeleteEntry>, _title: String) -> Result<()> {

      Ok(())

    }

}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateEntry<'info> {

  #[account(

    mut,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump,
    // we may add or delete charas, hence rent will change
    realloc = 8 + JournalEntryState::INIT_SPACE,
    realloc::payer = owner,
    realloc::zero = true, // before doing rent calc, we set to 0 for a clean slate

  )]
  pub journal_entry: Account<'info, JournalEntryState>,

  #[account(mut)]
  pub owner: Signer<'info>,

  pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteEntry<'info> {

  #[account(
    mut,
    seeds = [title.as_bytes(), owner.key().as_ref()],
    bump,
    close = owner, // only owner should be able to close journal entry
  )]
  pub journal_entry: Account<'info, JournalEntryState>,

  #[account(mut)]
  pub owner: Signer<'info>, 

  pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
#[instruction(title: String)] // pulling this from the instruction
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
