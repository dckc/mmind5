#![feature(plugin, custom_derive)]
#![plugin(rand_macros)]

extern crate rand;

use rand::{Rand, Rng, thread_rng};

#[derive_Rand]
#[derive(PartialEq, Eq, Debug)]
enum CodePeg {
  Red, Orn, Yel,
  Grn, Blu, Wht
}

#[derive(Debug)]
struct Pattern (Vec<CodePeg>);

#[derive(Debug)]
struct Distance {
    blacks: usize,
    whites: usize
}


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
                let right_place = |pos: &usize| s[*pos] == g[*pos];
                let mut s_used: Vec<_> = (0..Pattern::size()).filter(right_place).collect();
                let blacks = s_used.len();

                let mut g_used = s_used.clone();

                for gpos in 0..Pattern::size() {
                    // Find an unused "self" peg of the same color.
                    let scan = (0..Pattern::size()).find(
                        |spos| s[*spos] == g[gpos]
                            && !s_used.contains(spos)
                            && !g_used.contains(&gpos));
                    match scan {
                        Some(spos) => {
                            s_used.push(spos);
                            g_used.push(gpos);
                        }
                        None => ()
                    }
                }
                let whites = s_used.len() - blacks;

                Distance { blacks: blacks, whites: whites }
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
    println!("codemaker: {:?}", secret);
    for _ in 0..10 {
        let guess = Pattern::rand(&mut rng);
        println!("guess    : {:?}", guess);
        println!("...........feedback: {:?}", secret.score(&guess));
    }
}
