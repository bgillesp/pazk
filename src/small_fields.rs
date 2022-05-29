use ark_ff::biginteger::BigInteger64;
use ark_ff::fields::{PrimeField,FpParameters,FftParameters,Fp64,Fp64Parameters};

pub type F5  = Fp64<F5Parameters>;
pub type F13 = Fp64<F13Parameters>;
pub type F251 = Fp64<F251Parameters>;

pub struct F5Parameters {}
impl FftParameters for F5Parameters {
    type BigInt = BigInteger64;

    const TWO_ADICITY: u32
        = 2u32;
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger64
        = BigInteger64::new([2u64]); // montgomery form of 2 = 2*1 % 5
    const SMALL_SUBGROUP_BASE: Option<u32>
        = None;
    const SMALL_SUBGROUP_BASE_ADICITY: Option<u32>
        = None;
    const LARGE_SUBGROUP_ROOT_OF_UNITY: Option<BigInteger64>
        = None;
}
impl FpParameters for F5Parameters {
    const MODULUS: BigInteger64
        = BigInteger64::new([5u64]);
    const MODULUS_BITS: u32
        = 3u32;
    const REPR_SHAVE_BITS: u32
        = 61u32;
    const R: BigInteger64
        = BigInteger64::new([1u64]);
    const R2: BigInteger64
        = BigInteger64::new([1u64]);
    const INV: u64
        = 3689348814741910323u64;
    const GENERATOR: BigInteger64
        = BigInteger64::new([2u64]); // montgomery form of 2 = 2*1 % 5
    const CAPACITY: u32
        = 2u32;
    const T: BigInteger64
        = BigInteger64::new([1u64]);
    const T_MINUS_ONE_DIV_TWO: BigInteger64
        = BigInteger64::new([0u64]);
    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger64
        = BigInteger64::new([2u64]);
}
impl Fp64Parameters for F5Parameters {}

pub struct F13Parameters {}
impl FftParameters for F13Parameters {
    type BigInt = BigInteger64;

    const TWO_ADICITY: u32
        = 2u32;
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger64
        = BigInteger64::new([11u64]); // montgomery form of 8 = 8*3 % 13
    const SMALL_SUBGROUP_BASE: Option<u32>
        = Some(3u32);
    const SMALL_SUBGROUP_BASE_ADICITY: Option<u32>
        = Some(1u32);
    const LARGE_SUBGROUP_ROOT_OF_UNITY: Option<BigInteger64>
        = Some(BigInteger64::new([6u64])); // montgomery form of 2 = 2*3 % 13
}
impl FpParameters for F13Parameters {
    const MODULUS: BigInteger64
        = BigInteger64::new([13u64]);
    const MODULUS_BITS: u32
        = 4u32;
    const REPR_SHAVE_BITS: u32
        = 60u32;
    const R: BigInteger64
        = BigInteger64::new([3u64]);
    const R2: BigInteger64
        = BigInteger64::new([9u64]);
    const INV: u64
        = 12770822820260458811u64;
    const GENERATOR: BigInteger64
        = BigInteger64::new([6u64]); // montgomery form of 2 = 2*3 % 13
    const CAPACITY: u32
        = 3u32;
    const T: BigInteger64
        = BigInteger64::new([3u64]);
    const T_MINUS_ONE_DIV_TWO: BigInteger64
        = BigInteger64::new([1u64]);
    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger64
        = BigInteger64::new([6u64]);
}
impl Fp64Parameters for F13Parameters {}

pub struct F251Parameters {}
impl FftParameters for F251Parameters {
    type BigInt = BigInteger64;

    const TWO_ADICITY: u32
        = 1u32;
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger64
        = BigInteger64::new([182u64]); // montgomery form of 250 = 250*69 % 251
    const SMALL_SUBGROUP_BASE: Option<u32>
        = Some(5u32);
    const SMALL_SUBGROUP_BASE_ADICITY: Option<u32>
        = Some(3u32);
    const LARGE_SUBGROUP_ROOT_OF_UNITY: Option<BigInteger64>
        = Some(BigInteger64::new([163u64])); // montgomery form of 6 = 6*69 % 251
}
impl FpParameters for F251Parameters {
    const MODULUS: BigInteger64
        = BigInteger64::new([251u64]);
    const MODULUS_BITS: u32
        = 8u32;
    const REPR_SHAVE_BITS: u32
        = 56u32;
    const R: BigInteger64
        = BigInteger64::new([69u64]);
    const R2: BigInteger64
        = BigInteger64::new([243u64]);
    const INV: u64
        = 15507023902600459725u64;
    const GENERATOR: BigInteger64
        = BigInteger64::new([163u64]); // montgomery form of 6 = 6*69 % 251
    const CAPACITY: u32
        = 7u32;
    const T: BigInteger64
        = BigInteger64::new([125u64]);
    const T_MINUS_ONE_DIV_TWO: BigInteger64
        = BigInteger64::new([62u64]);
    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger64
        = BigInteger64::new([125u64]);
}
impl Fp64Parameters for F251Parameters {}

pub fn from_u64<T: Fp64Parameters> (n: u64) -> Fp64<T> {
    let n_mod = n % bigint64_value(&T::MODULUS);
    Fp64::<T>::from_repr(BigInteger64::new([n_mod])).unwrap()
}

pub fn to_u64<T: Fp64Parameters> (n: Fp64<T>) -> u64 {
    bigint64_value(&n.into_repr())
}

fn bigint64_value(n: &BigInteger64) -> u64 {
    n.0[0]
}
