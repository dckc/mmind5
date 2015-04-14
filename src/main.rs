#![feature(plugin, custom_derive)]
#![plugin(rand_macros)]
#![feature(debug_builders)]

extern crate rand;

use std::fmt::{self, Debug, Formatter};
use rand::{Rand, Rng};

#[derive_Rand]
#[derive(PartialEq, Eq, Copy, Clone)]
#[derive(Debug)]
// this used to work, no? #[derive(FromPrimitive)]
enum CodePeg {
  Red, Orn, Yel,
  Grn, Blu, Wht
}

struct Pattern (u32);

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

    #[inline(always)]
    fn radix() -> u32 {
        6 // CodePeg::Wht.to_usize().unwrap() + 1
    }

    fn cardinality() -> u32 {
        Pattern::radix().pow(Pattern::size() as u32)
    }

    fn new(lex_ix: u32) -> Pattern {
        assert!(lex_ix <= Pattern::cardinality());
        Pattern(lex_ix)
    }

    fn pegs(&self) -> [CodePeg; 4] {
        let arb = CodePeg::Red;
        let mut out = [arb; 4];
        let mut ith = self.0;
        let mut pos = 0;

        let mut next = || {
            let it = match ith % Pattern::size() as u32 {
                // what happened to FromPrimitive and from_usize?
                0 => CodePeg::Red,
                1 => CodePeg::Orn,
                2 => CodePeg::Yel,
                3 => CodePeg::Grn,
                4 => CodePeg::Blu,
                5 => CodePeg::Wht,
                _ => panic!("ith % size() > 5")
            };
            ith = ith / Pattern::size() as u32;
            it
        };

        out[pos] = next();
        pos += 1;
        out[pos] = next();
        pos += 1;
        out[pos] = next();
        pos += 1;
        out[pos] = next();
        
        out
    }

    fn score(self: &Pattern, guess: Pattern) -> Distance {
        let s = self.pegs();
        let g = guess.pegs();

        let right_place = |pos: &usize| s[*pos] == g[*pos];
        let g_used: Vec<_> = (0..Pattern::size()).filter(right_place).collect();
        let blacks = g_used.len();
        
        let mut s_used = g_used.clone();
        
        for gpos in 0..Pattern::size() {
            if !g_used.contains(&gpos) {
                // Find an unused "self" peg of the same color.
                let scan = (0..Pattern::size()).find(
                    |spos| s[*spos] == g[gpos] && !s_used.contains(spos));
                
                if let Some(spos) = scan {
                    s_used.push(spos);
                }
            }
                }
        let whites = s_used.len() - blacks;
        
        Distance { blacks: blacks, whites: whites }
    }
}

impl Debug for Pattern {
    fn fmt(self: &Pattern, fmt: &mut Formatter) -> fmt::Result {
        let parts = self.pegs();
        parts.iter().fold(fmt.debug_list(), |b, e| b.entry(e)).finish()
    }
}


impl Rand for Pattern {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let ith = rng.gen::<u32>() % Pattern::cardinality();
        Pattern::new(ith)
    }
}


fn main() {
    use rand::{thread_rng};

    let mut rng = thread_rng();
    let secret = Pattern::rand(&mut rng);
    println!("codemaker: {:?}", secret);
    for _ in 0..10 {
        let guess = Pattern::rand(&mut rng);
        println!("guess    : {:?}", guess);
        println!("...........feedback: {:?}", secret.score(guess));
    }
}
