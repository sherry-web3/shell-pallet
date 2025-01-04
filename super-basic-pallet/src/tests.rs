// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0



use crate::*;
use frame_support::{
	assert_ok, derive_impl,
	dispatch::{DispatchInfo, GetDispatchInfo},
	traits::{ConstU64, OnInitialize},
};
use sp_core::H256;
// The testing primitives are very useful for avoiding having to work with signatures
// or public keys. `u64` is used as the `AccountId` and no `Signature`s are required.
use sp_runtime::{
	traits::{BlakeTwo256, DispatchTransaction, IdentityLookup},
	transaction_validity::TransactionSource::External,
	BuildStorage,
};
// Reexport crate as its pallet name for construct_runtime.
use crate as pallet_example_super_basic;

type Block = frame_system::mocking::MockBlock<Test>;

// For testing the pallet, we construct a mock runtime.
frame_support::construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		Example: pallet_example_super_basic,
	}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type Nonce = u64;
	type Hash = H256;
	type RuntimeCall = RuntimeCall;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for Test {
	type AccountStore = System;
}

impl Config for Test {}

// This function basically just builds a genesis storage key/value store according to
// our desired mockup.
pub fn new_test_ext() -> sp_io::TestExternalities {	
	let t = RuntimeGenesisConfig {
		// We use default for brevity, but you can configure as desired if needed.
		system: Default::default(),
		balances: Default::default(),
		example: pallet_example_super_basic::GenesisConfig {
			dummy: 42,
		},
	}
	.build_storage()
	.unwrap();
	t.into()
}
#[test]
fn genesis_storage_works() {
	new_test_ext().execute_with(|| {
		
	// Check that 42 is actually the aswer of the universe
	assert_eq!(Example::dummy(), Some(42));
	});

}

#[test]
fn accumlate_dummy_works() {
	new_test_ext().execute_with(|| {
		
		let genesis_val = 42;
		let increment_by = 100;

		// check that accumlate works when we have some values in dummy already
		assert_ok!(Example::accumlate_dummy(RuntimeOrigin::signed(1), increment_by));

		assert_eq!(Example::dummy(), Some(genesis_val + increment_by));
	});
}


#[test]
fn kill_dummy_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(Example::dummy(), Some(42));
		assert_ok!(Example::kill_dummy(RuntimeOrigin::root()));
		assert_eq!(Example::dummy(), None);

	});
} 