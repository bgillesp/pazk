use std::fmt;
use std::io;
use std::io::Write;

use rand;
use rand::Rng;
use rand::rngs::ThreadRng;

use ark_std::UniformRand;
use ark_ff::{Field};
use ark_bls12_381::Fq2 as F;

use ndarray::{Array,Array1,Array2,Dimension};

// Dimension of vectors and arrays
const N: usize = 64;

fn main() {
    let mut rng = rand::thread_rng();
    let mut r: F;

    // generate random A and B
    print!("Generating random {}x{} matrices A and B ... ", N, N);
    io::stdout().flush().unwrap();
    let mut a = Array2::<F>::default((N, N));
    let mut b = Array2::<F>::default((N, N));
    fill_random(&mut a, &mut rng);
    fill_random(&mut b, &mut rng);
    println!("Done");

    // compute matrix product
    print!("Computing product matrix A*B ... ");
    io::stdout().flush().unwrap();
    let mut c = a.dot(&b);
    println!("Done");

    // test with C = A*B
    print!("Testing with C = A*B ... ");
    io::stdout().flush().unwrap();
    r = F::rand(&mut rng);
    println!("{}", freivald_check(&a, &b, &c, &r));

    // test with C = A*B modified by a single entry
    print!("Testing with C = A*B but one entry randomized ... ");

    let (u, v): (usize, usize) = (rng.gen_range(0..N), rng.gen_range(0..N));
    c[(u,v)] = F::rand(&mut rng);
    r = F::rand(&mut rng);
    println!("{}", freivald_check(&a, &b, &c, &r));

    // test with C a different random matrix
    print!("Testing with C uniform random ... ");
    io::stdout().flush().unwrap();
    fill_random(&mut c, &mut rng);
    r = F::rand(&mut rng);
    println!("{}", freivald_check(&a, &b, &c, &r));
}

fn freivald_check<T: Field> (a: &Array2<T>, b: &Array2<T>, c: &Array2<T>, r: &T) -> Decision {
    let mut test_vector = Array1::<T>::default(N);
    let mut r_power = T::one();
    for d in test_vector.iter_mut() {
        *d = r_power;
        r_power = r_power * r;
    }

    let test_product = c.dot(&test_vector);
    let actual_product = a.dot(&b.dot(&test_vector));

    Decision::from_bool(test_product == actual_product)
}

fn fill_random<T: Field, D: Dimension> (arr: &mut Array<T, D>, rng: &mut ThreadRng) {
    for d in arr.iter_mut() {
        *d = T::rand(rng);
    }
}

enum Decision {
    Accept,
    Reject,
}

impl Decision {
    fn from_bool(b: bool) -> Decision {
        if b { Decision::Accept } else { Decision::Reject }
    }
}

impl fmt::Display for Decision {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Decision::Accept => {
                write!(f, "Accept")
            }
            Decision::Reject => {
                write!(f, "Reject")
            }
        }
    }
}
