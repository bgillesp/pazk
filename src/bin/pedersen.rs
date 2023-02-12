use std::iter::zip;
use rand::Rng;

use ark_std::UniformRand;
use ark_ec::Group;

use pazk::small_curves::C17Projective as G;

fn main() {
	let mut rng = rand::thread_rng();
    let n: usize = 3;

    let gens: Vec<G> = rand_gens(n, &mut rng);
    println!("Generators:");
    for g in &gens {
    	println!("{g}");
    }

    let data: Vec<<G as Group>::ScalarField> = (0..n).map(|_| <G as Group>::ScalarField::rand(&mut rng)).collect();
    println!("\nData:");
    for d in &data {
		println!("{d}");
    }

    let commitment: G = multi_exponent(&gens, &data);
    println!("\nCommitment:");
    let terms: Vec<String> = zip(gens.iter(), data.iter())
    		.map(|(g, d)| format!("{}*{}", d.to_string(), g.to_string()))
    		.collect();
    let expr: String = terms.join(" + ");
    println!("{}", commitment);
    println!(" = {}", expr);
}

/// Produces a Vec containing n distinct generators chosen uniformly from G
fn rand_gens<G: Group>(n: usize, rng: &mut impl Rng) -> Vec<G> {
	let mut gens: Vec<G> = Vec::with_capacity(n);
	while gens.len() < n {
		let g = G::rand(rng); // produces a random non-identity element
		if gens.iter().all(|&h| h != g) {
			gens.push(g)
		}
	}
	gens
}

/// Computes the multi-exponential produced by raising the given list of generators
/// to the exponents specified by the given list of coefficients
fn multi_exponent<G: Group>(gens: &Vec<G>, coeffs: &Vec<G::ScalarField>) -> G {
	// TODO implement a more efficient algorithm like Pippenger
	zip(gens.iter(), coeffs.iter())
		.map(|(&x, &y)| x * y)
		.fold(G::zero(), |acc, x| acc + x)
}


#[cfg(test)]
mod tests {
    use super::*;
    use ark_test_curves::bls12_381::{G1Projective as G, Fr as ScalarField};

	#[test]
    fn test_gens() {
        let mut rng = rand::thread_rng();
        let n: usize = 3;

        let gens: Vec<G> = rand_gens(n, &mut rng); 
        let data: Vec<ScalarField> = (0..n).map(|_| ScalarField::rand(&mut rng)).collect();

        let mexp: G = multi_exponent(&gens, &data);

        let mexp_by_hand: G =
            (0..n)
                .map(|idx| gens[idx] * data[idx])
                .fold(G::zero(), |acc, x| acc + x);

        assert_eq!(mexp, mexp_by_hand);
    }
}
