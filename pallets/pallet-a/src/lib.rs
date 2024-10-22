#![cfg_attr(not(feature = "std"), no_std)]

pub use crate::pallet_a::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet_a {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
	}

    #[pallet::storage]
    pub type DummyCounter<T> = StorageValue<_, u32>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
        CounterUpdated
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::dummy_weight())]
        pub fn dummy_call(_origin: OriginFor<T>, number: u32) -> DispatchResult {
            <DummyCounter<T>>::put(number);
            Self::deposit_event(Event::CounterUpdated);
            Ok(())
        }
	}

    impl<T: Config> Pallet<T> {
        pub fn get_dummy_counter() -> u32 {
            <DummyCounter<T>>::get().unwrap_or_else(|| 0)
        }
    }
}