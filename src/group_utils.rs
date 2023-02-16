use std::fmt;
use rand::Rng;

use ark_ec::Group;
use std::ops::{Add, Mul};
use ark_std::Zero;


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

/// Computes the multi-scalar multiplication of the given elements with given
///  produced by raising the given list of generators
pub fn msm<E, S>(elts: &[E], scalars: &[S]) -> E where
	E: Zero + Add<E, Output=E> + Mul<S, Output=E> + Copy,
	S: Copy,
{
	elts.iter().zip(scalars.iter())
		.map(|(&x, &y)| x * y)
		.fold(E::zero(), |acc, x| acc + x)
	// TODO implement a more efficient algorithm like Pippenger
}

pub fn list_vec<T: fmt::Display>(vec: &[T], sep: &str) -> String {
	vec.iter()
		.map(|x| x.to_string())
		.collect::<Vec<_>>()
		.join(sep)
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
