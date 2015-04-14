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
use CodePeg::*;

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

    fn ith(lex_ix: u32) -> Pattern {
        assert!(lex_ix <= Pattern::cardinality());
        Pattern(lex_ix)
    }

    fn new(pegs: [CodePeg; 4]) -> Pattern {

        // what happened to ToPrimitive and to_usize?
        let encode = |pos| match pegs[pos] {
            Red => 0,
            Orn => 1,
            Yel => 2,
            Grn => 3,
            Blu => 4,
            Wht => 5
        };
        let radix = Pattern::radix();
        let ix = encode(0) + radix * (encode(1) + radix * (encode(2) + radix * encode(3)));
        Pattern(ix)
    }

    fn pegs(&self) -> [CodePeg; 4] {
        let arb = Red;
        let mut out = [arb; 4];
        let mut ith = self.0;

        for pos in 0..Pattern::size() {
            let it = match ith % Pattern::radix() as u32 {
                // what happened to FromPrimitive and from_usize?
                0 => Red,
                1 => Orn,
                2 => Yel,
                3 => Grn,
                4 => Blu,
                5 => Wht,
                _ => panic!("ith % size() > 5")
            };
            ith = ith / Pattern::radix() as u32;
            out[pos] = it;
        }

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
        let which = rng.gen::<u32>() % Pattern::cardinality();
        Pattern::ith(which)
    }
}


#[derive(Debug)]
struct Solver {
    guess: Pattern
}

impl Solver {
    fn new() -> Solver {
        // 2. Start with initial guess 1122
        let initial_guess = [Red, Red, Orn, Orn];
        Solver { guess: Pattern::new(initial_guess) }
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

    let mut breaker = Solver::new();
    println!("Five guess codebreaker: {:?}", breaker);
}
