use anchor_lang::prelude::*;

declare_id!("JDH6YuAFD6c8YR8xaEgzvqwqCB6FJaV6oYeiFd79Bk3X");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
