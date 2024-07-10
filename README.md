# hand / counter (CPI)


Source : [Cross Program Invocation in Anchor](https://www.rareskills.io/post/cross-program-invocation)

The **Hand** program will call a function (`click`) on the **Counter** program.

## 1. counter

`anchor init counter`

In "/counter/src/lib.rs"

```rust
use anchor_lang::prelude::*;
use std::mem::size_of;

// REPLACE WITTH YOUR <PROGRAM_ID>declare_id!("6wZDNWprmb9TAZYMAPpT23kHDPABvBLT8jbWQKLHEmBy");

#[program]
pub mod counter {

	use super::*;

	pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
		msg!("Data Account Initialized: {}", ctx.accounts.counter_data_account.key());

		Ok(())
	}

	pub fn click(ctx: Context<CounterClickOp>) -> Result<()> {
		// MODIFY/UPDATE THE DATA ACCOUNT
		ctx.accounts.counter_data_account.value += 1;
		Ok(())
	}
}

#[account]
pub struct CounterData {
	pub value: u64,
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
```


Let's create another program `hand` that calls `counter.click()`.


## 2. hand

In "counter" project, type :

`anchor new hand`

In `Hand`’s Cargo.toml file at "programs/hand/Cargo.toml".

```toml
[dependencies]
bob = {path = "../counter", features = ["cpi"]}
```

In "/hand/src/lib.rs"

```rust
use anchor_lang::prelude::*;
// account struct for add_and_storeuse cunter::cpi::accounts::CounterClickOp;

// The program definition for Counter
use counter::program::Counter;

// the account where Counter is storing the sum
use counter::CounterData;

// REPLACE WITTH YOUR <PROGRAM_ID>declare_id!declare_id!("6wZDNWprmb9TAZYMAPpT23kHDPABvBLT8jbWQKLHEmBy");

#[program]
pub mod hand {
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
```

## 3. Test units

```javascript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Counter } from "../target/types/counter";
import { Hand } from "../target/types/hand";
import { expect } from "chai";

describe("CPI from Hand to Counter", () => {
  const provider = anchor.AnchorProvider.env();

  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  const counterProgram     = anchor.workspace.Counter as Program<Counter>;
  const handProgram        = anchor.workspace.Hand as Program<Hand>;
  const dataAccountKeypair = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await counterProgram.methods
      .initialize()
      .accounts({
        counterDataAccount: dataAccountKeypair.publicKey,
        signer            : provider.wallet.publicKey,
        systemProgram     : anchor.web3.SystemProgram.programId,
      })
      .signers([dataAccountKeypair])
      .rpc();
  });

  it("Can increment counter!", async () => {
    // Add your test here.
    const tx = await handProgram.methods
      .handClick()
      .accounts({
        counterDataAccount: dataAccountKeypair.publicKey,
        counterProgram    : counterProgram.programId,
      })
      .rpc();
  });

   it("Can assert value in Counter's data account equals 1", async () => {

    const CounterAccountValue = (
      await counterProgram.account.counterData.fetch(dataAccountKeypair.publicKey)
    ).value.toNumber();

    expect(CounterAccountValue).to.equal(1);
  });
});
```



## Repository tree

```
TO DO
``` 

## Launch

![](deploy_local_test.png)

### Local validator

`solana-test-validator --reset`

Beware it creates local files and directories at the current working directory.


### Real-time logs display

`solana logs`


### Deploy and launch tests

`anchor test --skip-local-validator`

Just check if read/write instructions works.
Display all account(s) bind to program (`await program.account.myStorage.all()`)

## Versions

``` 
rustc 1.79.0 (129f3b996 2024-06-10)
cargo 1.79.0 (ffa9cf99a 2024-06-03)
solana-cli 1.18.17 (src:b685182a; feat:4215500110, client:SolanaLabs)
anchor-cli 0.29.0
yarn 1.22.19
node v18.16.0
npm 9.6.7
``` 

`cargo build-sbf -V`
``` 
solana-cargo-build-sbf 1.18.17
platform-tools v1.41
rustc 1.75.0
``` 
