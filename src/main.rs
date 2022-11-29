#[subxt::subxt(runtime_metadata_path = "./metadata/kusama.scale")]
pub mod kusama {
    #[subxt(substitute_type = "sp_runtime::multiaddress::MultiAddress")]
    use ::subxt::ext::sp_runtime::MultiAddress;
}

use subxt::{ext::sp_runtime::{AccountId32, MultiAddress}, };
use parity_scale_codec::Encode as _;
use std::str::FromStr as _;
use kusama::runtime_types::{
    kusama_runtime::RuntimeCall,
    pallet_conviction_voting::pallet::Call as ConvictionVotingCall,
    pallet_proxy::pallet::Call as ProxyCall,
    pallet_utility::pallet::Call as UtilityCall,
};

struct UserInputs {
    to: &'static str,
    conviction: u8,
    balance: u128,
}

fn user_inputs() -> UserInputs {
    let decimals: u128 = 10_u128.pow(12);
    return UserInputs {
        // address
        to: "GcDZZCVPwkPqoWxx8vfLb4Yfpz9yQ1f4XEyqngSH8ygsL9p",
        // 0..=6
        conviction: 1,
        // KSM has 12 decimals
        balance: 1 * decimals,
    }
}

fn main() -> Result<(), &'static str> {
    let prefs = user_inputs();
    let mut calls = Vec::new();

    // Track 0: Root
    add_delegation(&mut calls, 0, prefs.to, prefs.conviction, prefs.balance)?;
    // Track 1: Whitelisted Caller
    // Track 10: Staking Admin
    // Track 11: Treasurer
    // Track 12: Lease Admin
    // Track 13: Fellowship Admin
    // Track 14: General Admin
    // Track 15: Auction Admin

    let batch = RuntimeCall::Utility(UtilityCall::batch {
        calls: calls.into_iter().collect()
    });

    let bytes = batch.encode();

    println!("0x{}", hex::encode(bytes));

    Ok(())
}

fn add_delegation(
    calls: &mut Vec<ConvictionVotingCall>,
    class: u8,
    to: &str,
    conviction: u8,
    balance: u128,
) -> Result<(), &'static str> {
    let to = AccountId32::from_str(to)?;
    calls.push(ConvictionVotingCall::delegate {
        class: class.into(),
        to: MultiAddress::Id(to.clone()),
        conviction: conviction.into(),
        balance: balance.into(),
    });
    Ok(())
}
