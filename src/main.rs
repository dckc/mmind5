#![feature(collections)]
#![feature(plugin, custom_derive)]
#![plugin(rand_macros)]
#![feature(debug_builders)]

extern crate rand;

use std::cmp::Ordering;
use std::iter::FromIterator;
use std::fmt::{self, Debug, Formatter};
use std::collections::{BitSet, BitVec};
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

#[derive(PartialEq, Eq, Copy, Clone)]
struct Pattern (usize);

#[derive(Debug)]
#[derive(PartialEq, Eq, Copy, Clone)]
struct Distance {
    blacks: usize,
    whites: usize
}

impl Ord for Distance {
    fn cmp(&self, other: &Self) -> Ordering {
        use std::cmp::Ordering::*;

        let total = |d: &Distance| d.blacks + d.whites;
        match (total(self).cmp(&total(other)), self.blacks.cmp(&other.blacks)) {
            (Greater, _) => Greater,
            (Less, _) => Greater,
            (_, Greater) => Greater,
            (_, Less) => Less,
            _ => Equal
        }
    }
}
impl PartialOrd for Distance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl Pattern {
    #[inline(always)]
    fn size() -> usize {
        4
    }

    #[inline(always)]
    fn radix() -> usize {
        6 // CodePeg::Wht.to_usize().unwrap() + 1
    }

    fn cardinality() -> usize {
        Pattern::radix().pow(Pattern::size() as u32) as usize
    }

    fn ith(lex_ix: usize) -> Pattern {
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
            let it = match ith % Pattern::radix() {
                // what happened to FromPrimitive and from_usize?
                0 => Red,
                1 => Orn,
                2 => Yel,
                3 => Grn,
                4 => Blu,
                5 => Wht,
                _ => panic!("ith % size() > 5")
            };
            ith = ith / Pattern::radix();
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
        let which = rng.gen::<u32>() % Pattern::cardinality() as u32;
        Pattern::ith(which as usize)
    }
}


#[derive(Debug)]
struct Solver {
    s: BitSet,
    guess: Pattern
}

// http://en.wikipedia.org/wiki/Mastermind_%28board_game%29#Five-guess_algorithm
impl Solver {
    fn new() -> Solver {
        // 1. Create the set S of 1296 possible codes, 1111,1112,.., 6666.
        let possible_codes = BitSet::from_bit_vec(
            BitVec::from_fn(Pattern::cardinality() as usize, |_| true));
        // 2. Start with initial guess 1122
        let initial_guess = [Red, Red, Orn, Orn];
        Solver { s: possible_codes, guess: Pattern::new(initial_guess) }
    }

    // Return Some(winning_pattern) or None if we need another turn.
    fn play(self: &mut Self, peg_score: &Fn(&Pattern) -> Distance) -> Option<Pattern> {
        // 3. Play the guess to get a response of colored and white pegs.
        let response = peg_score(&self.guess);

        // If the response is four colored pegs, the game is won, the algorithm terminates.
        if response.blacks == Pattern::size() {
            Some(self.guess)
        } else {
            // 5. Otherwise, remove from S any code that would not
            // give the same response if it (the guess) were the code.
            self.remove_mismatches(peg_score, response);

            // 6. Apply minimax technique ... TODO

            None
        }
    }

    fn remove_mismatches(self: &mut Self, peg_score: &Fn(&Pattern) -> Distance, response: Distance) {
        // 5. Otherwise, remove from S any code that would not
        // give the same response if it (the guess) were the code.
        let to_keep = {
            let same_response = || {
                self.s.iter().filter(|g| peg_score(&Pattern::ith(*g)) == response)
            };
            BitSet::from_iter(same_response())
        };

        self.s.intersect_with(&to_keep);
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
    breaker.play(&|g| secret.score(*g));
    println!("Five guess codebreaker: {:?}", breaker);
}
