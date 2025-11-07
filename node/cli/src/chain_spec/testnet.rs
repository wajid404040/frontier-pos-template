// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate chain configurations.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(clippy::clone_on_copy)]

use common_runtime::AccountId;
use ecdsa_keyring::Keyring;
use fp_account::AccountId20;
use hex_literal::hex;
use kitchensink_testnet_runtime::{
    constants::currency::*, wasm_binary_unwrap, Block, MaxNominations, SessionKeys, StakerStatus,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use polkadot_sdk::*;
use sc_chain_spec::ChainSpecExtension;
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_consensus_beefy::ecdsa_crypto::AuthorityId as BeefyId;
use sp_consensus_grandpa::AuthorityId as GrandpaId;
use sp_core::{crypto::UncheckedInto, ecdsa, sr25519, Pair, Public, H160, U256};
use sp_mixnet::types::AuthorityId as MixnetId;
use sp_runtime::{
    traits::{IdentifyAccount, Verify},
    Perbill,
};
use sp_std::collections::btree_map::BTreeMap;
use std::str::FromStr;

pub use kitchensink_testnet_runtime::RuntimeGenesisConfig;
pub use node_primitives::{Balance, Signature};

type AccountPublic = <Signature as Verify>::Signer;

// const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const ENDOWMENT: Balance = 100 * DOLLARS;
const STASH: Balance = ENDOWMENT;

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// Block numbers with known hashes.
    pub fork_blocks: sc_client_api::ForkBlocks<Block>,
    /// Known bad block hashes.
    pub bad_blocks: sc_client_api::BadBlocks<Block>,
    /// The light sync state extension used by the sync-state rpc.
    pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<Extensions>;

fn session_keys(
    // ed25519
    grandpa: GrandpaId,
    // sr25519
    babe: BabeId,
    // sr25519
    im_online: ImOnlineId,
    // sr25519
    authority_discovery: AuthorityDiscoveryId,
    // sr25519
    mixnet: MixnetId,
    // ecdsa
    beefy: BeefyId,
) -> SessionKeys {
    SessionKeys {
        grandpa,
        babe,
        im_online,
        authority_discovery,
        mixnet,
        beefy,
    }
}

/// Helper function to generate a crypto pair from seed.
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed.
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to convert Ethereum address (H160) string to AccountId.
/// 
/// # Example
/// ```
/// let account = account_from_hex("0xf24ff3a9cf04c71dbc94d0b566f7a27b94566cac");
/// ```
pub fn account_from_hex(hex_address: &str) -> AccountId {
    // Remove "0x" prefix if present
    let hex_str = hex_address.strip_prefix("0x").unwrap_or(hex_address);
    // Parse hex string to H160
    let h160 = H160::from_str(hex_str)
        .expect("Invalid Ethereum address format");
    // Convert H160 to AccountId20, then to AccountId
    AccountId20::from(h160).into()
}

/// Helper function to generate stash, controller and session key from seed.
pub fn authority_keys_from_alice() -> (
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
    MixnetId,
    BeefyId,
) {
    let seed = "Alice";
    (
        Keyring::Alith.into(),
        Keyring::Alith.into(),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
        get_from_seed::<MixnetId>(seed),
        get_from_seed::<BeefyId>(seed),
    )
}

/// Helper function to generate stash, controller and session key from seed.
pub fn authority_keys_from_bob() -> (
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
    MixnetId,
    BeefyId,
) {
    let seed = "Bob";
    (
        Keyring::Baltathar.into(),
        Keyring::Baltathar.into(),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
        get_from_seed::<MixnetId>(seed),
        get_from_seed::<BeefyId>(seed),
    )
}

/// Helper function to generate stash, controller and session key from seed for Validator 3.
pub fn authority_keys_from_charlie() -> (
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
    MixnetId,
    BeefyId,
) {
    let seed = "Charlie";
    (
        get_account_id_from_seed::<ecdsa::Public>(seed),
        get_account_id_from_seed::<ecdsa::Public>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
        get_from_seed::<MixnetId>(seed),
        get_from_seed::<BeefyId>(seed),
    )
}

/// Helper function to generate stash, controller and session key from seed for Validator 4.
pub fn authority_keys_from_dave() -> (
    AccountId,
    AccountId,
    GrandpaId,
    BabeId,
    ImOnlineId,
    AuthorityDiscoveryId,
    MixnetId,
    BeefyId,
) {
    let seed = "Dave";
    (
        get_account_id_from_seed::<ecdsa::Public>(seed),
        get_account_id_from_seed::<ecdsa::Public>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
        get_from_seed::<MixnetId>(seed),
        get_from_seed::<BeefyId>(seed),
    )
}

fn configure_accounts(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
        MixnetId,
        BeefyId,
    )>,
    initial_nominators: Vec<AccountId>,
    endowed_accounts: Option<Vec<AccountId>>,
    stash: Balance,
) -> (
    Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
        MixnetId,
        BeefyId,
    )>,
    Vec<AccountId>,
    usize,
    Vec<(AccountId, AccountId, Balance, StakerStatus<AccountId>)>,
) {
    let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            Keyring::Alith.into(),
            Keyring::Baltathar.into(),
            Keyring::CharLeth.into(),
            Keyring::Dorothy.into(),
            Keyring::Ethan.into(),
            Keyring::Faith.into(),
        ]
    });
    // endow all authorities and nominators.
    initial_authorities
        .iter()
        .map(|x| &x.0)
        .chain(initial_nominators.iter())
        .for_each(|x| {
            if !endowed_accounts.contains(x) {
                endowed_accounts.push(x.clone())
            }
        });

    let stakers = initial_authorities
        .iter()
        .map(|x| (x.0.clone(), x.0.clone(), stash, StakerStatus::Validator))
        .collect::<Vec<_>>();

    let num_endowed_accounts = endowed_accounts.len();

    (
        initial_authorities,
        endowed_accounts,
        num_endowed_accounts,
        stakers,
    )
}

/// Helper function to create RuntimeGenesisConfig json patch for testing.
pub fn testnet_genesis(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
        MixnetId,
        BeefyId,
    )>,
    initial_nominators: Vec<AccountId>,
    root_key: AccountId,
    endowed_accounts: Option<Vec<AccountId>>,
    extra_endowed_accounts_balance: Vec<(AccountId, u128)>,
    evm_chain_id: u32,
) -> serde_json::Value {
    let (initial_authorities, endowed_accounts, num_endowed_accounts, stakers) = configure_accounts(
        initial_authorities,
        initial_nominators,
        endowed_accounts,
        STASH,
    );

    serde_json::json!({
        "balances": {
            "balances": endowed_accounts.iter().cloned().map(|x| (x, ENDOWMENT)).chain(extra_endowed_accounts_balance).collect::<Vec<_>>(),
        },
        "session": {
            "keys": initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(
                            x.2.clone(),
                            x.3.clone(),
                            x.4.clone(),
                            x.5.clone(),
                            x.6.clone(),
                            x.7.clone(),
                        ),
                    )
                })
                .collect::<Vec<_>>(),
        },
        "staking": {
            "validatorCount": initial_authorities.len() as u32,
            "minimumValidatorCount": initial_authorities.len() as u32,
            "invulnerables": initial_authorities.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
            "slashRewardFraction": Perbill::from_percent(10),
            "stakers": stakers.clone(),
        },
        "sudo": { "key": Some(root_key.clone()) },
        "babe": {
            "epochConfig": Some(kitchensink_testnet_runtime::BABE_GENESIS_EPOCH_CONFIG),
        },
        "nominationPools": {
            "minCreateBond": 10 * DOLLARS,
            "minJoinBond": 1 * DOLLARS,
        },
        "evmChainId": { "chainId": evm_chain_id },
    })
}

fn development_config_genesis_json() -> serde_json::Value {
    // ============================================================
    // YOUR CUSTOM ACCOUNTS - MegapayerTestnet
    // ============================================================
    // These are YOUR MetaMask wallet addresses
    // Funds will be automatically allocated when chain starts
    // ============================================================
    
    // Validator accounts (for your 4 validator nodes)
    let validator1_account = account_from_hex("0x14A53536671FBeE7Cc3840F26c07B0280FB3f01d");
    let validator2_account = account_from_hex("0x03A5359ABC4d625f5639fC29E9F63774E6Fee1BB");
    let validator3_account = account_from_hex("0x14123f0468256d0F322B51A3F47809bFfa840176");
    let validator4_account = account_from_hex("0x82cA4c56F1d2DF02F7fB287807af301cEe0a63CB");
    
    // Faucet account (for testing/faucet service)
    let faucet_account = account_from_hex("0x211f6C872fF68700Be5CC0232B9595F5d53290e9");
    
    // Treasury account (for governance, bounties, etc.)
    let treasury_account = account_from_hex("0xbBdCe65969Ab5ABB0A47398a4605348cE38e00DE");
    
    // Testing account (for development/testing)
    let testing_account = account_from_hex("0xa6D812269DF172a3478EaADd127aBbb57c2B4757");
    
    // Total: 1 billion MPC distribution for MegapayerTestnet
    let extra_endowed_accounts_balance = vec![
        // Faucet account - 500 million MPC (for testing/faucet)
        (faucet_account, 500_000_000 * DOLLARS),
        
        // Validators - 200 million MPC total (50M each for staking and operations)
        (validator1_account, 50_000_000 * DOLLARS),      // Validator 1
        (validator2_account, 50_000_000 * DOLLARS),      // Validator 2
        (validator3_account, 50_000_000 * DOLLARS),      // Validator 3
        (validator4_account, 50_000_000 * DOLLARS),      // Validator 4
        
        // Treasury - 200 million MPC (for governance, bounties, etc.)
        (treasury_account, 200_000_000 * DOLLARS),
        
        // Testing account - 100 million MPC (for development/testing)
        (testing_account, 100_000_000 * DOLLARS),
    ];
    
    // Add all accounts to the endowed accounts list so they get default 100 MPC
    let custom_endowed_accounts = vec![
        faucet_account,
        validator1_account,
        validator2_account,
        validator3_account,
        validator4_account,
        treasury_account,
        testing_account,
    ];
    
    // Note: For validators, we still use seed-based session keys generation
    // The validator accounts above are for staking/operations
    // Session keys will be generated separately when setting up validator nodes
    testnet_genesis(
        vec![
            authority_keys_from_alice(),   // Validator 1 (uses seed "Alice" for session keys)
            authority_keys_from_bob(),     // Validator 2 (uses seed "Bob" for session keys)
            authority_keys_from_charlie(), // Validator 3 (uses seed "Charlie" for session keys)
            authority_keys_from_dave(),    // Validator 4 (uses seed "Dave" for session keys)
        ],
        vec![], // No initial nominators
        validator1_account, // Root/admin account (Validator 1)
        Some(custom_endowed_accounts),
        extra_endowed_accounts_balance,
        20240u32, // MegapayerTestnet Chain ID
    )
}

/// Development config (single validator Alice).
pub fn development_config() -> ChainSpec {
    ChainSpec::builder(wasm_binary_unwrap(), Default::default())
        .with_name("MegapayerTestnet")
        .with_id("megapayer-testnet")
        .with_chain_type(ChainType::Development)
        .with_properties(
            serde_json::from_str(
                "{\"isEthereum\": true, \"tokenDecimals\": 18, \"tokenSymbol\": \"MPC\"}",
            )
            .expect("Provided valid json map"),
        )
        .with_genesis_config_patch(development_config_genesis_json())
        .build()
}
