#![cfg_attr(not(feature = "std"), no_std)]

pub use crate::pallet_b::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet_b {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
    use pallet_a::Config as PalletAConfig;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + PalletAConfig {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
	}

    #[pallet::storage]
    pub type Dummy<T> = StorageValue<_, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        Dummy
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(<T as pallet_b::Config>::WeightInfo::dummy_weight())]
		pub fn dummy_call_against_pallet_a(_origin: OriginFor<T>, number: u32) -> DispatchResult {
            pallet_a::DummyCounter::<T>::put(number);
            Self::deposit_event(Event::Dummy);
			Ok(())
		}
	}
}