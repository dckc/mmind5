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

#[derive(Debug)]
enum KeyPeg {
  Black, White
}


#[derive(Debug)]
struct Pattern (Vec<CodePeg>);

#[derive(Debug)]
struct Distance (Vec<Option<KeyPeg>>);


impl Pattern {
    #[inline(always)]
    fn size() -> usize {
        4
    }

    fn new(pegs: Vec<CodePeg>) -> Pattern {
        assert!(pegs.len() == Pattern::size());
        Pattern(pegs)
    }

    //fn score(guess: &Pattern) -> Distance {
    //    let rightColorAndPlace = 
    //}
}


impl Rand for Pattern {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let mkpeg = |_| CodePeg::rand(rng);
        Pattern::new((0..4).map(mkpeg).collect())
    }
}


fn main() {
    let mut rng = thread_rng();
    let secret = Pattern::rand(&mut rng);
    println!("codemaker chooses: {:?}", secret);
}
