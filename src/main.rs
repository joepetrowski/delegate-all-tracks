#[subxt::subxt(runtime_metadata_url = "wss://kusama-rpc.polkadot.io:443")]
pub mod kusama {
    #[subxt(substitute_type = "sp_runtime::multiaddress::MultiAddress")]
    use ::subxt::ext::sp_runtime::MultiAddress;
}

use subxt::{ext::sp_runtime::{AccountId32, MultiAddress}, };
use parity_scale_codec::Encode as _;
use std::str::FromStr as _;
use kusama::runtime_types::{
    kusama_runtime::RuntimeCall,
    pallet_conviction_voting::{conviction::Conviction, pallet::Call as ConvictionVotingCall},
    pallet_proxy::pallet::Call as ProxyCall,
    pallet_utility::pallet::Call as UtilityCall,
};

type Address = &'static str;

struct UserInputs {
    to: Address,
    conviction: u8,
    amount: u128,
    as_proxy: Option<Address>,
}

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

fn main() -> Result<(), &'static str> {
    let prefs = user_inputs();
    let mut calls = Vec::new();

    // Tracks from
    // https://github.com/paritytech/polkadot/blob/release-v0.9.32/runtime/kusama/src/governance/tracks.rs#L288L318

    // Track 0: Root
    add_delegation(&mut calls, 0, &prefs)?;
    // Track 1: Whitelisted Caller
    add_delegation(&mut calls, 1, &prefs)?;
    // Track 10: Staking Admin
    add_delegation(&mut calls, 10, &prefs)?;
    // Track 11: Treasurer
    add_delegation(&mut calls, 11, &prefs)?;
    // Track 12: Lease Admin
    add_delegation(&mut calls, 12, &prefs)?;
    // Track 13: Fellowship Admin
    add_delegation(&mut calls, 13, &prefs)?;
    // Track 14: General Admin
    add_delegation(&mut calls, 14, &prefs)?;
    // Track 15: Auction Admin
    add_delegation(&mut calls, 15, &prefs)?;
    // Track 20: Referendum Canceller
    add_delegation(&mut calls, 20, &prefs)?;
    // Track 21: Referendum Killer
    add_delegation(&mut calls, 21, &prefs)?;
    // Track 30: Small Tipper
    add_delegation(&mut calls, 30, &prefs)?;
    // Track 31: Big Tipper
    add_delegation(&mut calls, 31, &prefs)?;
    // Track 32: Small Spender
    add_delegation(&mut calls, 32, &prefs)?;
    // Track 33: Medium Spender
    add_delegation(&mut calls, 33, &prefs)?;
    // Track 34: Big Spender
    add_delegation(&mut calls, 34, &prefs)?;

    let mut call = RuntimeCall::Utility(UtilityCall::batch {
        calls: calls.into_iter().map(RuntimeCall::ConvictionVoting).collect()
    });

    if let Some(proxied) = prefs.as_proxy {
        let proxied = AccountId32::from_str(proxied)?;
        call = RuntimeCall::Proxy(ProxyCall::proxy {
            real: MultiAddress::Id(proxied),
            force_proxy_type: None,
            call: Box::new(call),
        });
    } 

    let bytes = call.encode();
    println!("0x{}", hex::encode(bytes));

    Ok(())
}

fn add_delegation(
    calls: &mut Vec<ConvictionVotingCall>,
    class: u16,
    prefs: &UserInputs,
) -> Result<(), &'static str> {
    let to = AccountId32::from_str(prefs.to.clone())?;

    let conviction = match prefs.conviction.clone() {
        0 => Conviction::None,
        1 => Conviction::Locked1x,
        2 => Conviction::Locked2x,
        3 => Conviction::Locked3x,
        4 => Conviction::Locked4x,
        5 => Conviction::Locked5x,
        6 => Conviction::Locked6x,
        _ => return Err("not a valid conviction")
    };

    calls.push(ConvictionVotingCall::delegate {
        class,
        to: MultiAddress::Id(to),
        conviction,
        balance: prefs.amount.clone(),
    });
    Ok(())
}
