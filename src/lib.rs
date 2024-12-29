// # Shell Pallet
//
// A Pallet with minimal functionality to use as a starting point when creating a new frame Pallet.
// WIP

// We make sure this pallet use "no_std" for compiling to wasm
#![cfg_attr(not(feature = "std"), no_std)]

use sp_arithmetic::traits::Saturating;

// Re-export pallet items so that they can be accessed from the create namespace
pub use pallet::*;

// ** TESTING MODULE **
#[cfg(test)]
mod tests;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
	// import various useful types required for all FRAME pallets
	use super::*;
	use frame_support::{dispatch::DispatchResult, pallet_prelude::*};
	use frame_system::pallet_prelude::{OriginFor, *};
use sp_runtime::traits::Saturating;

	// The `Pallet` struct serves as a placeholder to implement the traits, methods, and dispatchables in
	// this pallet.
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// ** PALLET CONFIGURATION TRAIT **
	#[pallet::config]
	pub trait Config: pallet_balances::Config + frame_system::Config {}

	/// ** STORAGE **
	#[pallet::storage]
	#[pallet::getter(fn  dummy)]
	pub(super) type Dummy<T: Config> = StorageValue<_, T::Balance>;


	// ** GENESIS **
	// The genesis config type.
	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)]
	pub struct GenesisConfig<T: Config> {
		pub dummy: T::Balance,
	}
	
	// The build of genesis for the pallet.
	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			Dummy::<T>::put(&self.dummy);
			
		}
	}

	/// ** PALLET CALL BlOCK ** 
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		// Your Pallet's Callable function go here
		// Read the references material on the different callable functions you can create here.
		pub fn accumlate_dummy(origin: OriginFor<T>, increase_by: T::Balance) -> DispatchResult {
			
			// Check that the caller is signed!
			let _sender = ensure_signed(origin)?;

			// Quick Demo of none values in storage
			let current_value_in_dummy =  Self::dummy();
			assert_eq!(current_value_in_dummy, None);

			Dummy::<T>::mutate(|dummy| {
				// Using `saturating_add` instead of a regular `+` to avoid overflowing
				let new_dummy = dummy.map_or(increase_by, |d| d.saturating_add(increase_by));
				*dummy = Some(new_dummy);
			});
			Ok(())

		}

		// A function that deletes the storage of dummy, and that can only be called by some root origin
		pub fn kill_dummy(origin: OriginFor<T>) -> DispatchResult {
			let _caller = ensure_root(origin)?;

			Dummy::<T>::kill();

			Ok(())
		}
	}
} 
