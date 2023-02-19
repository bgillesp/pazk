# ZK and VC Protocol Implementations

This repository contains implementations for a selection of zero-knowledge and verifiable computation protocols from Justin Thaler's book, "[Proofs, Arguments, and Zero-knowledge](https://people.cs.georgetown.edu/jthaler/ProofsArgsAndZK.html)."  Implementations are coded in Rust using the [Arkworks](https://github.com/arkworks-rs) ecosystem libraries for algebraic primitives.  The code design emphasizes:

* Protocol implementations which are readable and self-contained
* Style and presentation closely mirroring that of the source material
* Program output that can be reasoned about by hand

Currently the repository includes the following implementations:

* The multivariate [sum-check protocol](https://github.com/bgillesp/pazk/blob/main/src/bin/sum_check.rs) for proving that the sum over an exponentially-sized domain of a multivariate polynomial function is a given value
* [Freivalds' algorithm](https://github.com/bgillesp/pazk/blob/main/src/bin/freivald.rs) for efficient randomized verification of matrix products
* A non-succinct zero-knowledge protocol for [inner product relations](https://github.com/bgillesp/pazk/blob/main/src/bin/pedersen_poly_commitment.rs) of Pedersen commitments (Protocol 11)
* The [Bulletproofs protocol](https://github.com/bgillesp/pazk/blob/main/src/bin/bulletproof.rs) for inner product relations, requiring only logarithmic communication complexity using recursive folding (Protocol 13)
* [Small finite fields](https://github.com/bgillesp/pazk/blob/main/src/small_fields.rs) and [small elliptic curve groups](https://github.com/bgillesp/pazk/blob/main/src/small_curves.rs) implemented using the Arkworks algebra backend, to allow for protocol transcripts which are easier to follow
* A simple framework for threaded execution of 2-party [interactive proof protocols](https://github.com/bgillesp/pazk/blob/main/src/ip.rs)

**IMPORTANT:**  While this software aims to provide correct implementations of the relevant protocols, it is meant for academic and educational purposes, and has not been audited for security.  As such, it is strongly recommended not to use this code for production applications.
