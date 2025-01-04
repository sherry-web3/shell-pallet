// make sure this pallet uses "no_std" for compiling to wasm.
#![cfg_attr(not(feature = "std"), no_std)]

// Re-export pallet items so that they can be accessed from the crate namespace.
pub use pallet::*;

pub type Balance = u128;

#[frame_support::pallet(dev_mode)]
pub mod pallet {
    // Import various useful types required by all FRAME pallets.
    use super::*;
    use frame_support::{pallet, pallet_prelude::*};
    use frame_system::pallet_prelude::*;
    use scale_info::prelude::vec::{self, Vec};

    // The `Pallet` struct serves as a placeholder to implement the traits, methods, and dispatchables in
    // this pallet.
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// The pallet's configuration trait,
    #[pallet::config]
    pub trait Config: frame_system::Config {}

    /// ** STORAGE **
    #[pallet::storage]
    #[pallet::getter(fn  dummy)]
    pub(super) type Dummy<T: Config> = StorageValue<_, Balance>;

	#[pallet::storage]
	pub(super) type DummyStorageVec<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;


    /// ** PALLET CALL BlOCK **
    #[pallet::call]
    impl<T: Config> Pallet<T> { 
        // // Your Pallet's Callable function go here
        // // Read the references material on the different callable functions you can create here.
		// #[pallet::weight(0)]
		pub fn accumlate_dummy(origin: OriginFor<T>, increase_by: Balance) -> DispatchResult {
            // Check that the caller is signed!
            let _sender = ensure_signed(origin)?;
            Dummy::<T>::mutate(|dummy| {
                // Using `saturating_add` instead of a regular `+` to avoid overflowing
                let new_dummy = dummy.map_or(increase_by, |d| d.saturating_add(increase_by));
                *dummy = Some(new_dummy);
            });
            Ok(())
        }

		pub fn add_to_vec(origin: OriginFor<T>, account_id: AccountId) -> DispatchResult {
            
			let _caller = ensure_signed(origin)?;


            DummyStorageVec::put(account_id);
			
			Ok(())
        }
    }
}
