pub mod channel;
pub use self::channel::*;

use std::thread;
use std::fmt::Display;

#[derive(Copy,Clone)]
pub enum Role {
    Prover,
    Verifier,
}

// 2-party interactive protocol
pub trait IP<T: Clone> {
    fn execute(&self, ch: Channel<T, Role>);
}

pub fn execute<T: Clone + Send + 'static + Display>(
        prover: impl IP<T> + Send + 'static,
        verifier: impl IP<T> + Send + 'static) {
    let coord: Coordinator<T, Role> = Coordinator::new();
    let (ch1, ch2) = coord.create_channel(Role::Prover, Role::Verifier);

    let prover_handle = thread::spawn(move || { prover.execute(ch1); });
    let verifier_handle = thread::spawn(move || { verifier.execute(ch2); });

    prover_handle.join().unwrap();
    verifier_handle.join().unwrap();

    let log = &*(coord.get_log().lock().unwrap());
    for m in log {
        match m.producer {
            Role::Prover => { println!("P: {}", m.data); }
            Role::Verifier => { println!("V: {}", m.data); }
        }
    }
}
