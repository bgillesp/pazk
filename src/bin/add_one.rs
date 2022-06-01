use rand;

use std::fmt;
use pazk::ip;
use pazk::ip::{IP,Channel,Log};

fn main() {
    let n = rand::random::<u8>();
    println!("Random value: {}", n);

    println!("\nPrescribed prover");
    println!(  "=================");
    let good_prover = Add1Prover {};
    let verifier = Add1Verifier{ n };
    ip::execute(good_prover, verifier);

    println!("\nRandom prover");
    println!(  "=============");
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
    fn execute(&self, ch: Channel<Data>, log: Log) {
        let n = ch.receive().to_number().unwrap();

        log.write("P computes m = n+1".to_string());
        let m = n.wrapping_add(1);

        log.write(format!("P --> (m={})", m));
        ch.send(Data::Number(m));
    }
}

struct Add1Verifier {
    n: u8,
}

impl IP<Data> for Add1Verifier {
    fn execute(&self, ch: Channel<Data>, log: Log) {
        log.write(format!("V starts with value n={}", self.n));

        log.write(format!("V --> (n={})", self.n));
        ch.send(Data::Number(self.n));

        let m = ch.receive().to_number().unwrap();

        log.write(format!("V checks m == n+1"));
        let decision = Data::Decision(m == self.n.wrapping_add(1));

        log.write(format!("V --> ({})", decision));
        ch.send(decision);
    }
}

struct RandomProver {}

impl IP<Data> for RandomProver {
    fn execute(&self, ch: Channel<Data>, log: Log) {
        let _ = ch.receive();

        let m = rand::random::<u8>();
        log.write("P picks m uniformly at random".to_string());

        log.write(format!("P --> (m={})", m));
        ch.send(Data::Number(m));
    }
}
