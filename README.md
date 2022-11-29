# Delegate All Tracks

Polkadot's OpenGov has track-independent vote delegation. That is, an account can delegate conviction-weighted votes to different accounts for different tracks. For example, one might delegate to account A for Staking referenda and account B for Treasury proposals. However, the `delegate` function must be called for each track, meaning even if one wants to delegate all their votes to the same delegate, they must submit a delegation for each track.

This script constructs a call that will delegate some configuration of (amount, conviction, delegate) to every track.

This only works on Kusama for now.

## Usage

You need to edit the `UserInputs` in `src/main.rs`:

```rust
fn user_inputs() -> UserInputs {
    let decimals: u128 = 10_u128.pow(12);
    return UserInputs {
        // Address to which to delegate votes.
        to: "GcDZZCVPwkPqoWxx8vfLb4Yfpz9yQ1f4XEyqngSH8ygsL9p",
        // Conviction, from 0 (0.1x) to 6 (6x).
        conviction: 1,
        // Amount of KSM to delegate. KSM has 12 decimals.
        amount: 1 * decimals,
        // Submit this call via proxy. Enter `Some("address")` if submitting via proxy.
        as_proxy: None,
    }
}
```

Once you've configured your settings, just call `cargo run`, and it will give you a call. You can then paste this in e.g. Polkadot JS Apps UI and sign/submit it with whatever wallet you prefer.

[Call generated from above sample.](https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fkusama-rpc.polkadot.io#/extrinsics/decode/0x18003c1401000000b2636043fc3b8dfa608167a9fb6fb9d065b9f2f5821dc4bfc9785a244b24a92a010010a5d4e800000000000000000000001401010000b2636043fc3b8dfa608167a9fb6fb9d065b9f2f5821dc4bfc9785a244b24a92a010010a5d4e8000000000000000000000014010a0000b2636043fc3b8dfa608167a9fb6fb9d065b9f2f5821dc4bfc9785a244b24a92a010010a5d4e8000000000000000000000014010b0000b2636043fc3b8dfa608167a9fb6fb9d065b9f2f5821dc4bfc9785a244b24a92a010010a5d4e8000000000000000000000014010c0000b2636043fc3b8dfa608167a9fb6fb9d065b9f2f5821dc4bfc9785a244b24a92a010010a5d4e8000000000000000000000014010d0000b2636043fc3b8dfa608167a9fb6fb9d065b9f2f5821dc4bfc9785a244b24a92a010010a5d4e8000000000000000000000014010e0000b2636043fc3b8dfa608167a9fb6fb9d065b9f2f5821dc4bfc9785a244b24a92a010010a5d4e8000000000000000000000014010f0000b2636043fc3b8dfa608167a9fb6fb9d065b9f2f5821dc4bfc9785a244b24a92a010010a5d4e800000000000000000000001401140000b2636043fc3b8dfa608167a9fb6fb9d065b9f2f5821dc4bfc9785a244b24a92a010010a5d4e800000000000000000000001401150000b2636043fc3b8dfa608167a9fb6fb9d065b9f2f5821dc4bfc9785a244b24a92a010010a5d4e8000000000000000000000014011e0000b2636043fc3b8dfa608167a9fb6fb9d065b9f2f5821dc4bfc9785a244b24a92a010010a5d4e8000000000000000000000014011f0000b2636043fc3b8dfa608167a9fb6fb9d065b9f2f5821dc4bfc9785a244b24a92a010010a5d4e800000000000000000000001401200000b2636043fc3b8dfa608167a9fb6fb9d065b9f2f5821dc4bfc9785a244b24a92a010010a5d4e800000000000000000000001401210000b2636043fc3b8dfa608167a9fb6fb9d065b9f2f5821dc4bfc9785a244b24a92a010010a5d4e800000000000000000000001401220000b2636043fc3b8dfa608167a9fb6fb9d065b9f2f5821dc4bfc9785a244b24a92a010010a5d4e80000000000000000000000)

## To Do

Accept arguments as CLI args so you don't need to rebuild every time.
