#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
use frame_system::ensure_signed;

pub mod alt_bn128;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as TemplateModule {

	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {

		EcPoint(Vec<u8>, AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// alt_bn128 errors
		AltBn128Error,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

// TODO Specify gas costs

		#[weight = 10_000]
		pub fn alt_bn128_pairing_check(origin, data:Vec<u8>) -> dispatch::DispatchResultWithPostInfo {
			match alt_bn128::alt_bn128_pairing_check(&data) {
				Err(_) => Err(Error::<T>::AltBn128Error.into()),
				Ok(res) => Ok(Some(res as u64).into())
			}
		}

		#[weight = 10_000]
		pub fn alt_bn128_g1_multiexp(origin, data:Vec<u8>) -> dispatch::DispatchResult {
			match alt_bn128::alt_bn128_g1_multiexp(&data) {
				Err(_) => Err(Error::<T>::AltBn128Error.into()),
				Ok(res) => {
					let who = ensure_signed(origin)?;
					Self::deposit_event(RawEvent::EcPoint(res, who));
					Ok(())
				}
			}
		}

		#[weight = 10_000]
		pub fn alt_bn128_g1_sum(origin, data:Vec<u8>) -> dispatch::DispatchResult {
			match alt_bn128::alt_bn128_g1_sum(&data) {
				Err(_) => Err(Error::<T>::AltBn128Error.into()),
				Ok(res) => {
					let who = ensure_signed(origin)?;
					Self::deposit_event(RawEvent::EcPoint(res, who));
					Ok(())
				}
			}
		}
	}
}
