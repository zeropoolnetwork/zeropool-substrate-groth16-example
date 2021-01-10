// Zeropool Substrate Pallet
// example to submit to test_groth16verify: {"proof":"Qexag8d0jvm1IWZywscRBuvdSEvlGuhvVg5Qj97vhS5VFas06bgj/yXiuZ+yJ/WZWCYDYq8e5HZPITpoaHAvGckDPBplyUtn8zZ3UI4f5E1uLmxlehAkzVK33Fp8/SEZX4v8OLLT3MP/FWhDvS43u2sLvZcCstjVjbarImuLiSA0IW7UmNgG7u8x99JExO0pp0EAGJ3PiBOzyZ/PhxUPBXvOgxhwNzx0nzZzp+aSY8yhsWxFWRl6UWzmS6J/ieUS1q5Tjwq9gs4qcX6+Q9WWRpvYVboY+f4d6smQyryKdB5Hi5E8/jWGPoD9tFJDN4KVnnESrKi7fVjH6A3twUaQEw==","input":"AwAAAMI1CN4U9DnKW3soxArLClszrtTa/MGicksQVWpir/QNW/hp3N50wmjr1CUHvGP6u6WnrdK7oRDtSHgjcjmUVyr8NQtA06gcVk9m3KPdmWele0Bx9AcLpToixRb2FCx/JQ=="}
#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;
use alloc::vec;
use alt_serde::{Deserialize, Deserializer};
use borsh::{BorshDeserialize, BorshSerialize};
use ff_uint::{construct_uint, Uint};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    traits::{Currency, EnsureOrigin, Get, OnUnbalanced, ReservableCurrency},
};
use frame_system::ensure_signed;
use sp_std::prelude::*;
pub mod alt_bn128;
construct_uint! {
    pub struct U256(4);
}

pub type G1 = [U256; 2];
pub type G2 = [U256; 4];
pub type VU256 = sp_std::prelude::Vec<U256>;

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, Default)]
pub struct VK {
    alpha: G1,
    beta: G2,
    gamma: G2,
    delta: G2,
    ic: Vec<G1>,
}

#[derive(Debug, Clone, BorshSerialize, BorshDeserialize, Default)]
pub struct Proof {
    a: G1,
    b: G2,
    c: G1,
}

#[serde(crate = "alt_serde")]
#[derive(Deserialize, Default)]
struct Jsonproofinput {
    #[serde(deserialize_with = "de_string_to_bytes")]
    proof: sp_std::prelude::Vec<u8>,
    #[serde(deserialize_with = "de_string_to_bytes")]
    input: sp_std::prelude::Vec<u8>,
}

type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;
type NegativeImbalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::NegativeImbalance;

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
        /// The lookup table for verificationkey.
        VerificationKey: map hasher(twox_64_concat) T::AccountId => Option<(Vec<u8>, BalanceOf<T>)>;
    }
}

decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        VerificationKeySet(AccountId),
        VerificationKeyUpdated(AccountId),
        VerificationSuccessful(AccountId),
        VerificationFailed(AccountId),
    }
);

decl_error! {
    /// Error for the Zeropool module.
    pub enum Error for Module<T: Trait> {
        TooShort,
        TooLong,
        VerificationFailed,
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

        /// Store Verificatoion Key - VK
        /// # <weight>
        /// - O(1).
        /// - At most one balance operation.
        /// - One storage read/write.
        /// - One event.
        /// # </weight>
        /// test with: yO5EICtE+JVjTbRYkayI0Y/BoOJtE9lsrqeCTKTDnxD8UjB8B51wrVsQsVrsi6Uk0b2UKGfs3AJUX2Eud5wnET/ze/CsefR74bOn50BmVoExPPDiJGRD2IejItInd/wbtAH7GstyB1Q1j9uROBBAgE2eEj/cwRcHJobe4MP1mQIsiHdC5KXrBZFMlcRQi19O5pRHJ3fra+CNrckf5PHVL1NDT3E4ah/xGoRIbB0DfMSC2AO5NCyuZzJiAMBEKEcLbiazu9JOT65EXSc7HGM9IKlQXgpITR/jikWNxJc/Jyn6KiimTBN/yj4NVjAogElMdLmVoelMa0SAen8Z5ZwkFc6j3IriiWbKQnnkocqd++FqYs4gTh2rFDvcn2YpAhAmnMf35ssgfTFSIOyLZeRQPJ/SzCQMvSq8p1TAkgF85xv+1Vwd0UmrwJXyPVWhevfis0jEd6Cw78ESIMwB7S4dJwNAnVjEBRrKGfOAAzBIiTQRVMSMY2a1nMP/vr57eJwrOYvVboNDUHw8N+u1KoT3vTQkt6+bdeUBw2X/HBbeuyLcmx9AdsbJ0QY1GGF4cgGnSx9kGtcL9UY4qMWVtJ++LAQAAABZB9VFKNzCZgjPMZ9MTfotIL1czmkU9p4L3+6udM/DCAIGsaMeBAN/AhWI+GDLJK3EPzfiVDtw9PWWv+mifJUEQqRUa63wkfB2CouGxTpfsMPlZW93gzGXl5C4lmqMSQnAYpBIHANPM/R/DtA6eMTKKgKBfqgSMjf8YwlmfckmEkbsEZYwsUj2B+ryafp/qj39z80B/33p62Wz+OdwpcIYLSyprNYGC1nyO/jlRIhqRFhx9qkBRjKz/ddvFv7bdAeyPpjCqbT/6zrE22RSdm1I+tceC6xm3OUJE3wX4d5XF5z1EXo17iShXLdYhwVcd//YzyysetRirUxRPeXNAuAh
        /// data is LE-encoded VK struct in base64 (check groth16verify description)
        #[weight = 500_000]
        fn set_vk(origin, vkb: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?; //check is signed
            ensure!(vkb.len() >= 64, Error::<T>::TooShort); //check minimum length
            ensure!(vkb.len() <= 8192, Error::<T>::TooLong);  // check maximum length
            //deserialize from borsh
            let vkstorage=vkb.clone();
            let vk = base64::decode(vkb).unwrap_or_default();
            let _vkd=VK::try_from_slice(&vk).unwrap_or_default();
            let deposit = if let Some((_, deposit)) = <VerificationKey<T>>::get(&sender) {
                Self::deposit_event(RawEvent::VerificationKeyUpdated(sender.clone()));
                deposit
            } else {
                let deposit = T::ReservationFee::get();
                T::Currency::reserve(&sender, deposit)?;
                Self::deposit_event(RawEvent::VerificationKeySet(sender.clone()));
                deposit
            };
            <VerificationKey<T>>::insert(&sender, (vkstorage, deposit));
            // Return a successful DispatchResult
            Ok(())
        }
        /// Verify groth16 by json including proof and input (verification key is loaded from storage)
        /// # <weight>
        /// - O(1).
        /// - At most one balance operation.
        /// - One storage read/write.
        /// - One event.
        /// # </weight>
        ///
        /// data is Proof struct and inputs in LE-encoding, base64 and JSON (check groth16verify description)
        #[weight = 500_000]
        fn test_groth16verify(origin, jproofinput: Vec<u8>) -> dispatch::DispatchResult {
            let sender = ensure_signed(origin)?; //check is signed
            ensure!(jproofinput.len() >= 64, Error::<T>::TooShort); //check minimum length
            ensure!(jproofinput.len() <= 8192, Error::<T>::TooLong);  // check maximum length
            //deserialize json jproofinput
            #[allow(unused_assignments)] let mut proofinput: Jsonproofinput= Jsonproofinput::default();
            let r = serde_json::from_slice(&jproofinput.as_slice());
            match r {
                Ok(rs) => proofinput=rs,
                Err(_e) => {
                                Self::deposit_event(RawEvent::VerificationFailed(sender));
                                return Err(Error::<T>::VerificationFailed.into())
                }
            };
            //deserialize from borsh
            let proof = base64::decode(proofinput.proof).unwrap_or_default();
            let proofd=Proof::try_from_slice(&proof).unwrap_or_default();
            let input = base64::decode(proofinput.input).unwrap_or_default();
            let inputd=VU256::try_from_slice(&input).unwrap_or_default();
            // get vk from storage
            let vks = if let Some((vkstorage, _deposit)) = <VerificationKey<T>>::get(&sender) {
                vkstorage
            } else {
                Self::deposit_event(RawEvent::VerificationFailed(sender));
                return Err(Error::<T>::VerificationFailed.into())
            };
            let vk: Vec<u8> = base64::decode(&vks).unwrap_or_default();
            let vkd=VK::try_from_slice(&vk).unwrap_or_default();
            //groth16 verification
            if groth16verify(vkd,proofd,inputd){
                // Return a successful DispatchResult
                Self::deposit_event(RawEvent::VerificationSuccessful(sender));
                Ok(())
            }
            else{
                // Return a failed verification DispatchResult
                Self::deposit_event(RawEvent::VerificationFailed(sender));
                Err(Error::<T>::VerificationFailed.into())
            }
        }

    }
}


/// Computes Groth16 verify on alt_bn128 curve.
/// # Arguments
///
/// * `vkd` - verification key, struct of (alpha: G1, beta:G2, gamma:G2, delta:G2, ic:[G1])
/// * `proofd` - proof, struct of (a:G1, b:G2, c:G1)
/// * `inputd` - vector of imputs [Fr]
/// 
/// # Data types
/// G2 is Fr-ordered subgroup point (x:Fq2, y:Fq2) on alt_bn128 twist,
/// alt_bn128 twist is Y^2 = X^3 + 3/(i+9) curve over Fq2
/// Fq2 is complex field element (re: Fq, im: Fq)
/// G1 is point (x:Fq, y:Fq) on alt_bn128,
/// alt_bn128 is Y^2 = X^3 + 3 curve over Fq
/// Fq is LE-serialized u256 number lesser than 21888242871839275222246405745257275088696311157297823662689037894645226208583
/// Fr is LE-serialized u256 number lesser than 21888242871839275222246405745257275088548364400416034343698204186575808495617
fn groth16verify(vkd: VK, proofd: Proof, inputd: VU256) -> bool {
    // make verification
    let neg_a = alt_bn128_g1_neg(proofd.a);
    let acc_expr = vkd
        .ic
        .iter()
        .zip([U256::ONE].iter().chain(inputd.iter()))
        .map(|(&base, &exp)| (base, exp))
        .collect::<Vec<_>>();
    let acc = alt_bn128_g1_multiexp(&acc_expr);
    let pairing_expr = vec![
        (neg_a, proofd.b),
        (vkd.alpha, vkd.beta),
        (acc, vkd.gamma),
        (proofd.c, vkd.delta),
    ];
    alt_bn128_pairing_check(&pairing_expr)
}
// function to deserialize json field
pub fn de_string_to_bytes<'de, D>(de: D) -> Result<sp_std::prelude::Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(de)?;
    Ok(s.as_bytes().to_vec())
}

// groth16 verification functions (further calls to functions in  alt_b128.rs)
pub fn alt_bn128_g1_neg(p: G1) -> G1 {
    alt_bn128_g1_sum(&[(true, p)])
}
pub fn alt_bn128_g1_sum(v: &[(bool, G1)]) -> G1 {
    let data = v.try_to_vec().unwrap_or_default();
    let res = crate::alt_bn128::alt_bn128_g1_sum(&data).unwrap_or_default();
    let mut res_ptr = &res[..];
    <G1 as BorshDeserialize>::deserialize(&mut res_ptr).unwrap_or_default()
}
pub fn alt_bn128_g1_multiexp(v: &[(G1, U256)]) -> G1 {
    let data = v.try_to_vec().unwrap_or_default();
    let res = crate::alt_bn128::alt_bn128_g1_multiexp(&data).unwrap_or_default();
    let mut res_ptr = &res[..];
    <G1 as BorshDeserialize>::deserialize(&mut res_ptr).unwrap_or_default()
}
pub fn alt_bn128_pairing_check(v: &[(G1, G2)]) -> bool {
    let data = v.try_to_vec().unwrap_or_default();
    crate::alt_bn128::alt_bn128_pairing_check(&data).unwrap_or(false)
}

// Testing unit, execue by: cargo run test
#[cfg(test)]
mod tests {
    extern crate pretty_assertions;
    use super::*;
    use std::str;
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

    #[test]
    fn deserialize_json() {
        let jvkproofinput: Vec<u8>=br#"{"vk":"yO5EICtE+JVjTbRYkayI0Y/BoOJtE9lsrqeCTKTDnxD8UjB8B51wrVsQsVrsi6Uk0b2UKGfs3AJUX2Eud5wnET/ze/CsefR74bOn50BmVoExPPDiJGRD2IejItInd/wbtAH7GstyB1Q1j9uROBBAgE2eEj/cwRcHJobe4MP1mQIsiHdC5KXrBZFMlcRQi19O5pRHJ3fra+CNrckf5PHVL1NDT3E4ah/xGoRIbB0DfMSC2AO5NCyuZzJiAMBEKEcLbiazu9JOT65EXSc7HGM9IKlQXgpITR/jikWNxJc/Jyn6KiimTBN/yj4NVjAogElMdLmVoelMa0SAen8Z5ZwkFc6j3IriiWbKQnnkocqd++FqYs4gTh2rFDvcn2YpAhAmnMf35ssgfTFSIOyLZeRQPJ/SzCQMvSq8p1TAkgF85xv+1Vwd0UmrwJXyPVWhevfis0jEd6Cw78ESIMwB7S4dJwNAnVjEBRrKGfOAAzBIiTQRVMSMY2a1nMP/vr57eJwrOYvVboNDUHw8N+u1KoT3vTQkt6+bdeUBw2X/HBbeuyLcmx9AdsbJ0QY1GGF4cgGnSx9kGtcL9UY4qMWVtJ++LAQAAABZB9VFKNzCZgjPMZ9MTfotIL1czmkU9p4L3+6udM/DCAIGsaMeBAN/AhWI+GDLJK3EPzfiVDtw9PWWv+mifJUEQqRUa63wkfB2CouGxTpfsMPlZW93gzGXl5C4lmqMSQnAYpBIHANPM/R/DtA6eMTKKgKBfqgSMjf8YwlmfckmEkbsEZYwsUj2B+ryafp/qj39z80B/33p62Wz+OdwpcIYLSyprNYGC1nyO/jlRIhqRFhx9qkBRjKz/ddvFv7bdAeyPpjCqbT/6zrE22RSdm1I+tceC6xm3OUJE3wX4d5XF5z1EXo17iShXLdYhwVcd//YzyysetRirUxRPeXNAuAh","proof":"Qexag8d0jvm1IWZywscRBuvdSEvlGuhvVg5Qj97vhS5VFas06bgj/yXiuZ+yJ/WZWCYDYq8e5HZPITpoaHAvGckDPBplyUtn8zZ3UI4f5E1uLmxlehAkzVK33Fp8/SEZX4v8OLLT3MP/FWhDvS43u2sLvZcCstjVjbarImuLiSA0IW7UmNgG7u8x99JExO0pp0EAGJ3PiBOzyZ/PhxUPBXvOgxhwNzx0nzZzp+aSY8yhsWxFWRl6UWzmS6J/ieUS1q5Tjwq9gs4qcX6+Q9WWRpvYVboY+f4d6smQyryKdB5Hi5E8/jWGPoD9tFJDN4KVnnESrKi7fVjH6A3twUaQEw==","input":"AwAAAMI1CN4U9DnKW3soxArLClszrtTa/MGicksQVWpir/QNW/hp3N50wmjr1CUHvGP6u6WnrdK7oRDtSHgjcjmUVyr8NQtA06gcVk9m3KPdmWele0Bx9AcLpToixRb2FCx/JQ=="}"#.to_vec();
        let vkc = r#"yO5EICtE+JVjTbRYkayI0Y/BoOJtE9lsrqeCTKTDnxD8UjB8B51wrVsQsVrsi6Uk0b2UKGfs3AJUX2Eud5wnET/ze/CsefR74bOn50BmVoExPPDiJGRD2IejItInd/wbtAH7GstyB1Q1j9uROBBAgE2eEj/cwRcHJobe4MP1mQIsiHdC5KXrBZFMlcRQi19O5pRHJ3fra+CNrckf5PHVL1NDT3E4ah/xGoRIbB0DfMSC2AO5NCyuZzJiAMBEKEcLbiazu9JOT65EXSc7HGM9IKlQXgpITR/jikWNxJc/Jyn6KiimTBN/yj4NVjAogElMdLmVoelMa0SAen8Z5ZwkFc6j3IriiWbKQnnkocqd++FqYs4gTh2rFDvcn2YpAhAmnMf35ssgfTFSIOyLZeRQPJ/SzCQMvSq8p1TAkgF85xv+1Vwd0UmrwJXyPVWhevfis0jEd6Cw78ESIMwB7S4dJwNAnVjEBRrKGfOAAzBIiTQRVMSMY2a1nMP/vr57eJwrOYvVboNDUHw8N+u1KoT3vTQkt6+bdeUBw2X/HBbeuyLcmx9AdsbJ0QY1GGF4cgGnSx9kGtcL9UY4qMWVtJ++LAQAAABZB9VFKNzCZgjPMZ9MTfotIL1czmkU9p4L3+6udM/DCAIGsaMeBAN/AhWI+GDLJK3EPzfiVDtw9PWWv+mifJUEQqRUa63wkfB2CouGxTpfsMPlZW93gzGXl5C4lmqMSQnAYpBIHANPM/R/DtA6eMTKKgKBfqgSMjf8YwlmfckmEkbsEZYwsUj2B+ryafp/qj39z80B/33p62Wz+OdwpcIYLSyprNYGC1nyO/jlRIhqRFhx9qkBRjKz/ddvFv7bdAeyPpjCqbT/6zrE22RSdm1I+tceC6xm3OUJE3wX4d5XF5z1EXo17iShXLdYhwVcd//YzyysetRirUxRPeXNAuAh"#;
        let vkproofinput: Jsonvkproofinput =
            serde_json::from_slice(&jvkproofinput.as_slice()).unwrap();
        let vkcmp = match str::from_utf8(&vkproofinput.vk) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        assert_eq!(vkcmp, vkc);
    }
    #[test]
    fn failed_deserialize_json() {
        let jvkproofinput: Vec<u8>=br#"{"vk":"xO5EICtE+JVjTbRYkayI0Y/BoOJtE9lsrqeCTKTDnxD8UjB8B51wrVsQsVrsi6Uk0b2UKGfs3AJUX2Eud5wnET/ze/CsefR74bOn50BmVoExPPDiJGRD2IejItInd/wbtAH7GstyB1Q1j9uROBBAgE2eEj/cwRcHJobe4MP1mQIsiHdC5KXrBZFMlcRQi19O5pRHJ3fra+CNrckf5PHVL1NDT3E4ah/xGoRIbB0DfMSC2AO5NCyuZzJiAMBEKEcLbiazu9JOT65EXSc7HGM9IKlQXgpITR/jikWNxJc/Jyn6KiimTBN/yj4NVjAogElMdLmVoelMa0SAen8Z5ZwkFc6j3IriiWbKQnnkocqd++FqYs4gTh2rFDvcn2YpAhAmnMf35ssgfTFSIOyLZeRQPJ/SzCQMvSq8p1TAkgF85xv+1Vwd0UmrwJXyPVWhevfis0jEd6Cw78ESIMwB7S4dJwNAnVjEBRrKGfOAAzBIiTQRVMSMY2a1nMP/vr57eJwrOYvVboNDUHw8N+u1KoT3vTQkt6+bdeUBw2X/HBbeuyLcmx9AdsbJ0QY1GGF4cgGnSx9kGtcL9UY4qMWVtJ++LAQAAABZB9VFKNzCZgjPMZ9MTfotIL1czmkU9p4L3+6udM/DCAIGsaMeBAN/AhWI+GDLJK3EPzfiVDtw9PWWv+mifJUEQqRUa63wkfB2CouGxTpfsMPlZW93gzGXl5C4lmqMSQnAYpBIHANPM/R/DtA6eMTKKgKBfqgSMjf8YwlmfckmEkbsEZYwsUj2B+ryafp/qj39z80B/33p62Wz+OdwpcIYLSyprNYGC1nyO/jlRIhqRFhx9qkBRjKz/ddvFv7bdAeyPpjCqbT/6zrE22RSdm1I+tceC6xm3OUJE3wX4d5XF5z1EXo17iShXLdYhwVcd//YzyysetRirUxRPeXNAuAh","proof":"Qexag8d0jvm1IWZywscRBuvdSEvlGuhvVg5Qj97vhS5VFas06bgj/yXiuZ+yJ/WZWCYDYq8e5HZPITpoaHAvGckDPBplyUtn8zZ3UI4f5E1uLmxlehAkzVK33Fp8/SEZX4v8OLLT3MP/FWhDvS43u2sLvZcCstjVjbarImuLiSA0IW7UmNgG7u8x99JExO0pp0EAGJ3PiBOzyZ/PhxUPBXvOgxhwNzx0nzZzp+aSY8yhsWxFWRl6UWzmS6J/ieUS1q5Tjwq9gs4qcX6+Q9WWRpvYVboY+f4d6smQyryKdB5Hi5E8/jWGPoD9tFJDN4KVnnESrKi7fVjH6A3twUaQEw==","input":"AwAAAMI1CN4U9DnKW3soxArLClszrtTa/MGicksQVWpir/QNW/hp3N50wmjr1CUHvGP6u6WnrdK7oRDtSHgjcjmUVyr8NQtA06gcVk9m3KPdmWele0Bx9AcLpToixRb2FCx/JQ=="}"#.to_vec();
        let vkc = r#"yO5EICtE+JVjTbRYkayI0Y/BoOJtE9lsrqeCTKTDnxD8UjB8B51wrVsQsVrsi6Uk0b2UKGfs3AJUX2Eud5wnET/ze/CsefR74bOn50BmVoExPPDiJGRD2IejItInd/wbtAH7GstyB1Q1j9uROBBAgE2eEj/cwRcHJobe4MP1mQIsiHdC5KXrBZFMlcRQi19O5pRHJ3fra+CNrckf5PHVL1NDT3E4ah/xGoRIbB0DfMSC2AO5NCyuZzJiAMBEKEcLbiazu9JOT65EXSc7HGM9IKlQXgpITR/jikWNxJc/Jyn6KiimTBN/yj4NVjAogElMdLmVoelMa0SAen8Z5ZwkFc6j3IriiWbKQnnkocqd++FqYs4gTh2rFDvcn2YpAhAmnMf35ssgfTFSIOyLZeRQPJ/SzCQMvSq8p1TAkgF85xv+1Vwd0UmrwJXyPVWhevfis0jEd6Cw78ESIMwB7S4dJwNAnVjEBRrKGfOAAzBIiTQRVMSMY2a1nMP/vr57eJwrOYvVboNDUHw8N+u1KoT3vTQkt6+bdeUBw2X/HBbeuyLcmx9AdsbJ0QY1GGF4cgGnSx9kGtcL9UY4qMWVtJ++LAQAAABZB9VFKNzCZgjPMZ9MTfotIL1czmkU9p4L3+6udM/DCAIGsaMeBAN/AhWI+GDLJK3EPzfiVDtw9PWWv+mifJUEQqRUa63wkfB2CouGxTpfsMPlZW93gzGXl5C4lmqMSQnAYpBIHANPM/R/DtA6eMTKKgKBfqgSMjf8YwlmfckmEkbsEZYwsUj2B+ryafp/qj39z80B/33p62Wz+OdwpcIYLSyprNYGC1nyO/jlRIhqRFhx9qkBRjKz/ddvFv7bdAeyPpjCqbT/6zrE22RSdm1I+tceC6xm3OUJE3wX4d5XF5z1EXo17iShXLdYhwVcd//YzyysetRirUxRPeXNAuAh"#;
        let vkproofinput: Jsonvkproofinput =
            serde_json::from_slice(&jvkproofinput.as_slice()).unwrap();
        let vkcmp = match str::from_utf8(&vkproofinput.vk) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        assert!(!vkcmp.eq(vkc));
    }
    #[test]
    fn deserialize_borsh() {
        let jvkproofinput: Vec<u8>=br#"{"vk":"yO5EICtE+JVjTbRYkayI0Y/BoOJtE9lsrqeCTKTDnxD8UjB8B51wrVsQsVrsi6Uk0b2UKGfs3AJUX2Eud5wnET/ze/CsefR74bOn50BmVoExPPDiJGRD2IejItInd/wbtAH7GstyB1Q1j9uROBBAgE2eEj/cwRcHJobe4MP1mQIsiHdC5KXrBZFMlcRQi19O5pRHJ3fra+CNrckf5PHVL1NDT3E4ah/xGoRIbB0DfMSC2AO5NCyuZzJiAMBEKEcLbiazu9JOT65EXSc7HGM9IKlQXgpITR/jikWNxJc/Jyn6KiimTBN/yj4NVjAogElMdLmVoelMa0SAen8Z5ZwkFc6j3IriiWbKQnnkocqd++FqYs4gTh2rFDvcn2YpAhAmnMf35ssgfTFSIOyLZeRQPJ/SzCQMvSq8p1TAkgF85xv+1Vwd0UmrwJXyPVWhevfis0jEd6Cw78ESIMwB7S4dJwNAnVjEBRrKGfOAAzBIiTQRVMSMY2a1nMP/vr57eJwrOYvVboNDUHw8N+u1KoT3vTQkt6+bdeUBw2X/HBbeuyLcmx9AdsbJ0QY1GGF4cgGnSx9kGtcL9UY4qMWVtJ++LAQAAABZB9VFKNzCZgjPMZ9MTfotIL1czmkU9p4L3+6udM/DCAIGsaMeBAN/AhWI+GDLJK3EPzfiVDtw9PWWv+mifJUEQqRUa63wkfB2CouGxTpfsMPlZW93gzGXl5C4lmqMSQnAYpBIHANPM/R/DtA6eMTKKgKBfqgSMjf8YwlmfckmEkbsEZYwsUj2B+ryafp/qj39z80B/33p62Wz+OdwpcIYLSyprNYGC1nyO/jlRIhqRFhx9qkBRjKz/ddvFv7bdAeyPpjCqbT/6zrE22RSdm1I+tceC6xm3OUJE3wX4d5XF5z1EXo17iShXLdYhwVcd//YzyysetRirUxRPeXNAuAh","proof":"Qexag8d0jvm1IWZywscRBuvdSEvlGuhvVg5Qj97vhS5VFas06bgj/yXiuZ+yJ/WZWCYDYq8e5HZPITpoaHAvGckDPBplyUtn8zZ3UI4f5E1uLmxlehAkzVK33Fp8/SEZX4v8OLLT3MP/FWhDvS43u2sLvZcCstjVjbarImuLiSA0IW7UmNgG7u8x99JExO0pp0EAGJ3PiBOzyZ/PhxUPBXvOgxhwNzx0nzZzp+aSY8yhsWxFWRl6UWzmS6J/ieUS1q5Tjwq9gs4qcX6+Q9WWRpvYVboY+f4d6smQyryKdB5Hi5E8/jWGPoD9tFJDN4KVnnESrKi7fVjH6A3twUaQEw==","input":"AwAAAMI1CN4U9DnKW3soxArLClszrtTa/MGicksQVWpir/QNW/hp3N50wmjr1CUHvGP6u6WnrdK7oRDtSHgjcjmUVyr8NQtA06gcVk9m3KPdmWele0Bx9AcLpToixRb2FCx/JQ=="}"#.to_vec();
        let _vkc = r#"yO5EICtE+JVjTbRYkayI0Y/BoOJtE9lsrqeCTKTDnxD8UjB8B51wrVsQsVrsi6Uk0b2UKGfs3AJUX2Eud5wnET/ze/CsefR74bOn50BmVoExPPDiJGRD2IejItInd/wbtAH7GstyB1Q1j9uROBBAgE2eEj/cwRcHJobe4MP1mQIsiHdC5KXrBZFMlcRQi19O5pRHJ3fra+CNrckf5PHVL1NDT3E4ah/xGoRIbB0DfMSC2AO5NCyuZzJiAMBEKEcLbiazu9JOT65EXSc7HGM9IKlQXgpITR/jikWNxJc/Jyn6KiimTBN/yj4NVjAogElMdLmVoelMa0SAen8Z5ZwkFc6j3IriiWbKQnnkocqd++FqYs4gTh2rFDvcn2YpAhAmnMf35ssgfTFSIOyLZeRQPJ/SzCQMvSq8p1TAkgF85xv+1Vwd0UmrwJXyPVWhevfis0jEd6Cw78ESIMwB7S4dJwNAnVjEBRrKGfOAAzBIiTQRVMSMY2a1nMP/vr57eJwrOYvVboNDUHw8N+u1KoT3vTQkt6+bdeUBw2X/HBbeuyLcmx9AdsbJ0QY1GGF4cgGnSx9kGtcL9UY4qMWVtJ++LAQAAABZB9VFKNzCZgjPMZ9MTfotIL1czmkU9p4L3+6udM/DCAIGsaMeBAN/AhWI+GDLJK3EPzfiVDtw9PWWv+mifJUEQqRUa63wkfB2CouGxTpfsMPlZW93gzGXl5C4lmqMSQnAYpBIHANPM/R/DtA6eMTKKgKBfqgSMjf8YwlmfckmEkbsEZYwsUj2B+ryafp/qj39z80B/33p62Wz+OdwpcIYLSyprNYGC1nyO/jlRIhqRFhx9qkBRjKz/ddvFv7bdAeyPpjCqbT/6zrE22RSdm1I+tceC6xm3OUJE3wX4d5XF5z1EXo17iShXLdYhwVcd//YzyysetRirUxRPeXNAuAh"#;
        let vkproofinput: Jsonvkproofinput =
            serde_json::from_slice(&jvkproofinput.as_slice()).unwrap();
        let _vkcmp = match str::from_utf8(&vkproofinput.vk) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        let _vkstorage = vkproofinput.vk.clone();
        let vk = base64::decode(vkproofinput.vk).unwrap_or(vec![]);
        let _vkd = VK::try_from_slice(&vk).unwrap_or(VK::default());
        let proof = base64::decode(vkproofinput.proof).unwrap_or(vec![]);
        let _proofd = Proof::try_from_slice(&proof).unwrap_or(Proof::default());
        let input = base64::decode(vkproofinput.input).unwrap_or(vec![]);
        let _inputd = VU256::try_from_slice(&input).unwrap_or(VU256::default());
    }
    #[test]
    fn failed_deserialize_borsh() {
        let jvkproofinput: Vec<u8>=br#"{"vk":"xO5EICtE+JVjTbRYkayI0Y/BoOJtE9lsrqeCTKTDnxD8UjB8B51wrVsQsVrsi6Uk0b2UKGfs3AJUX2Eud5wnET/ze/CsefR74bOn50BmVoExPPDiJGRD2IejItInd/wbtAH7GstyB1Q1j9uROBBAgE2eEj/cwRcHJobe4MP1mQIsiHdC5KXrBZFMlcRQi19O5pRHJ3fra+CNrckf5PHVL1NDT3E4ah/xGoRIbB0DfMSC2AO5NCyuZzJiAMBEKEcLbiazu9JOT65EXSc7HGM9IKlQXgpITR/jikWNxJc/Jyn6KiimTBN/yj4NVjAogElMdLmVoelMa0SAen8Z5ZwkFc6j3IriiWbKQnnkocqd++FqYs4gTh2rFDvcn2YpAhAmnMf35ssgfTFSIOyLZeRQPJ/SzCQMvSq8p1TAkgF85xv+1Vwd0UmrwJXyPVWhevfis0jEd6Cw78ESIMwB7S4dJwNAnVjEBRrKGfOAAzBIiTQRVMSMY2a1nMP/vr57eJwrOYvVboNDUHw8N+u1KoT3vTQkt6+bdeUBw2X/HBbeuyLcmx9AdsbJ0QY1GGF4cgGnSx9kGtcL9UY4qMWVtJ++LAQAAABZB9VFKNzCZgjPMZ9MTfotIL1czmkU9p4L3+6udM/DCAIGsaMeBAN/AhWI+GDLJK3EPzfiVDtw9PWWv+mifJUEQqRUa63wkfB2CouGxTpfsMPlZW93gzGXl5C4lmqMSQnAYpBIHANPM/R/DtA6eMTKKgKBfqgSMjf8YwlmfckmEkbsEZYwsUj2B+ryafp/qj39z80B/33p62Wz+OdwpcIYLSyprNYGC1nyO/jlRIhqRFhx9qkBRjKz/ddvFv7bdAeyPpjCqbT/6zrE22RSdm1I+tceC6xm3OUJE3wX4d5XF5z1EXo17iShXLdYhwVcd//YzyysetRirUxRPeXNAuAh","proof":"Qexag8d0jvm1IWZywscRBuvdSEvlGuhvVg5Qj97vhS5VFas06bgj/yXiuZ+yJ/WZWCYDYq8e5HZPITpoaHAvGckDPBplyUtn8zZ3UI4f5E1uLmxlehAkzVK33Fp8/SEZX4v8OLLT3MP/FWhDvS43u2sLvZcCstjVjbarImuLiSA0IW7UmNgG7u8x99JExO0pp0EAGJ3PiBOzyZ/PhxUPBXvOgxhwNzx0nzZzp+aSY8yhsWxFWRl6UWzmS6J/ieUS1q5Tjwq9gs4qcX6+Q9WWRpvYVboY+f4d6smQyryKdB5Hi5E8/jWGPoD9tFJDN4KVnnESrKi7fVjH6A3twUaQEw==","input":"AwAAAMI1CN4U9DnKW3soxArLClszrtTa/MGicksQVWpir/QNW/hp3N50wmjr1CUHvGP6u6WnrdK7oRDtSHgjcjmUVyr8NQtA06gcVk9m3KPdmWele0Bx9AcLpToixRb2FCx/JQ=="}"#.to_vec();
        let _vkc = r#"yO5EICtE+JVjTbRYkayI0Y/BoOJtE9lsrqeCTKTDnxD8UjB8B51wrVsQsVrsi6Uk0b2UKGfs3AJUX2Eud5wnET/ze/CsefR74bOn50BmVoExPPDiJGRD2IejItInd/wbtAH7GstyB1Q1j9uROBBAgE2eEj/cwRcHJobe4MP1mQIsiHdC5KXrBZFMlcRQi19O5pRHJ3fra+CNrckf5PHVL1NDT3E4ah/xGoRIbB0DfMSC2AO5NCyuZzJiAMBEKEcLbiazu9JOT65EXSc7HGM9IKlQXgpITR/jikWNxJc/Jyn6KiimTBN/yj4NVjAogElMdLmVoelMa0SAen8Z5ZwkFc6j3IriiWbKQnnkocqd++FqYs4gTh2rFDvcn2YpAhAmnMf35ssgfTFSIOyLZeRQPJ/SzCQMvSq8p1TAkgF85xv+1Vwd0UmrwJXyPVWhevfis0jEd6Cw78ESIMwB7S4dJwNAnVjEBRrKGfOAAzBIiTQRVMSMY2a1nMP/vr57eJwrOYvVboNDUHw8N+u1KoT3vTQkt6+bdeUBw2X/HBbeuyLcmx9AdsbJ0QY1GGF4cgGnSx9kGtcL9UY4qMWVtJ++LAQAAABZB9VFKNzCZgjPMZ9MTfotIL1czmkU9p4L3+6udM/DCAIGsaMeBAN/AhWI+GDLJK3EPzfiVDtw9PWWv+mifJUEQqRUa63wkfB2CouGxTpfsMPlZW93gzGXl5C4lmqMSQnAYpBIHANPM/R/DtA6eMTKKgKBfqgSMjf8YwlmfckmEkbsEZYwsUj2B+ryafp/qj39z80B/33p62Wz+OdwpcIYLSyprNYGC1nyO/jlRIhqRFhx9qkBRjKz/ddvFv7bdAeyPpjCqbT/6zrE22RSdm1I+tceC6xm3OUJE3wX4d5XF5z1EXo17iShXLdYhwVcd//YzyysetRirUxRPeXNAuAh"#;
        let vkproofinput: Jsonvkproofinput =
            serde_json::from_slice(&jvkproofinput.as_slice()).unwrap();
        let _vkcmp = match str::from_utf8(&vkproofinput.vk) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };
        let vkstorage = vkproofinput.vk.clone();
        let vk = base64::decode(vkstorage).unwrap();
        let vkd = VK::try_from_slice(&vk).unwrap();
        let vkstoragec = vkproofinput.vk.clone();
        let vkc = base64::decode(vkstoragec).unwrap();
        let vkdc = VK::try_from_slice(&vkc).unwrap();
        assert!(vkc.eq(&vk));
    }
    #[test]
    fn groth16_verification() {
        let jvkproofinput: Vec<u8>=br#"{"vk":"yO5EICtE+JVjTbRYkayI0Y/BoOJtE9lsrqeCTKTDnxD8UjB8B51wrVsQsVrsi6Uk0b2UKGfs3AJUX2Eud5wnET/ze/CsefR74bOn50BmVoExPPDiJGRD2IejItInd/wbtAH7GstyB1Q1j9uROBBAgE2eEj/cwRcHJobe4MP1mQIsiHdC5KXrBZFMlcRQi19O5pRHJ3fra+CNrckf5PHVL1NDT3E4ah/xGoRIbB0DfMSC2AO5NCyuZzJiAMBEKEcLbiazu9JOT65EXSc7HGM9IKlQXgpITR/jikWNxJc/Jyn6KiimTBN/yj4NVjAogElMdLmVoelMa0SAen8Z5ZwkFc6j3IriiWbKQnnkocqd++FqYs4gTh2rFDvcn2YpAhAmnMf35ssgfTFSIOyLZeRQPJ/SzCQMvSq8p1TAkgF85xv+1Vwd0UmrwJXyPVWhevfis0jEd6Cw78ESIMwB7S4dJwNAnVjEBRrKGfOAAzBIiTQRVMSMY2a1nMP/vr57eJwrOYvVboNDUHw8N+u1KoT3vTQkt6+bdeUBw2X/HBbeuyLcmx9AdsbJ0QY1GGF4cgGnSx9kGtcL9UY4qMWVtJ++LAQAAABZB9VFKNzCZgjPMZ9MTfotIL1czmkU9p4L3+6udM/DCAIGsaMeBAN/AhWI+GDLJK3EPzfiVDtw9PWWv+mifJUEQqRUa63wkfB2CouGxTpfsMPlZW93gzGXl5C4lmqMSQnAYpBIHANPM/R/DtA6eMTKKgKBfqgSMjf8YwlmfckmEkbsEZYwsUj2B+ryafp/qj39z80B/33p62Wz+OdwpcIYLSyprNYGC1nyO/jlRIhqRFhx9qkBRjKz/ddvFv7bdAeyPpjCqbT/6zrE22RSdm1I+tceC6xm3OUJE3wX4d5XF5z1EXo17iShXLdYhwVcd//YzyysetRirUxRPeXNAuAh","proof":"Qexag8d0jvm1IWZywscRBuvdSEvlGuhvVg5Qj97vhS5VFas06bgj/yXiuZ+yJ/WZWCYDYq8e5HZPITpoaHAvGckDPBplyUtn8zZ3UI4f5E1uLmxlehAkzVK33Fp8/SEZX4v8OLLT3MP/FWhDvS43u2sLvZcCstjVjbarImuLiSA0IW7UmNgG7u8x99JExO0pp0EAGJ3PiBOzyZ/PhxUPBXvOgxhwNzx0nzZzp+aSY8yhsWxFWRl6UWzmS6J/ieUS1q5Tjwq9gs4qcX6+Q9WWRpvYVboY+f4d6smQyryKdB5Hi5E8/jWGPoD9tFJDN4KVnnESrKi7fVjH6A3twUaQEw==","input":"AwAAAMI1CN4U9DnKW3soxArLClszrtTa/MGicksQVWpir/QNW/hp3N50wmjr1CUHvGP6u6WnrdK7oRDtSHgjcjmUVyr8NQtA06gcVk9m3KPdmWele0Bx9AcLpToixRb2FCx/JQ=="}"#.to_vec();
        let mut _vkproofinput: Jsonvkproofinput = Jsonvkproofinput::default();
        let vkproofinput: Jsonvkproofinput =
            serde_json::from_slice(&jvkproofinput.as_slice()).unwrap();
        let _vkstorage = vkproofinput.vk.clone();
        let vk = base64::decode(vkproofinput.vk).unwrap_or(vec![]);
        let vkd = VK::try_from_slice(&vk).unwrap_or(VK::default());
        let proof = base64::decode(vkproofinput.proof).unwrap_or(vec![]);
        let proofd = Proof::try_from_slice(&proof).unwrap_or(Proof::default());
        let input = base64::decode(vkproofinput.input).unwrap_or(vec![]);
        let inputd = VU256::try_from_slice(&input).unwrap_or(VU256::default());
        let neg_a = alt_bn128_g1_neg(proofd.a);
        let acc_expr = vkd
            .ic
            .iter()
            .zip([U256::ONE].iter().chain(inputd.iter()))
            .map(|(&base, &exp)| (base, exp))
            .collect::<Vec<_>>();
        let acc = alt_bn128_g1_multiexp(&acc_expr);
        let pairing_expr = vec![
            (neg_a, proofd.b),
            (vkd.alpha, vkd.beta),
            (acc, vkd.gamma),
            (proofd.c, vkd.delta),
        ];
        let verification: bool = alt_bn128_pairing_check(&pairing_expr);
        assert_eq!(verification, true);
    }
    #[test]
    fn failed_groth16_verification() {
        let jvkproofinput: Vec<u8>=br#"{"vk":"XO5EICtE+JVjTbRYkayI0Y/BoOJtE9lsrqeCTKTDnxD8UjB8B51wrVsQsVrsi6Uk0b2UKGfs3AJUX2Eud5wnET/ze/CsefR74bOn50BmVoExPPDiJGRD2IejItInd/wbtAH7GstyB1Q1j9uROBBAgE2eEj/cwRcHJobe4MP1mQIsiHdC5KXrBZFMlcRQi19O5pRHJ3fra+CNrckf5PHVL1NDT3E4ah/xGoRIbB0DfMSC2AO5NCyuZzJiAMBEKEcLbiazu9JOT65EXSc7HGM9IKlQXgpITR/jikWNxJc/Jyn6KiimTBN/yj4NVjAogElMdLmVoelMa0SAen8Z5ZwkFc6j3IriiWbKQnnkocqd++FqYs4gTh2rFDvcn2YpAhAmnMf35ssgfTFSIOyLZeRQPJ/SzCQMvSq8p1TAkgF85xv+1Vwd0UmrwJXyPVWhevfis0jEd6Cw78ESIMwB7S4dJwNAnVjEBRrKGfOAAzBIiTQRVMSMY2a1nMP/vr57eJwrOYvVboNDUHw8N+u1KoT3vTQkt6+bdeUBw2X/HBbeuyLcmx9AdsbJ0QY1GGF4cgGnSx9kGtcL9UY4qMWVtJ++LAQAAABZB9VFKNzCZgjPMZ9MTfotIL1czmkU9p4L3+6udM/DCAIGsaMeBAN/AhWI+GDLJK3EPzfiVDtw9PWWv+mifJUEQqRUa63wkfB2CouGxTpfsMPlZW93gzGXl5C4lmqMSQnAYpBIHANPM/R/DtA6eMTKKgKBfqgSMjf8YwlmfckmEkbsEZYwsUj2B+ryafp/qj39z80B/33p62Wz+OdwpcIYLSyprNYGC1nyO/jlRIhqRFhx9qkBRjKz/ddvFv7bdAeyPpjCqbT/6zrE22RSdm1I+tceC6xm3OUJE3wX4d5XF5z1EXo17iShXLdYhwVcd//YzyysetRirUxRPeXNAuAh","proof":"Qexag8d0jvm1IWZywscRBuvdSEvlGuhvVg5Qj97vhS5VFas06bgj/yXiuZ+yJ/WZWCYDYq8e5HZPITpoaHAvGckDPBplyUtn8zZ3UI4f5E1uLmxlehAkzVK33Fp8/SEZX4v8OLLT3MP/FWhDvS43u2sLvZcCstjVjbarImuLiSA0IW7UmNgG7u8x99JExO0pp0EAGJ3PiBOzyZ/PhxUPBXvOgxhwNzx0nzZzp+aSY8yhsWxFWRl6UWzmS6J/ieUS1q5Tjwq9gs4qcX6+Q9WWRpvYVboY+f4d6smQyryKdB5Hi5E8/jWGPoD9tFJDN4KVnnESrKi7fVjH6A3twUaQEw==","input":"AwAAAMI1CN4U9DnKW3soxArLClszrtTa/MGicksQVWpir/QNW/hp3N50wmjr1CUHvGP6u6WnrdK7oRDtSHgjcjmUVyr8NQtA06gcVk9m3KPdmWele0Bx9AcLpToixRb2FCx/JQ=="}"#.to_vec();
        let mut _vkproofinput: Jsonvkproofinput = Jsonvkproofinput::default();
        let vkproofinput: Jsonvkproofinput =
            serde_json::from_slice(&jvkproofinput.as_slice()).unwrap();
        let _vkstorage = vkproofinput.vk.clone();
        let vk = base64::decode(vkproofinput.vk).unwrap_or(vec![]);
        let vkd = VK::try_from_slice(&vk).unwrap_or(VK::default());
        let proof = base64::decode(vkproofinput.proof).unwrap_or(vec![]);
        let proofd = Proof::try_from_slice(&proof).unwrap_or(Proof::default());
        let input = base64::decode(vkproofinput.input).unwrap_or(vec![]);
        let inputd = VU256::try_from_slice(&input).unwrap_or(VU256::default());
        let neg_a = alt_bn128_g1_neg(proofd.a);
        let acc_expr = vkd
            .ic
            .iter()
            .zip([U256::ONE].iter().chain(inputd.iter()))
            .map(|(&base, &exp)| (base, exp))
            .collect::<Vec<_>>();
        let acc = alt_bn128_g1_multiexp(&acc_expr);
        let pairing_expr = vec![
            (neg_a, proofd.b),
            (vkd.alpha, vkd.beta),
            (acc, vkd.gamma),
            (proofd.c, vkd.delta),
        ];
        let verification: bool = alt_bn128_pairing_check(&pairing_expr);
        assert!(!verification);
    }
}
