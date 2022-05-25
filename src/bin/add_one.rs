use rand;

use std::fmt;
use pazk::ip;
use pazk::ip::{IP, Role};
use pazk::ip::channel::{Channel};

fn main() {
    let n = rand::random::<u8>();
    println!("Random value: {}", n);

    println!("\nPrescribed prover transcript:");
    let good_prover = Add1Prover {};
    let verifier = Add1Verifier{ n };
    ip::execute(good_prover, verifier);

    println!("\nRandom prover transcript:");
    let bad_prover = RandomProver {};
    let verifier = Add1Verifier{ n };
    ip::execute(bad_prover, verifier);
}

#[derive(Clone)]
enum Data {
    Number(u8),
    Decision(bool),
}

impl Data {
    fn to_number(self) -> Option<u8> {
        if let Data::Number(n) = self { Some(n) } else { None }
    }

    fn _to_decision(self) -> Option<bool> {
        if let Data::Decision(d) = self { Some(d) } else { None }
    }
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Data::Number(n) => { write!(f, "{}", n) }
            Data::Decision(b) => {
                if *b {
                    write!(f, "Accept")
                } else {
                    write!(f, "Reject")
                }
            }
        }
    }
}

struct Add1Prover {}

impl IP<Data> for Add1Prover {
    fn execute(&self, ch: Channel<Data, Role>) {
        let n = ch.receive().to_number().unwrap();
        ch.send(Data::Number(n.wrapping_add(1)));
    }
}

struct Add1Verifier {
    n: u8,
}

impl IP<Data> for Add1Verifier {
    fn execute(&self, ch: Channel<Data, Role>) {
        ch.send(Data::Number(self.n));

        let n_plus_one = ch.receive().to_number().unwrap();
        let decision = Data::Decision(n_plus_one == self.n.wrapping_add(1));
        ch.send(decision);
    }
}

struct RandomProver {}

impl IP<Data> for RandomProver {
    fn execute(&self, ch: Channel<Data, Role>) {
        let _ = ch.receive();
        let m = rand::random::<u8>();
        ch.send(Data::Number(m));
    }
}
