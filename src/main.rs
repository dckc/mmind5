#![feature(collections)]

extern crate rand;

use rand::distributions::{IndependentSample, Range};

pub mod gameplay;
pub mod solver;

use gameplay::{DecodingBoard, Pattern, shield};
use solver::{Solver};


/// One player becomes the *codemaker*, the other the
/// *codebreaker*. Guesses and feedback continue to alternate until
/// either the codebreaker guesses correctly, or ten incorrect guesses
/// are made.
pub fn main() {
    use rand::{thread_rng};

    let secret = {
        let rng = &mut thread_rng();
        let r = Range::new(0, Pattern::cardinality());
        let x = r.ind_sample(rng);
        Pattern::ith(x)
    };
    println!("codemaker: {}", secret);

    let maker = shield(secret);

    let breaker = Solver::new(maker);

    // TODO: support twelve (or ten, or eight) CLI arg
    let rows = DecodingBoard::default().rows as usize;

    for (turn, g) in breaker.take(rows).enumerate() {
        println!("turn {}:    {}  {}",
                 turn + 1, g, secret.score(g));
    }
}
