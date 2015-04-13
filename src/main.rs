#![feature(plugin, custom_derive)]
#![plugin(rand_macros)]

extern crate rand;

use rand::{Rand, Rng, thread_rng};

#[derive_Rand]
#[derive(Debug)]
enum CodePeg {
  Red, Orange, Yellow,
  Green, Blue, White
}

enum KeyPeg {
  Black, White
}


#[derive(Debug)]
struct Pattern {
  pegs: [CodePeg; 4]
}

impl Rand for Pattern {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        Pattern {
            pegs: [CodePeg::rand(rng),
                   CodePeg::rand(rng),
                   CodePeg::rand(rng),
                   CodePeg::rand(rng)]
        }
    }
}


struct Distance {
  pegs: [Option<KeyPeg>; 4]
}

fn main() {
    let mut rng = thread_rng();
    let secret = Pattern::rand(&mut rng);
    println!("codemaker chooses: {:?}", secret);
}
