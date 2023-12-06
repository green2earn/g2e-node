#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, traits::Currency};
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::vec::Vec;
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_assets::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// The currency trait.
		type Currency: Currency<Self::AccountId>;

		/// The maximum amount of spec
		#[pallet::constant]
		type SpecMaxLength: Get<u32>;

		#[pallet::constant]
		type NameMaxLength: Get<u32>;

		#[pallet::constant]
		type SymbolMaxLength: Get<u32>;

		// Type representing the weight of this pallet
		// type WeightInfo: WeightInfo;
	}

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [who, token_id]
		CreateToken { who: T::AccountId, token_name: Vec<u8> },
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		TooLong,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::call_index(0)]
		#[pallet::weight(100_000)]
		pub fn create_token(
			origin: OriginFor<T>,
			asset_id: <T as pallet_assets::Config>::AssetIdParameter,
			token_name: Vec<u8>,
			token_symbol: Vec<u8>,
			decimals: u8,
			total_supply: <T as pallet_assets::Config>::Balance,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin.clone())?;
			let admin = <T::Lookup as sp_runtime::traits::StaticLookup>::unlookup(who.clone());

			// Create Asset
			pallet_assets::Pallet::<T>::create(
				origin.clone(),
				asset_id,
				admin.clone(),
				total_supply,
			)?;

			// Set metadata

			pallet_assets::Pallet::<T>::set_metadata(
				origin.clone(),
				asset_id,
				token_name.clone(),
				token_symbol,
				decimals,
			)?;

			// Mint Token
			pallet_assets::Pallet::<T>::mint(origin, asset_id, admin.clone(), total_supply)?;

			// Emit an event.
			Self::deposit_event(Event::CreateToken { who, token_name });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}
	}
}
