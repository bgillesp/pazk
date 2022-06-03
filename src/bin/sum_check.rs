use std::fmt;
use std::iter::Iterator;
use std::sync::Arc;

use rand;

use ark_ff::{Zero,One,UniformRand};
use ark_poly::Polynomial;
use ark_poly::polynomial::multivariate::{SparsePolynomial as MultiPoly, SparseTerm};
use ark_poly::polynomial::univariate::DensePolynomial as UniPoly;

use pazk::small_fields as sf;
use pazk::small_fields::{F13 as Fq};
use pazk::polynomials as polys;
use pazk::ip;
use pazk::ip::{IP,Channel,Log};

fn main() {
    // f(x, y) = x^2 + 2xy + 3y^2
    // --> 2x^2 + 2x + 3
    // --> 10
    let poly_spec =
        vec![
            (1, vec![(0, 2)]),
            (2, vec![(0, 1), (1, 1)]),
            (3, vec![(1, 2)]),
        ];
    let polynomial = Arc::new(polys::construct_poly(2, poly_spec));
    let claimed_sum = Fq::from(10u64);
    let degrees = polys::variable_degrees(&polynomial);

    println!("Sum Check Protocol");
    println!("==================");
    let prover = SumCheckProver {
        polynomial: polynomial.clone(),
    };
    let verifier = SumCheckVerifier {
        polynomial: polynomial.clone(),
        claimed_sum,
        degrees,
    };
    ip::execute(prover, verifier);
}

struct SumCheckProver {
    polynomial: Arc< MultiPoly<Fq, SparseTerm> >,
}

impl IP<Data> for SumCheckProver {
    fn execute(&self, ch: Channel<Data>, log: Log) {
        let num_vars = self.polynomial.num_vars;
        let mut poly = self.polynomial.clone();

        for j in (0..num_vars).rev() {
            // compute univariate restriction
            log.write(format!("P computes univariate polynomial g_{}", j));
            let vals = (0..num_vars)
                .map(|n| {
                    if n < j {
                        Some(vec![Fq::zero(), Fq::one()])
                    } else {
                        None
                    }
                })
                .collect();
            let partial = polys::partial_summation(&poly, &vals);
            let uni = polys::into_univariate(&partial, j);

            // send univariate restriction to verifier
            let data = Data::Polynomial(uni);
            log.write(format!("P --> (g_{} = {})", j, data));
            ch.send(data);

            // wait for random challenge, except for last challenge
            if j > 0 {
                let data = ch.receive();
                if let Data::Decision(false) = data { return; }
                let challenge = data.to_scalar().unwrap();

                // restrict according to random challenge
                log.write(format!("P computes partial evaluation at x_{} = r_{}", j, j));
                poly = Arc::new(polys::partial_eval(&poly, challenge, j));
            }
        }
    }
}

struct SumCheckVerifier {
    polynomial: Arc< MultiPoly<Fq, SparseTerm> >,
    degrees: Vec<usize>,
    claimed_sum: Fq,
}

impl IP<Data> for SumCheckVerifier {
    fn execute(&self, ch: Channel<Data>, log: Log) {
        let mut rng = rand::thread_rng();

        let zero = Fq::zero();
        let one  = Fq::one();

        // let current check value equal to claimed sum
        let mut check_value = self.claimed_sum;
        let mut challenges: Vec<Fq> = Vec::with_capacity(self.polynomial.num_vars);

        for j in (0..self.polynomial.num_vars).rev() {
            // wait for univariate restriction
            let uni = ch.receive().to_polynomial().unwrap();

            log.write(format!("V checks g_{} has small enough degree", j));
            if uni.degree() > self.degrees[j] {
                let data = Data::Decision(false);
                log.write(format!("V --> ({})", data));
                ch.send(data);
                return;
            }

            log.write(format!("V checks g_{} sums to check value", j));
            if uni.evaluate(&zero) + uni.evaluate(&one) != check_value {
                let data = Data::Decision(false);
                log.write(format!("V --> ({})", data));
                ch.send(data);
                return;
            }

            log.write(format!("V picks r_{} uniformly at random", j));
            let challenge = Fq::rand(&mut rng);

            log.write(format!("V updates check value to g_{}(r_{})", j, j));
            check_value = uni.evaluate(&challenge);

            // record challenge for later reference
            challenges.push(challenge);

            // send random challenge to prover, except for last challenge
            let data = Data::Scalar(challenge);
            if j > 0 {
                log.write(format!("V --> (r_{} = {})", j, data));
                ch.send(data);
            } else {
                log.write(format!("V has (r_0 = {}) but does not send to P", data));
            }
        }

        // evaluate polynomial at vector of challenge points
        log.write(String::from("V evaluates g(r) with a single oracle query"));
        challenges.reverse();
        let oracle_evaluation = self.polynomial.evaluate(&challenges);

        // accept if the oracle evaluation equals the final check value
        // otherwise reject
        log.write(String::from("V checks that oracle evaluation equals final check value"));
        let decision = Data::Decision(oracle_evaluation == check_value);

        log.write(format!("V --> ({})", decision));
        ch.send(Data::Decision(oracle_evaluation == check_value));
    }
}

// TODO implement brute force prover which works for small fields


#[derive(Clone)]
enum Data {
    Scalar(Fq),
    Polynomial(UniPoly<Fq>),
    Decision(bool),
}

impl Data {
    fn to_scalar(self) -> Option<Fq> {
        if let Data::Scalar(x) = self { Some(x) } else { None }
    }

    fn to_polynomial(self) -> Option<UniPoly<Fq>> {
        if let Data::Polynomial(p) = self { Some(p) } else { None }
    }

    fn _to_decision(self) -> Option<bool> {
        if let Data::Decision(d) = self { Some(d) } else { None }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Data::Scalar(x) => {
                write!(f, "{}", sf::to_u64(*x))
            }
            Data::Polynomial(p) => {
                write!(f, "{}", polys::format_univ_poly(&p, "x"))
            }
            Data::Decision(b) => {
                if *b { write!(f, "Accept") } else { write!(f, "Reject") }
            }
        }
    }
}
