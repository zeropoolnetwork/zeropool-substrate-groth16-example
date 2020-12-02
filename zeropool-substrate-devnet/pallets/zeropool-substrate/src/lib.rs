// Zeropool Substrate Pallet
// example: {"vk":"yO5EICtE+JVjTbRYkayI0Y/BoOJtE9lsrqeCTKTDnxD8UjB8B51wrVsQsVrsi6Uk0b2UKGfs3AJUX2Eud5wnET/ze/CsefR74bOn50BmVoExPPDiJGRD2IejItInd/wbtAH7GstyB1Q1j9uROBBAgE2eEj/cwRcHJobe4MP1mQIsiHdC5KXrBZFMlcRQi19O5pRHJ3fra+CNrckf5PHVL1NDT3E4ah/xGoRIbB0DfMSC2AO5NCyuZzJiAMBEKEcLbiazu9JOT65EXSc7HGM9IKlQXgpITR/jikWNxJc/Jyn6KiimTBN/yj4NVjAogElMdLmVoelMa0SAen8Z5ZwkFc6j3IriiWbKQnnkocqd++FqYs4gTh2rFDvcn2YpAhAmnMf35ssgfTFSIOyLZeRQPJ/SzCQMvSq8p1TAkgF85xv+1Vwd0UmrwJXyPVWhevfis0jEd6Cw78ESIMwB7S4dJwNAnVjEBRrKGfOAAzBIiTQRVMSMY2a1nMP/vr57eJwrOYvVboNDUHw8N+u1KoT3vTQkt6+bdeUBw2X/HBbeuyLcmx9AdsbJ0QY1GGF4cgGnSx9kGtcL9UY4qMWVtJ++LAQAAABZB9VFKNzCZgjPMZ9MTfotIL1czmkU9p4L3+6udM/DCAIGsaMeBAN/AhWI+GDLJK3EPzfiVDtw9PWWv+mifJUEQqRUa63wkfB2CouGxTpfsMPlZW93gzGXl5C4lmqMSQnAYpBIHANPM/R/DtA6eMTKKgKBfqgSMjf8YwlmfckmEkbsEZYwsUj2B+ryafp/qj39z80B/33p62Wz+OdwpcIYLSyprNYGC1nyO/jlRIhqRFhx9qkBRjKz/ddvFv7bdAeyPpjCqbT/6zrE22RSdm1I+tceC6xm3OUJE3wX4d5XF5z1EXo17iShXLdYhwVcd//YzyysetRirUxRPeXNAuAh","proof":"Qexag8d0jvm1IWZywscRBuvdSEvlGuhvVg5Qj97vhS5VFas06bgj/yXiuZ+yJ/WZWCYDYq8e5HZPITpoaHAvGckDPBplyUtn8zZ3UI4f5E1uLmxlehAkzVK33Fp8/SEZX4v8OLLT3MP/FWhDvS43u2sLvZcCstjVjbarImuLiSA0IW7UmNgG7u8x99JExO0pp0EAGJ3PiBOzyZ/PhxUPBXvOgxhwNzx0nzZzp+aSY8yhsWxFWRl6UWzmS6J/ieUS1q5Tjwq9gs4qcX6+Q9WWRpvYVboY+f4d6smQyryKdB5Hi5E8/jWGPoD9tFJDN4KVnnESrKi7fVjH6A3twUaQEw==","input":"AwAAAMI1CN4U9DnKW3soxArLClszrtTa/MGicksQVWpir/QNW/hp3N50wmjr1CUHvGP6u6WnrdK7oRDtSHgjcjmUVyr8NQtA06gcVk9m3KPdmWele0Bx9AcLpToixRb2FCx/JQ=="}
#![cfg_attr(not(feature = "std"), no_std)]
use sp_std::prelude::*;
use sp_runtime::{
	traits::{StaticLookup, Zero}
};
use frame_support::{
	decl_module, decl_event, decl_storage, ensure, decl_error,
	traits::{Currency, EnsureOrigin, ReservableCurrency, OnUnbalanced, Get},
};
use frame_system::ensure_signed;
use alt_serde::{Deserialize, Deserializer};
use base64::{encode, decode};
use borsh::{BorshSerialize, BorshDeserialize};
use ff_uint::{
    construct_primefield_params, construct_uint, overflowing, Field, LegendreSymbol, SqrtField,
    Uint,
};
use sp_std::vec;
construct_uint! {
	pub struct U256(4);
}
/*pub type G1 = [U256;2];
pub type G2 = [U256;4];
pub type VU256 = sp_std::prelude::Vec<U256>;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct VK {
    alpha:G1,
    beta:G2,
    gamma:G2,
    delta:G2,
    ic: Vec<G1>
}
#[derive(Debug, Clone, BorshSerialize, BorshDeserialize)]
pub struct Proof {
    a:G1,
    b:G2,
    c:G1
}*/

#[serde(crate = "alt_serde")]
#[derive(Deserialize, Default)]
struct Jsonvkproofinput {
    #[serde(deserialize_with = "de_string_to_bytes")]
    vk: sp_std::prelude::Vec<u8>,
    #[serde(deserialize_with = "de_string_to_bytes")]
    proof: sp_std::prelude::Vec<u8>,
    #[serde(deserialize_with = "de_string_to_bytes")]
    input: sp_std::prelude::Vec<u8>,
}




type BalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;
type NegativeImbalanceOf<T> = <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::NegativeImbalance;

pub trait Trait: frame_system::Trait {
	/// The overarching event type.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;

	/// The currency trait.
	type Currency: ReservableCurrency<Self::AccountId>;

	/// Reservation fee.
	type ReservationFee: Get<BalanceOf<Self>>;

	/// What to do with slashed funds.
	type Slashed: OnUnbalanced<NegativeImbalanceOf<Self>>;

	/// The origin which may forcibly set or remove a proof. Root can always do this.
	type ForceOrigin: EnsureOrigin<Self::Origin>;

	/// The minimum length a proof may be.
	type MinLength: Get<usize>;

	/// The maximum length a proof may be.
	type MaxLength: Get<usize>;
}

decl_storage! {
	trait Store for Module<T: Trait> as Zeropool {
		/// The lookup table for names.
		VerificationKey: map hasher(twox_64_concat) T::AccountId => Option<(Vec<u8>, BalanceOf<T>)>;
	}
}

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId {
	//pub enum Event<T> where AccountId = <T as frame_system::Trait>::AccountId, Balance = BalanceOf<T> {
		VerificationKeySet(AccountId),

	}
);

decl_error! {
	/// Error for the Zeropool module.
	pub enum Error for Module<T: Trait> {
		TooShort,
		TooLong,
		Unnamed,
		NoneValue,
		StorageOverflow,
	}
}

decl_module! {
	/// Zeropool module declaration.
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		type Error = Error<T>;

		fn deposit_event() = default;

		/// Reservation fee.
		const ReservationFee: BalanceOf<T> = T::ReservationFee::get();

		/// The minimum length a proof may be.
		const MinLength: u32 = T::MinLength::get() as u32;

		/// The maximum length a proof may be.
		const MaxLength: u32 = T::MaxLength::get() as u32;

		/// Verify groth16
		/// # <weight>
		/// - O(1).
		/// - At most one balance operation.
		/// - One storage read/write.
		/// - One event.
		/// # </weight>
		/// //		pub fn groth16verify(origin, jvkproofinput: Vec<u8>) -> dispatch::DispatchResult {

		#[weight = 50_000_000]
		fn groth16verify(origin, jvkproofinput: Vec<u8>) {
			let sender = ensure_signed(origin)?; //check is signed
			ensure!(jvkproofinput.len() >= T::MinLength::get(), Error::<T>::TooShort); //check minimum length
			ensure!(jvkproofinput.len() <= T::MaxLength::get(), Error::<T>::TooLong);  // check maximum length
			//deserialize json jkvproofinput
			let mut vkproofinput: Jsonvkproofinput= Jsonvkproofinput::default();
			let r = serde_json::from_slice(&jvkproofinput.as_slice());
			match r {
				Ok(rs) => vkproofinput=rs,
				Err(e) => Err(Error::<T>::NoneValue)?,
			};
			let v: Vec<i32> = vec![];
			//deserialize from borsh
			let vkstorage=vkproofinput.vk.clone();
			/*let vk = base64::decode(vkproofinput.vk).unwrap();
			let vkd=VK::try_from_slice(&vk).unwrap();
			let proof = base64::decode(vkproofinput.proof).unwrap();
		    let proofd=Proof::try_from_slice(&proof).unwrap();
    		let input = base64::decode(vkproofinput.input).unwrap();
			let inputd=VU256::try_from_slice(&input).unwrap();*/
			// make verification
			//let neg_a = alt_bn128_g1_neg(proofd.a);

			let deposit = if let Some((_, deposit)) = <VerificationKey<T>>::get(&sender) {
				Self::deposit_event(RawEvent::VerificationKeySet(sender.clone()));
				deposit
			} else {
				let deposit = T::ReservationFee::get();
				T::Currency::reserve(&sender, deposit.clone())?;
				Self::deposit_event(RawEvent::VerificationKeySet(sender.clone()));
				deposit
			};

			<VerificationKey<T>>::insert(&sender, (vkstorage, deposit));
		}
	}
}

pub fn de_string_to_bytes<'de, D>(de: D) -> Result<sp_std::prelude::Vec<u8>, D::Error> where D: Deserializer<'de> {
    let s: &str = Deserialize::deserialize(de)?;
    Ok(s.as_bytes().to_vec())
}
/*pub fn alt_bn128_g1_neg(p:G1) -> G1 {
    alt_bn128_g1_sum(&[(true, p)])
}
pub fn alt_bn128_g1_sum(v:&[(bool, G1)]) -> G1{
    let data = v.try_to_vec().unwrap_or_else(|_| env::panic(b"Cannot serialize data."));
    let res = env::alt_bn128_g1_sum(&data);
    let mut res_ptr = &res[..];
    <G1 as BorshDeserialize>::deserialize(&mut res_ptr).unwrap_or_else(|_| env::panic(b"Cannot deserialize data."))
}*/