use std::iter::zip;
use rand::Rng;

use ark_ec::Group;


/// Produces a Vec containing n distinct generators chosen uniformly from G
pub fn rand_gens<G: Group>(n: usize, rng: &mut impl Rng) -> Vec<G> {
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
pub fn multi_exponent<G: Group>(gens: &Vec<G>, coeffs: &Vec<G::ScalarField>) -> G {
	// TODO implement a more efficient algorithm like Pippenger
	zip(gens.iter(), coeffs.iter())
		.map(|(&x, &y)| x * y)
		.fold(G::zero(), |acc, x| acc + x)
}


#[cfg(test)]
mod tests {
    use super::*;
    use ark_std::UniformRand;
    use ark_ff::Zero;
    use crate::small_curves::C17Projective as G;
    use crate::small_fields::F17 as ScalarField;

	#[test]
    fn test_multi_exponent() {
        let mut rng = rand::thread_rng();
        let n: usize = 3;

        let gens: Vec<G> = (0..n).map(|_| G::rand(&mut rng)).collect(); 
        let data: Vec<ScalarField> = (0..n).map(|_| ScalarField::rand(&mut rng)).collect();

        let mexp: G = multi_exponent(&gens, &data);

        let mexp_by_hand: G =
            (0..n)
                .map(|idx| gens[idx] * data[idx])
                .fold(G::zero(), |acc, x| acc + x);

        assert_eq!(mexp, mexp_by_hand);
    }

    #[test]
    fn test_gens() {
    	let mut rng = rand::thread_rng();
        let n: usize = 8;

        let gens: Vec<G> = rand_gens(n, &mut rng);

        for i in 0..n {
        	// generators must be non-identity elements
        	assert_ne!(gens[i], G::zero());
    		// list entries must be distinct
        	for j in 0..i {
        		assert_ne!(gens[i], gens[j]);
        	}
        }
    }
}
