use std::cmp;
use std::iter::Iterator;

use ark_ff::Zero;
use ark_ff::fields::{Field,Fp64,Fp64Parameters};

use ark_poly::{Polynomial,MVPolynomial};
use ark_poly::polynomial::multivariate::SparsePolynomial as MultiPoly;
use ark_poly::polynomial::multivariate::{Term,SparseTerm};
use ark_poly::polynomial::univariate::DensePolynomial as UniPoly;

use crate::small_fields;


pub fn format_univ_poly<T: Fp64Parameters>(poly: &UniPoly<Fp64<T>>, varname: &str) -> String {
    if poly.coeffs.len() == 0 || !poly.coeffs.iter().any(|coeff| *coeff != Fp64::zero()) {
        String::from("0")
    } else {
        poly.coeffs
        .iter()
        .enumerate()
        .rev()
        .filter(|(_, coeff)| **coeff != Fp64::zero())
        .map(|(exp, coeff)| {
            let coeff = small_fields::to_u64(*coeff);
            match (exp, coeff) {
                (0, _) => format!("{}", coeff),
                (1, 1) => format!("{}", varname),
                (1, _) => format!("{}*{}", coeff, varname),
                (_, 1) => format!("{}^{}", varname, exp),
                (_, _) => format!("{}*{}^{}", coeff, varname, exp),
            }
        })
        .collect::<Vec<_>>()
        .join(" + ")
    }
}

/// Compute the sums of k-powers of the list of summands for k up to `max_exponent`.
fn power_sums<F: Field>(max_exponent: usize, summands: &Vec<F>) -> Vec<F> {
    let mut powers: Vec<F> = vec![F::one(); summands.len()];
    let mut power_sums: Vec<F> = Vec::with_capacity(max_exponent + 1);
    for _ in 0..=max_exponent {
        power_sums.push(powers.iter().sum());
        for (power, term) in powers.iter_mut().zip(summands) {
            *power *= term;
        }
    }
    power_sums
}

/// Computes partial evaluation/summation of a multivariate polynomial.  Each
/// variable either remains unevaluated or is summed over a fixed set of values.
/// Using a single value for the summation set is equivalent to evaluating the
/// corresponding variable at this value.
pub fn partial_summation<F: Field>(f: &MultiPoly<F, SparseTerm>, vals: &Vec<Option<Vec<F>>>) -> MultiPoly<F, SparseTerm> {
    let monom_terms: Vec< Option<Vec<F>> > =
        vals.iter()
            .map(|val| match val {
                Some(summands) => Some(power_sums(f.degree(), &summands)),
                None => None,
            })
            .collect();

    let mut dense_monomial_exponents = vec![0usize; f.num_vars];
    let new_terms =
        f.terms
            .iter()
            .map(|(coeff, term)| {
                // load exponents of all variables (including degree 0) into Vec
                dense_monomial_exponents.fill(0usize);
                for (idx, exp) in term.iter() {
                    dense_monomial_exponents[*idx] = *exp;
                }
                let new_coeff: F = dense_monomial_exponents
                    .iter()
                    .enumerate()
                    .map(|(idx, exp)| {
                        match &monom_terms[idx] {
                        Some(power_sums) => power_sums[*exp],
                        None => F::one(),
                        }
                    })
                    .fold(*coeff, |acc, val| acc * val);
                let new_term = SparseTerm::new(term
                    .iter()
                    .filter(|(idx, _)| match &monom_terms[*idx] {
                        None => true,
                        _ => false,
                    })
                    .map(|tup| *tup )
                    .collect());
                (new_coeff, new_term)
            })
            .collect();
    MultiPoly::from_coefficients_vec(f.num_vars(), new_terms)
}

/// Compute the partial evaluation of `poly` by the specified field element and variable.
pub fn partial_eval<F: Field>(poly: &MultiPoly<F, SparseTerm>, value: F, variable: usize) -> MultiPoly<F, SparseTerm> {
    let vals = (0..poly.num_vars)
        .map(|n| {
            if n == variable {
                Some(vec![value])
            } else {
                None
            }
        })
        .collect();
    partial_summation(&poly, &vals)
}

/// Restrict a multivariate polynomial to the univariate polynomial obtained by
/// evaluating at zero for all variables except the one specified.
pub fn into_univariate<F: Field>(poly: &MultiPoly<F, SparseTerm>, variable: usize) -> UniPoly<F> {
    let mut coeffs: Vec<F> = vec![F::zero(); poly.degree()+1];
    poly.terms
        .iter()
        .filter(|(_, monom)| {
            !monom.iter().any(|(idx, exp)| *idx != variable && *exp > 0)
        })
        .fold(&mut coeffs, |acc, (coeff, monom)| {
            let exp = if monom.is_empty() { 0usize } else { monom[0].1 };
            (*acc)[exp] += coeff;
            acc
        });
    UniPoly { coeffs }
}

/// Compute the list of variable degrees of `poly`, i.e. for each `i` computes
/// the degree of `poly` as a univariate polynomial in just the variable x_i,
/// considering all other variables as constants.
pub fn variable_degrees<F: Field>(poly: &MultiPoly<F, SparseTerm>) -> Vec<usize> {
    let mut degs = vec![0usize; poly.num_vars];
    let zero = F::zero();
    for (coeff, monom) in &poly.terms {
        if *coeff == zero { continue; }
        for (variable, power) in monom.iter() {
            degs[*variable] = cmp::max(degs[*variable], *power);
        }
    }
    degs
}

/// Construct a multivariate polynomial over a 64-bit field using u64 coefficients and Vec monomials.
pub fn construct_poly<T: Fp64Parameters>(num_vars: usize, spec: Vec<(u64, Vec<(usize, usize)>)>) -> MultiPoly<Fp64<T>, SparseTerm> {
    let terms: Vec<(Fp64<T>, SparseTerm)> = spec
        .into_iter()
        .map(|(coeff, monom)| {
            (Fp64::from(coeff), SparseTerm::new(monom))
        })
        .collect();
    MultiPoly::from_coefficients_vec(num_vars, terms)
}

pub fn construct_const_poly<T: Fp64Parameters>(num_vars: usize, value: u64) -> MultiPoly<Fp64<T>, SparseTerm> {
    MultiPoly::from_coefficients_vec(
        num_vars,
        vec![(Fp64::from(value), SparseTerm::new(vec![]))]
    )
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::small_fields::{F5 as Fq};
    use ark_ff::{Zero,One,UniformRand};

    #[test]
    fn test_power_sums() {
        let mut rng = rand::thread_rng();
        let a = Fq::rand(&mut rng);
        let b = Fq::rand(&mut rng);

        let ps: Vec<Fq> = power_sums(3, &vec![a, b]);

        let ps_by_hand: Vec<Fq> =
            (0..=3u64)
                .map(|e| a.pow([e]) + b.pow([e]))
                .collect();

        assert_eq!(ps, ps_by_hand);
    }

    #[test]
    fn test_partial_summation() {
        let test_vectors = vec![
            (
                "With f(x, y) = x^2 + 2xy + 3y^2, f(2, 1) = 11",
                2usize,
                vec![
                    (1, vec![(0, 2)]),
                    (2, vec![(0, 1), (1, 1)]),
                    (3, vec![(1, 2)]),
                ],
                vec![
                    Some(vec![2]),
                    Some(vec![1]),
                ],
                vec![
                    (11, vec![]),
                ],
            ),
            (
                "With f(x, y) = x^2 + 2xy + 3y^2, f(2, y) = 3y^2 + 4y + 4",
                2usize,
                vec![
                    (1, vec![(0, 2)]),
                    (2, vec![(0, 1), (1, 1)]),
                    (3, vec![(1, 2)]),
                ],
                vec![
                    Some(vec![2]),
                    None,
                ],
                vec![
                    (3, vec![(1, 2)]),
                    (4, vec![(1, 1)]),
                    (4, vec![]),
                ],
            ),
            (
                "With f(x, y) = x^2 + 2xy + 3y^2, sum for x, y in {0, 1} f(x, y) = 10",
                2usize,
                vec![
                    (1, vec![(0, 2)]),
                    (2, vec![(0, 1), (1, 1)]),
                    (3, vec![(1, 2)]),
                ],
                vec![
                    Some(vec![0, 1]),
                    Some(vec![0, 1]),
                ],
                vec![
                    (10, vec![]),
                ],
            ),
            (
                "With f(x, y) = x^2 + 2xy + 3y^2, sum for y in {1, 3} f(x, y) = 2x^2 + 8x + 30",
                2usize,
                vec![
                    (1, vec![(0, 2)]),
                    (2, vec![(0, 1), (1, 1)]),
                    (3, vec![(1, 2)]),
                ],
                vec![
                    None,
                    Some(vec![1, 3]),
                ],
                vec![
                    (2, vec![(0,2)]),
                    (8, vec![(0,1)]),
                    (30, vec![]),
                ],
            ),
        ];

        for (desc, num_vars, poly, vals, result) in test_vectors {
            let result = construct_poly(num_vars, result);
            let poly = construct_poly(num_vars, poly);
            let vals = construct_vals(vals);
            let computed_result = partial_summation(&poly, &vals);
            assert_eq!(computed_result, result, "{}", desc);
        }
    }

    #[test]
    fn test_into_univariate() {
        // f(x, y) = x^2 + 2xy + 3y^2
        let poly_spec =
            vec![(1, vec![(0, 2)]),
                 (2, vec![(0, 1), (1, 1)]),
                 (3, vec![(1, 2)]),];
        let poly = construct_poly(2usize, poly_spec);

        let uni = into_univariate(&poly, 0);
        let result = UniPoly { coeffs: vec![Fq::zero(), Fq::zero(), Fq::one()] };
        assert_eq!(uni, result, "f(x, y) = x^2 + 2xy + 3y^2 restricted to x is g(x) = x^2");

        let partial = partial_summation(&poly, &vec![Some(vec![Fq::from(2)]), None]);
        let uni = into_univariate(&partial, 1);
        let result: UniPoly<Fq> = UniPoly { coeffs: vec![Fq::from(4), Fq::from(4), Fq::from(3)]};
        assert_eq!(uni, result, "f(x, y) = 3y^2 + 4y + 4 restricted to y is itself as a univariate polynomial");
    }

    fn construct_vals(vals_spec: Vec<Option<Vec<u64>>>) -> Vec<Option<Vec<Fq>>> {
    vals_spec.into_iter()
        .map(|var_option| {
            match var_option {
                None => None,
                Some(vec) => {
                    Some(
                        vec.into_iter()
                            .map(|x| {
                                Fq::from(x)
                            })
                            .collect()
                    )
                }
            }
        })
        .collect()
    }
}
