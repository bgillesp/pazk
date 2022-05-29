use ark_ff::{Zero,One};
use ark_ff::fields::{Field,PrimeField,FpParameters,FftField,FftParameters};

use pazk::small_fields as sf;
use pazk::small_fields::{F13 as F, F13Parameters as FieldParameters};

fn main() {
    println!("Exercising field F_{} with generator {}",
        FieldParameters::MODULUS.as_ref()[0],
        sf::to_u64(F::multiplicative_generator()),
    );

    println!("\nField computations:");
    let zero = F::zero();
    println!("  0:           {}", sf::to_u64(zero));
    let one = F::one();
    println!("  1:           {}", sf::to_u64(one));
    println!(" -1:           {}", sf::to_u64(-one));
    let g = F::multiplicative_generator();
    println!("  g:           {}", sf::to_u64(g));
    for e in 2..10u64 {
        println!("  g^{}:         {}", e, sf::to_u64(g.pow([e])));
    }
    let two: F = sf::from_u64(2);
    println!("  2:           {}", sf::to_u64(two));
    println!("  1+1:         {}", sf::to_u64(one+one));
    let three: F = sf::from_u64(3);
    println!("  3:           {}", sf::to_u64(three));
    println!("  2+1:         {}", sf::to_u64(two+one));
    println!("  3+0:         {}", sf::to_u64(three+zero));

    println!("\nConfig params:");
    println!("  Fp::MODULUS = {}", FieldParameters::MODULUS.as_ref()[0]);
    println!("  Fp::MODULUS_MINUS_ONE_DIV_TWO = {}", FieldParameters::MODULUS_MINUS_ONE_DIV_TWO.as_ref()[0]);
    println!("  Fp::GENERATOR = {}", FieldParameters::GENERATOR);
    println!("  Fp::R = {}", FieldParameters::R.as_ref()[0]);
    println!("  Fp::R2 = {}", FieldParameters::R2.as_ref()[0]);
    println!("  Fp::INV = {}", FieldParameters::INV);
    println!("  Fp::MODULUS_BITS = {}", FieldParameters::MODULUS_BITS);
    println!("  Fp::CAPACITY = {}", FieldParameters::CAPACITY);
    println!("  Fp::REPR_SHAVE_BITS = {}", FieldParameters::REPR_SHAVE_BITS);
    println!("  Fp::T = {}", FieldParameters::T.as_ref()[0]);
    println!("  Fp::T_MINUS_ONE_DIV_TWO = {}", FieldParameters::T_MINUS_ONE_DIV_TWO.as_ref()[0]);
    println!("  Fft::TWO_ADICITY = {}", FieldParameters::TWO_ADICITY);
    println!("  Fft::TWO_ADIC_ROOT_OF_UNITY = {}", FieldParameters::TWO_ADIC_ROOT_OF_UNITY);
    println!("  Fft::SMALL_SUBGROUP_BASE = {:?}", FieldParameters::SMALL_SUBGROUP_BASE);
    println!("  Fft::SMALL_SUBGROUP_BASE_ADICITY = {:?}", FieldParameters::SMALL_SUBGROUP_BASE_ADICITY);
    println!("  Fft::LARGE_SUBGROUP_ROOT_OF_UNITY = {:?}", FieldParameters::LARGE_SUBGROUP_ROOT_OF_UNITY);

    println!("\nCalculated params:");
    println!("  Modulus minus one divided by two: {}", F::modulus_minus_one_div_two());
    println!("  Trace: {}", F::trace());
    println!("  QNR^T: {}", sf::to_u64(F::qnr_to_t()));
    println!("  Size in bits: {}", F::size_in_bits());
    println!("  Two-adic root of unity: {}", sf::to_u64(F::two_adic_root_of_unity()));
}
