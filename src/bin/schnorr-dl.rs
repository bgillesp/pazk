use std::fmt;
use std::sync::Arc;

use ark_std::UniformRand;

use pazk::small_curves::C17Projective as G;
use pazk::small_fields::{self, F17 as F};
use pazk::ip::{self,IP,Channel,Log};

fn main() {
    let mut rng = rand::thread_rng();

    println!("Schnorr's protocol for proof of knowledge of a discrete logarithm");

    println!();
    println!("Begin setup...");

    println!("Using group G and scalar field F:");
    println!("  G: y^2 = x^3 + 2x + 4 over GF(13)");
    println!("  F: GF(17)");

    println!("Letting g be a random generator of G, and w a random element in F:");
    let g = G::rand(&mut rng);
    let w = F::rand(&mut rng);
    println!("  g = {g}, w = {w}");
    let h = g*w;
    println!("  h = g^w = {h}");

    let g = Arc::new(g);
    let w = Arc::new(w);
    let h = Arc::new(h);

	println!("Constructing prover P and verifier V with parameters:");
    let prover = SchnorrDLProver {
    	g: g.clone(),
    	w: w.clone(),
    };
	println!("  P <- (g, w)");
    let verifier = SchnorrDLVerifier {
    	g: g.clone(),
    	h: h.clone(),
    };
	println!("  V <- (g, h)");

    println!();
    println!("Begin interactive protocol execution...");
    ip::execute(prover, verifier);
}

// PAZK, Protocol 4:
// Sigma protocol convinces a verifier that prover knows the discrete log of a
// given group element to the base of a given generator

struct SchnorrDLProver {
	g: Arc< G >,
	w: Arc< F >,
}

impl IP<Data> for SchnorrDLProver {
    fn execute(&self, ch: Channel<Data>, log: Log) {
    	let mut rng = rand::thread_rng();

    	// message 1

    	let r: F = F::rand(&mut rng);
    	log.write(format!("P picks random exponent r = {r} from F"));

    	let a = *self.g * r;
    	log.write(format!("P computes a = g^r = {a}"));

    	log.write(format!("P -> a"));
    	ch.send( Data::GroupElement(a) );

    	// wait for verifier response

    	let e = ch.receive().to_scalar().unwrap();

    	// message 2

    	let z = *self.w * e + r;
    	log.write(format!("P computes exponent z = w*e + r = {z}"));

    	log.write(format!("P -> z"));
    	ch.send( Data::Scalar(z) );

		// execution complete
    }
}


struct SchnorrDLVerifier {
	g: Arc< G >,
	h: Arc< G >,
}

impl IP<Data> for SchnorrDLVerifier {
	fn execute(&self, ch: Channel<Data>, log: Log) {
		let mut rng = rand::thread_rng();

		// wait for Prover message

		let a = ch.receive().to_group_element().unwrap();

		// message 1

		let e: F = F::rand(&mut rng);
		log.write(format!("V picks random exponent e = {e} from F"));

		log.write(format!("V -> e"));
		ch.send( Data::Scalar(e) );

		// wait for Prover response

		let z = ch.receive().to_scalar().unwrap();

		// compute decision

		log.write(format!("V checks that a*h^e == g^z"));
		let decision = a + *self.h * e == *self.g * z;

		let data = Data::Decision(decision);
		log.write(format!("V -> {data}"));
		ch.send( data );

		// execution complete
	}
}


#[derive(Clone)]
enum Data {
    Scalar(F),
    GroupElement(G),
    Decision(bool),
}

impl Data {
    fn to_scalar(self) -> Option<F> {
        if let Data::Scalar(x) = self { Some(x) } else { None }
    }

    fn to_group_element(self) -> Option<G> {
        if let Data::GroupElement(p) = self { Some(p) } else { None }
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
            Data::GroupElement(g) => {
                write!(f, "{}", g)
            }
            Data::Decision(b) => {
                if *b { write!(f, "Accept") } else { write!(f, "Reject") }
            }
        }
    }
}
