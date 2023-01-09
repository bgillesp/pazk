use ark_ff::{Zero,One};
use ark_ff::fields::{Field,PrimeField,FftField};

use pazk::small_fields as sf;
use pazk::small_fields::F13 as F;

fn main() {
    println!("Exercising prime field F with modulus {} and generator {}",
        F::MODULUS.as_ref()[0],
        sf::to_u64(F::GENERATOR),
    );

    println!("\nField computations:");
    let zero = F::zero();
    println!("  0:           {}", sf::to_u64(zero));
    let one = F::one();
    println!("  1:           {}", sf::to_u64(one));
    println!(" -1:           {}", sf::to_u64(-one));
    let g = F::GENERATOR;
    println!("  g:           {}", sf::to_u64(g));
    for e in 2..10u64 {
        println!("  g^{}:         {}", e, sf::to_u64(g.pow([e])));
    }
    let two: F = F::from(2);
    println!("  2:           {}", sf::to_u64(two));
    println!("  1+1:         {}", sf::to_u64(one+one));
    let three: F = F::from(3);
    println!("  3:           {}", sf::to_u64(three));
    println!("  2+1:         {}", sf::to_u64(two+one));
    println!("  3+0:         {}", sf::to_u64(three+zero));

    println!("\nPrimeField params:");
    println!("  F::MODULUS = {}", F::MODULUS.as_ref()[0]);
    println!("  F::MODULUS_MINUS_ONE_DIV_TWO = {}", F::MODULUS_MINUS_ONE_DIV_TWO.as_ref()[0]);
    println!("  F::MODULUS_BIT_SIZE = {}", F::MODULUS_BIT_SIZE);
    println!("  F::TRACE = {}", F::TRACE);
    println!("  F::TRACE_MINUS_ONE_DIV_TWO = {}", F::TRACE_MINUS_ONE_DIV_TWO);

    println!("\nFftField params:");
    println!("  F::GENERATOR = {}", F::GENERATOR);
    println!("  F::TWO_ADICITY = {}", F::TWO_ADICITY);
    println!("  F::TWO_ADIC_ROOT_OF_UNITY = {}", F::TWO_ADIC_ROOT_OF_UNITY);
    println!("  F::SMALL_SUBGROUP_BASE = {:?}", F::SMALL_SUBGROUP_BASE);
    println!("  F::SMALL_SUBGROUP_BASE_ADICITY = {:?}", F::SMALL_SUBGROUP_BASE_ADICITY);
    println!("  F::LARGE_SUBGROUP_ROOT_OF_UNITY = {:?}", F::LARGE_SUBGROUP_ROOT_OF_UNITY);

    println!("\nMontConfig params:");
    println!("  F::R = {}", F::R.as_ref()[0]);
    println!("  F::R2 = {}", F::R2.as_ref()[0]);
    println!("  F::INV = {}", F::INV);
}
