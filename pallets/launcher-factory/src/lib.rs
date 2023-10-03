#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

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
	use sp_runtime::traits::{AtLeast32BitUnsigned, SaturatedConversion};

	#[cfg(feature = "std")]
	use frame_support::serde::{Deserialize, Serialize};

	type AccountOf<T> = <T as frame_system::Config>::AccountId;
	type BalanceOf<T> = <<T as Config>::Currency as Currency<AccountOf<T>>>::Balance;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		// /// The token ID type
		// type TokenId: Parameter
		// 	+ Member
		// 	+ AtLeast32BitUnsigned
		// 	+ Default
		// 	+ Copy
		// 	+ MaybeSerializeDeserialize
		// 	+ codec::FullCodec;

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

	// Token info
	#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
	#[scale_info(skip_type_params(T))]
	pub struct TokenArgs<T: Config> {
		pub owner: AccountOf<T>,
		pub spec: BoundedVec<u8, T::SpecMaxLength>,
		pub name: BoundedVec<u8, T::NameMaxLength>,
		pub symbol: BoundedVec<u8, T::SymbolMaxLength>,
		pub decimals: u8,
		pub total_supply: BalanceOf<T>,
	}

	// The pallet's runtime storage items.
	// https://docs.substrate.io/main-docs/build/runtime-storage/
	#[pallet::storage]
	#[pallet::getter(fn tokens)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
	pub type Tokens<T: Config> = StorageMap<_, Blake2_128, T::AccountId, TokenArgs<T>>;

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
			spec: Vec<u8>,
			token_name: Vec<u8>,
			token_symbol: Vec<u8>,
			decimals: u8,
			total_supply: u128,
		) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/main-docs/build/origins/
			let who = ensure_signed(origin)?;

			let spec: BoundedVec<_, _> =
				spec.clone().try_into().map_err(|_| Error::<T>::TooLong)?;
			let name: BoundedVec<_, _> =
				token_name.clone().try_into().map_err(|_| Error::<T>::TooLong)?;
			let symbol: BoundedVec<_, _> =
				token_symbol.clone().try_into().map_err(|_| Error::<T>::TooLong)?;

			let total_supply: BalanceOf<T> = total_supply.saturated_into::<BalanceOf<T>>();

			// Update storage.
			Tokens::<T>::insert(
				&who,
				TokenArgs { owner: who.clone(), spec, name, symbol, decimals, total_supply },
			);

			// Emit an event.
			Self::deposit_event(Event::CreateToken { who, token_name });
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::call_index(1)]
		#[pallet::weight(100_000)]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			// match <Something<T>>::get() {
			// 	// Return an error if the value has not been set.
			// 	None => return Err(Error::<T>::NoneValue.into()),
			// 	Some(old) => {
			// 		// Increment the value read from storage; will error in the event of overflow.
			// 		let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
			// 		// Update the value in storage with the incremented result.
			// 		// <Something<T>>::put(new);
			// 		Ok(())
			// 	},
			// }
			Ok(())
		}
	}
}
