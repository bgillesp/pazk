use std::iter::zip;

use ark_std::UniformRand;
use ark_ec::Group;

use pazk::small_curves::C17Projective as G;

use pazk::group_utils;

fn main() {
    let mut rng = rand::thread_rng();
    let n: usize = 3;

    let gens: Vec<G> = group_utils::rand_gens(n, &mut rng);
    println!("Generators:");
    for g in &gens {
        println!("{g}");
    }

    let data: Vec<<G as Group>::ScalarField> = (0..n).map(|_| <G as Group>::ScalarField::rand(&mut rng)).collect();
    println!("\nData:");
    for d in &data {
        println!("{d}");
    }

    let commitment: G = group_utils::msm(&gens, &data);
    println!("\nCommitment:");
    let terms: Vec<String> = zip(gens.iter(), data.iter())
            .map(|(g, d)| format!("{}*{}", d.to_string(), g.to_string()))
            .collect();
    let expr: String = terms.join(" + ");
    println!("{}", commitment);
    println!(" = {}", expr);
}

