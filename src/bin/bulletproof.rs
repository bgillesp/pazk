use std::fmt;
use std::iter;
use std::sync::Arc;

use ark_std::UniformRand;
use ark_ff::{Zero,One};

use pazk::small_curves::C17Projective as G;
use pazk::small_fields::{self, F17 as F};
use pazk::ip::{self,IP,Channel,Log};
use pazk::group_utils;

fn main() {
    let mut rng = rand::thread_rng();

    println!("Bulletproof protocol for knowledge of opening of a generalized Pedersen commitment");

    println!();

    println!("Cryptographic group: y^2 = x^3 + 2x + 4 over GF(13)");
    println!("Scalar field: GF(17)");

    let deg: usize = 2usize.pow(3) - 1; // should be one less than a power of 2
    println!("Polynomial degree bound: deg(p) <= {deg}");

    println!();

    // construct distinct independent generators

    println!("Begin setup...");
    println!("Picking random generators (g_i) from cryptographic group:");
    let gens: Vec<G> = group_utils::rand_gens(deg+1, &mut rng);
    println!("  {}", group_utils::list_vec(&gens, ", "));

    // construct polynomial evaluation point and vector of monomial evaluations

    let eval_point = F::rand(&mut rng);
    println!("Picking random polynomial evaluation point z: {eval_point}");

    let monoms: Vec<_> =
        iter::successors(Some(F::one()), |m| Some(m * &eval_point))
        .take(deg+1)
        .collect();
    println!("Computing vector y of powers of z: {}", group_utils::list_vec(&monoms, ", "));

    // construct polynomial coefficients and compute evaluation at random point

    let poly_coeffs: Vec<F> = (0..=deg).map(|_| F::rand(&mut rng)).collect();
    println!("Picking vector u of random polynomial coefficients: {}", group_utils::list_vec(&poly_coeffs, ", "));

    let evaluation: F = iter::zip(poly_coeffs.iter(), monoms.iter())
        .map(|(a, y)| a*y)
        .sum();
    println!("Computing evaluation v = <u,y> of polynomial at z: {evaluation}");

    // compute generalized Pedersen commitments and blinding coefficients

    println!("Computing generalized Pedersen commitment for polynomial coefficients");
    let com_u = group_utils::msm(&gens, &poly_coeffs);
    println!("  C_u = Com(u) = {com_u}");

    let gens = Arc::new(gens);
    let public_vector = Arc::new(monoms);
    let public_ip = Arc::new(evaluation);
    let coeffs = Arc::new(poly_coeffs);
    let coeffs_commitment = Arc::new(com_u);

    println!();
    println!("Constructing prover with: (g_i), y; u");
    let prover = BulletproofProver {
        gens: gens.clone(),
        public_vector: public_vector.clone(),
        coeffs: coeffs.clone(),
    };

    println!("Constructing verifier with:  (g_i), y; C_u, v");
    let verifier = BulletproofVerifier {
        gens: gens.clone(),
        public_vector: public_vector.clone(),
        public_ip: public_ip.clone(),
        coeffs_commitment: coeffs_commitment.clone(),
    };
    println!();
    println!("Begin interactive protocol execution...");
    ip::execute(prover, verifier);
}

// PAZK, Protocol 13
// Protocol convinces verifier in logarithmic communication that the inner
// product of a given public vector with a vector committed as a generalized
// Pedersen commitment (or as any additively homomorphic commitment type) is a
// specified value.

struct BulletproofProver {
    gens: Arc< Vec<G> >,
    public_vector: Arc< Vec<F> >,
    coeffs: Arc< Vec<F> >,
}

impl IP<Data> for BulletproofProver {
    fn execute(&self, ch: Channel<Data>, log: Log) {
        let mut _rng = rand::thread_rng();

        // variables updated each recursive round
        let mut vec_len = self.public_vector.len();
        let mut u = (*self.coeffs).clone();
        let mut g = (*self.gens).clone();
        let mut y = (*self.public_vector).clone();

        // compute number of rounds
        let n_rounds = vec_len.ilog2();
        if vec_len != 2usize.pow(n_rounds) {
            return;
        }

        for round in 0..n_rounds {
            log.write(format!(""));
            log.write(format!("Starting round {}...", round+1));

            let half = vec_len / 2;

            log.write(format!("P computes cross terms for folded Pedersen commitment"));
            let comm_cross_term_l = group_utils::msm(&g[half..], &u[..half]);
            let comm_cross_term_r = group_utils::msm(&g[..half], &u[half..]);
            log.write(format!("  v_L = <u_L,g_R> = {}", comm_cross_term_l));
            log.write(format!("  v_R = <u_R,g_L> = {}", comm_cross_term_r));

            log.write(format!("P computes cross terms for folded inner product"));
            let poly_cross_term_l = group_utils::msm(&y[half..], &u[..half]);
            let poly_cross_term_r = group_utils::msm(&y[..half], &u[half..]);
            log.write(format!("  v'_L = <u_L,y_R> = {}", poly_cross_term_l));
            log.write(format!("  v'_R = <u_R,y_L> = {}", poly_cross_term_r));

            log.write(format!("P -> (v_L, v_R, v'_L, v'_R)"));
            ch.send(Data::Commitment(comm_cross_term_l));
            ch.send(Data::Commitment(comm_cross_term_r));
            ch.send(Data::Scalar(poly_cross_term_l));
            ch.send(Data::Scalar(poly_cross_term_r));

            let alpha = ch.receive().to_scalar().unwrap();
            if alpha == F::zero() {
                log.write(format!("Error: received coefficient alpha is zero"));
                log.write(format!("Aborting..."));
                return;
            }
            let alpha_inv = F::one() / alpha;

            log.write(format!("P computes folded generators"));
            g = iter::zip(
                    g[..half].iter()
                        .map(|&x| x*alpha_inv),
                    g[half..].iter()
                        .map(|&x| x*alpha))
                .map(|(x, y)| x + y)
                .collect();

            log.write(format!("P computes folded public vector"));
            y = iter::zip(
                    y[..half].iter()
                        .map(|&x| alpha_inv*x),
                    y[half..].iter()
                        .map(|&x| alpha*x))
                .map(|(x, y)| x + y)
                .collect();

            log.write(format!("P computes folded coefficients vector"));
            u = iter::zip(
                    u[..half].iter()
                        .map(|&x| alpha*x),
                    u[half..].iter()
                        .map(|&x| alpha_inv*x))
                .map(|(x, y)| x + y)
                .collect();
            log.write(format!("  {}", group_utils::list_vec(&u, ", ")));

            vec_len = half;
        }

        // final round: send compressed discrete logarithm u

        log.write(format!(""));
        log.write(format!("Starting round {}...", n_rounds+1));

        log.write(format!("P sends final folded coefficient in the clear"));
        let data = Data::Scalar(u[0]);
        log.write(format!("P -> (u = {data})"));
        ch.send(data);
    }
}


struct BulletproofVerifier {
    gens: Arc< Vec<G> >,
    public_vector: Arc< Vec<F> >,
    public_ip: Arc< F >,
    coeffs_commitment: Arc< G >,
}

impl IP<Data> for BulletproofVerifier {
    fn execute(&self, ch: Channel<Data>, log: Log) {
        let mut rng = rand::thread_rng();

        // variables updated each recursive round
        let mut vec_len = self.public_vector.len();
        let mut c = (*self.coeffs_commitment).clone();
        let mut g = (*self.gens).clone();
        let mut y = (*self.public_vector).clone();
        let mut v = (*self.public_ip).clone();

        // compute number of rounds
        let n_rounds = vec_len.ilog2();
        if vec_len != 2usize.pow(n_rounds) {
            return;
        }

        for _round in 0..n_rounds {
            let half = vec_len / 2;

            let comm_cross_term_l = ch.receive().to_commitment().unwrap();
            let comm_cross_term_r = ch.receive().to_commitment().unwrap();
            let poly_cross_term_l = ch.receive().to_scalar().unwrap();
            let poly_cross_term_r = ch.receive().to_scalar().unwrap();

            log.write(format!("V picks nonzero scalar alpha uniformly at random"));
            let mut alpha = F::zero();
            while alpha == F::zero() {
                alpha = F::rand(&mut rng);
            }

            let alpha_inv = F::one() / alpha;

            log.write(format!("V computes folded generators"));
            g = iter::zip(
                    g[..half].iter()
                        .map(|&x| x*alpha_inv),
                    g[half..].iter()
                        .map(|&x| x*alpha))
                .map(|(x, y)| x + y)
                .collect();
            log.write(format!("  {}", group_utils::list_vec(&g, ", ")));

            log.write(format!("V computes folded public vector"));
            y = iter::zip(
                    y[..half].iter()
                        .map(|&x| alpha_inv*x),
                    y[half..].iter()
                        .map(|&x| alpha*x))
                .map(|(x, y)| x + y)
                .collect();
            log.write(format!("  {}", group_utils::list_vec(&y, ", ")));

            log.write(format!("V computes folded Pedersen commitment"));
            c += comm_cross_term_l*(alpha*alpha) + comm_cross_term_r*(alpha_inv*alpha_inv);
            log.write(format!("  {}", c));

            log.write(format!("V computes folded inner product"));
            v += poly_cross_term_l*(alpha*alpha) + poly_cross_term_r*(alpha_inv*alpha_inv);
            log.write(format!("  {}", v));

            vec_len = half;

            let data = Data::Scalar(alpha);
            log.write(format!("V -> (alpha = {data})"));
            ch.send(data);
        }

        let u0 = ch.receive().to_scalar().unwrap();
        let g0 = g[0];
        let y0 = y[0];

        log.write(format!("V checks that discrete log relations hold:"));
        log.write(format!("  u*g == {}*{} ?= {} == C_u",
            Data::Scalar(u0), Data::Commitment(g0), Data::Commitment(c)));
        log.write(format!("  u*y == {}*{} ?= {} == v",
            Data::Scalar(u0), Data::Scalar(y0), Data::Scalar(v)));

        let decision = g0*u0 == c && y0*u0 == v;

        let data = Data::Decision(decision);
        log.write(format!("V --> ({})", data));
        ch.send(data);
    }
}


#[derive(Clone)]
enum Data {
    Scalar(F),
    Commitment(G),
    Decision(bool),
}

impl Data {
    fn to_scalar(self) -> Option<F> {
        if let Data::Scalar(x) = self { Some(x) } else { None }
    }

    fn to_commitment(self) -> Option<G> {
        if let Data::Commitment(p) = self { Some(p) } else { None }
    }

    fn _to_decision(self) -> Option<bool> {
        if let Data::Decision(d) = self { Some(d) } else { None }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Data::Scalar(x) => {
                write!(f, "{}", small_fields::to_u64(*x))
            }
            Data::Commitment(g) => {
                write!(f, "{}", g)
            }
            Data::Decision(b) => {
                if *b { write!(f, "Accept") } else { write!(f, "Reject") }
            }
        }
    }
}
