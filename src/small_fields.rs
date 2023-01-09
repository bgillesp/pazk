use ark_ff::biginteger::BigInteger64;
use ark_ff::fields::{Fp64,MontConfig,MontBackend,FpConfig};

#[derive(MontConfig)]
#[modulus="5"]
#[generator="2"]
pub struct F5Config {}
pub type F5 = Fp64<MontBackend<F5Config, 1>>;

#[derive(MontConfig)]
#[modulus="13"]
#[generator="2"]
pub struct F13Config {}
pub type F13 = Fp64<MontBackend<F13Config, 1>>;

#[derive(MontConfig)]
#[modulus="251"]
#[generator="6"]
pub struct F251Config {}
pub type F251 = Fp64<MontBackend<F251Config, 1>>;

pub fn to_u64<T: FpConfig<1>> (n: Fp64<T>) -> u64 {
    bigint64_value(T::into_bigint(n))
}

fn bigint64_value(n: BigInteger64) -> u64 {
    n.0[0]
}
