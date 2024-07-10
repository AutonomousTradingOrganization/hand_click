use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("JDH6YuAFD6c8YR8xaEgzvqwqCB6FJaV6oYeiFd79Bk3X");


#[program]
pub mod counter {

	use super::*;

	pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
		msg!("Data Account Initialized: {}", ctx.accounts.counter_data_account.key());

		Ok(())
	}

	pub fn click(ctx: Context<CounterClickOp>) -> Result<()> {
		// MODIFY/UPDATE THE DATA ACCOUNT
		ctx.accounts.counter_data_account.result += 1;
		Ok(())
	}
}

#[account]
pub struct CounterData {
	pub result: u64,
}

#[derive(Accounts)]
pub struct CounterClickOp<'info> {   
	#[account(mut)]
	pub counter_data_account: Account<'info, CounterData>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
	#[account(init, payer = signer, space = size_of::<CounterData>() + 8)]
	pub counter_data_account: Account<'info, CounterData>,

	#[account(mut)]
	pub signer: Signer<'info>,

	pub system_program: Program<'info, System>,
}
