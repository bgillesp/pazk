use std::fmt;
use std::iter;
use std::sync::Arc;

use ark_std::UniformRand;
use ark_ff::One;

use pazk::small_curves::C17Projective as G;
use pazk::small_fields::{self, F17 as F};
use pazk::ip::{self,IP,Channel,Log};
use pazk::group_utils;

fn main() {
    let mut rng = rand::thread_rng();

    println!("ZK protocol for polynomial opening relation of Pedersen commitments");

    println!();

    println!("Cryptographic group: y^2 = x^3 + 2x + 4 over GF(13)");
    println!("Scalar field: GF(17)");

    let deg: usize = 2;
    println!("Polynomial degree bound: deg(p) <= {deg}");

    println!();

    // construct distinct independent generators

    println!("Begin setup...");
    println!("Picking random generators from cryptographic group");
    let gens: Vec<G> = group_utils::rand_gens(deg+3, &mut rng);

    let g = gens[deg+2];
    let h = gens[deg+1];
    let rest = &gens[..deg+1];

    let gens = Generators {
    	vector_gens: rest.to_vec(),
    	scalar_gen: g,
		blinding_gen: h,
    };
    println!("  Vector generators g_i: {}", group_utils::list_vec(&gens.vector_gens, " "));
    println!("  Scalar generator g: {}", gens.scalar_gen);
    println!("  Blinding generator h: {}", gens.blinding_gen);

    // construct polynomial evaluation point and vector of monomial evaluations

    let eval_point = F::rand(&mut rng);
    println!("Picking random evaluation point z for protocol: {eval_point}");

    let monoms: Vec<_> =
    	iter::successors(Some(F::one()), |m| Some(m * &eval_point))
		.take(deg+1)
		.collect();
	println!("Computing vector y of powers of z: {}", group_utils::list_vec(&monoms, " "));

    // construct polynomial coefficients and compute evaluation at random point

	let poly_coeffs: Vec<F> = (0..=deg).map(|_| F::rand(&mut rng)).collect();
	println!("Picking vector u of random polynomial coefficients: {}", group_utils::list_vec(&poly_coeffs, " "));

	let evaluation: F = iter::zip(poly_coeffs.iter(), monoms.iter())
		.map(|(a, y)| a*y)
		.sum();
	println!("Computing evaluation v = <u,y> of polynomial at z: {evaluation}");

	// compute generalized Pedersen commitments and blinding coefficients

	println!("Computing generalized Pedersen commitment for polynomial coefficients");
	let rand_u = F::rand(&mut rng);
	let com_u = group_utils::msm(&gens.vector_gens, &poly_coeffs)
		+ (gens.blinding_gen * rand_u);
	println!("  r_u = {rand_u}; C_u = Com(u,r_u) = {com_u}");

	println!("Computing generalized Pedersen commitment for polynomial evaluation");
	let rand_v = F::rand(&mut rng);
	let com_v = (gens.scalar_gen * evaluation)
		+ (gens.blinding_gen * rand_v);
	println!("  r_v = {rand_v}; C_v = Com(v,r_v) = {com_v}");

    let gens = Arc::new(gens);
    let public_vector = Arc::new(monoms);
	let coeffs = Arc::new(poly_coeffs);
	let coeffs_blinding_factor = Arc::new(rand_u);
	let coeffs_commitment = Arc::new(com_u);
	let ip_blinding_factor = Arc::new(rand_v);
	let ip_commitment = Arc::new(com_v);

	println!();
	println!("Constructing prover with: (g_i), g, h, y; u, r_u, r_v");
    let prover = PedersenProver {
    	gens: gens.clone(),
		public_vector: public_vector.clone(),
		coeffs: coeffs.clone(),
		coeffs_blinding_factor: coeffs_blinding_factor.clone(),
		ip_blinding_factor: ip_blinding_factor.clone(),
    };

    println!("Constructing verifier with:  (g_i), g, h, y; C_u, C_v");
    let verifier = PedersenVerifier {
    	gens: gens.clone(),
		public_vector: public_vector.clone(),
		coeffs_commitment: coeffs_commitment.clone(),
		ip_commitment: ip_commitment.clone(),
    };
    println!();
    println!("Begin interactive protocol execution...");
    ip::execute(prover, verifier);
}

struct Generators {
	vector_gens: Vec<G>,
	scalar_gen: G,
	blinding_gen: G,
}

// PAZK, Protocol 11:
// Protocol to prove in zero knowledge that the inner product of a collection of
// coefficients, represented as a generalized Pedersen commitment, with a known
// public vector, is encoded in a second Pedersen commitment.

struct PedersenProver {
	gens: Arc< Generators >,
	public_vector: Arc< Vec<F> >,
	coeffs: Arc< Vec<F> >,
	coeffs_blinding_factor: Arc< F >,
	ip_blinding_factor: Arc< F >,
}

impl IP<Data> for PedersenProver {
    fn execute(&self, ch: Channel<Data>, log: Log) {
    	let mut rng = rand::thread_rng();
    	let vec_len = self.public_vector.len();

    	// round 1

   		log.write(format!("P picks vector d of field elements uniformly at random"));
    	let d: Vec<F> = (0..vec_len).map(|_| F::rand(&mut rng)).collect();
		log.write(format!("  d = {}", group_utils::list_vec(&d, " ")));

    	log.write(format!("P computes commitment to d"));
    	let r1 = F::rand(&mut rng);
    	let com_d = group_utils::msm(&self.gens.vector_gens, &d)
    		+ (self.gens.blinding_gen * r1);
		log.write(format!("  r1 = {r1}; C_d = Com(d, r1) = {com_d}"));

    	log.write(format!("P computes inner product <d,y> of d with public vector"));
    	let d_ip: F = iter::zip(d.iter(), self.public_vector.iter())
			.map(|(a, y)| a*y)
			.sum();
		log.write(format!("  <d,y> = {d_ip}"));

		log.write(format!("P computes commitment to inner product <d,y>"));
    	let r2 = F::rand(&mut rng);
    	let com_d_ip = (self.gens.scalar_gen * d_ip)
			+ (self.gens.blinding_gen * r2);
		log.write(format!("  r2 = {r2}; C_<d,y> = Com(<d,y>, r2) = {com_d_ip}"));

		log.write(format!("P -> (C_d, C_<d,y>)"));
		ch.send(Data::Commitment(com_d));
		ch.send(Data::Commitment(com_d_ip));

		// wait for Verifier message

		let data = ch.receive();
		let e = data.to_scalar().unwrap();

		// round 2

		log.write(format!("P computes random vector u' = e*u + d directly"));
		let rand_coeffs: Vec<F> = self.coeffs.iter()
			.map(|x| e*x)
			.zip(d)
			.map(|(x, y)| x+y)
			.collect();
		log.write(format!("  u' = {}", group_utils::list_vec(&rand_coeffs, " ")));

		log.write(format!("P computes derived blinding factors of derived commitments for u' and <u',y>"));
		let rand_blinding_factor = *self.coeffs_blinding_factor * e + r1;
		let rand_ip_blinding_factor = *self.ip_blinding_factor * e + r2;
		log.write(format!("  r_u' = {rand_blinding_factor}; r_<u',y> = {rand_ip_blinding_factor}"));

		log.write(format!("P -> (u', r_u', r_<u',y>)"));
		ch.send(Data::Vector(rand_coeffs));
		ch.send(Data::Scalar(rand_blinding_factor));
		ch.send(Data::Scalar(rand_ip_blinding_factor));

		// execution complete
    }
}


struct PedersenVerifier {
	gens: Arc< Generators >,
	public_vector: Arc< Vec<F> >,
	coeffs_commitment: Arc< G >,
	ip_commitment: Arc< G >,
}

impl IP<Data> for PedersenVerifier {
	fn execute(&self, ch: Channel<Data>, log: Log) {
		let mut rng = rand::thread_rng();

		// wait for Prover messages

		let data = ch.receive();
		let com_d = data.to_commitment().unwrap();

		let data = ch.receive();
		let com_d_ip = data.to_commitment().unwrap();

		// round 1

		log.write(format!("V picks e uniformly at random"));
		let e = F::rand(&mut rng);

		let data = Data::Scalar(e);
		log.write(format!("V -> (e = {data})"));
		ch.send(data);

		// wait for Prover messages

		let data = ch.receive();
		let rand_coeffs = data.to_vector().unwrap();

		let data = ch.receive();
		let rand_blinding_factor = data.to_scalar().unwrap();

		let data = ch.receive();
		let rand_ip_blinding_factor = data.to_scalar().unwrap();

		// compute decision

		log.write(format!("V computes inner product of u' and public vector directly"));
		let rand_ip: F = iter::zip(rand_coeffs.iter(), self.public_vector.iter())
			.map(|(a, y)| a*y)
			.sum();
		log.write(format!("  <u',y> = {rand_ip}"));

		log.write(format!("V computes commitments to u' and <u', y> directly"));
		let com_rc = group_utils::msm(&self.gens.vector_gens, &rand_coeffs)
			+ (self.gens.blinding_gen * rand_blinding_factor);
		let com_rc_ip = (self.gens.scalar_gen * rand_ip)
			+ (self.gens.blinding_gen * rand_ip_blinding_factor);
		log.write(format!("  C_u' = Com(u', r_u') = {com_rc}"));
		log.write(format!("  C_<u',y> = Com(<u',y>, r_<u',y>) = {com_rc_ip}"));

		log.write(format!("V derives commitments to random vector and inner product using additive homomorphism"));
		let com_rc_computed = *self.coeffs_commitment * e + com_d;
		let com_rc_ip_computed = *self.ip_commitment * e + com_d_ip;
		log.write(format!("  C_u'* = e*C_u + C_d = {com_rc_computed}"));
		log.write(format!("  C_<u',y>* = e*C_<u,y> + C_<d,y> = {com_rc_ip_computed}"));

		log.write(format!("V checks that directly computed commitments match derived commitments"));
		let decision = com_rc == com_rc_computed && com_rc_ip == com_rc_ip_computed;

		let data = Data::Decision(decision);
		log.write(format!("V -> ({})", data));
		ch.send( data );

		// execution complete
	}
}


#[derive(Clone)]
enum Data {
    Scalar(F),
    Vector(Vec<F>),
    Commitment(G),
    Decision(bool),
}

impl Data {
    fn to_scalar(self) -> Option<F> {
        if let Data::Scalar(x) = self { Some(x) } else { None }
    }

    fn to_vector(self) -> Option<Vec<F>> {
    	if let Data::Vector(x) = self { Some(x) } else { None }
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
            Data::Vector(v) => {
            	write!(f, "{}", group_utils::list_vec(v, " "))
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
