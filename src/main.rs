#![feature(plugin, custom_derive)]
#![plugin(rand_macros)]

extern crate rand;

use rand::{Rand, Rng, thread_rng};

#[derive_Rand]
#[derive(PartialEq, Eq, Debug)]
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

    fn score(self: &Pattern, guess: &Pattern) -> Distance {
        match (self, guess) {
            (&Pattern(ref s), &Pattern(ref g)) => {
                let rightColorAndPlace = (0..Pattern::size()).map(|pos| {
                    if g[pos] == s[pos] { Some(KeyPeg::Black) }
                    else { None }
                }).collect();
                
                // TODO: white pegs
                Distance(rightColorAndPlace)
            }
        }
    }
}


impl Rand for Pattern {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let mkpeg = |_| CodePeg::rand(rng);
        Pattern::new((0..Pattern::size()).map(mkpeg).collect())
    }
}


fn main() {
    let mut rng = thread_rng();
    let secret = Pattern::rand(&mut rng);
    let guess1 = Pattern::rand(&mut rng);
    println!("codemaker chooses: {:?}", secret);
    println!("guess 1: {:?}", guess1);
    println!("feedback: {:?}", secret.score(&guess1));
}
