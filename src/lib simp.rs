#![cfg_attr(not(feature = "std"), no_std)]

// Expose the pallet's types and storage to the runtime
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::{pallet_prelude::*, dispatch::DispatchResult};
    use frame_system::pallet_prelude::*;

    // Define the pallet struct
    #[pallet::pallet]
    pub struct Pallet<T>(_);

    // Configure the pallet
    #[pallet::config]
    pub trait Config: frame_system::Config {}

    // Storage item
    #[pallet::storage]
    #[pallet::getter(fn stored_value)]
    pub type StoredValue<T> = StorageValue<_, u32, ValueQuery>;

    // Dispatchable functions
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10_000)]
        pub fn set_value(origin: OriginFor<T>, value: u32) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            StoredValue::<T>::put(value);
            Ok(())
        }

        #[pallet::weight(10_000)]
        pub fn clear_value(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            StoredValue::<T>::kill();
            Ok(())
        }
    }
}
