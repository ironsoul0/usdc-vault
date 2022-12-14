## Setup

### Install

```bash
git clone https://github.com/ironsoul0/usdc-vault
cd usdc-vault
yarn
```

### Run

Run demo tests using Anchor.

```bash
anchor test
```

## Notes

There are two anchor tests covering two main scenarios:

- Capital call is created and target call amount was achieved before the deadline. In that case, investor claims LP tokens getting the amount proportional to the number of deposited USDC tokens.
- Capital call is created but target call was not achieved after the deadline. In that case, investor withdraws deposited tokens and do not get any LP tokens.

There are 3 PDAs used in this program:

- PDA derived from capital call account and investor account addresses. Helps to track how much USDC user deposited in some specific capital call. Also tracks whether user already claimed LP tokens or not to prevent double claiming.
- PDA owning USDC vault of the program to allow it to send USDC back to users when withdraw operation is handled.
- PDA owning LP token mint authority to allow program to mint LP tokens to users when capital call is achieved.

Misc:

- As was noted in the task description, LP token price is fixed and it is defined in `utils.rs:12`.
- Please have a look at `tests/credix-task.ts` since it should pretty self-explanatory.
