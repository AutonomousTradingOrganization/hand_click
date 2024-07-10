use anchor_lang::prelude::*;
// account struct for add_and_storeuse cunter::cpi::accounts::CounterClickOp;

// The program definition for Counter
use counter::program::Counter;

// the account where Counter is storing the sum
use counter::CounterData;

declare_id!("GGT2DGMhGv9TMBdTfkVpkULze2C4ct9FhECzXBS6LqWx");

#[program]
pub mod hand {
	use counter::cpi::accounts::CounterClickOp;

use super::*;

	pub fn hand_click(ctx: Context<HandCounterOp>) -> Result<()> {
		let cpi_ctx = CpiContext::new(
			ctx.accounts.counter_program.to_account_info(),
			CounterClickOp {
				counter_data_account: ctx.accounts.counter_data_account.to_account_info(),
			}
		);

		let res = counter::cpi::click(cpi_ctx);

		// return an error if the CPI failed
		if res.is_ok() {
			return Ok(());
		} else {
			return err!(Errors::CPIToCounterFailed);
		}
	}
}

#[error_code]
pub enum Errors {
	#[msg("cpi to 'counter' failed")]
	CPIToCounterFailed,
}

#[derive(Accounts)]
pub struct HandCounterOp<'info> {
	#[account(mut)]
	pub counter_data_account: Account<'info, CounterData>,

	pub counter_program: Program<'info, Counter>,
}
