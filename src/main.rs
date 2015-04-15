#![feature(collections)]
#![feature(plugin, custom_derive)]
#![plugin(rand_macros)]
#![feature(debug_builders)]

extern crate rand;

use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::iter::{FromIterator};
use std::fmt::{self, Debug, Formatter};
use std::collections::{BitSet, BitVec, HashMap};
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

impl Distance {
    fn win(self) -> bool {
        self.blacks == Pattern::size()
    }
}

impl Hash for Distance {
    fn hash<H>(&self, state: &mut H) 
        where H: Hasher {
        (self.blacks, self.whites).hash(state)
    }
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


struct Solver<'a> {
    codemaker: &'a Fn(&Pattern) -> Distance,
    guessed: Vec<Pattern>,
    s: BitSet,
}

// http://en.wikipedia.org/wiki/Mastermind_%28board_game%29#Five-guess_algorithm
impl<'a> Solver<'a> {
    fn new(codemaker: &'a Fn(&Pattern) -> Distance) -> Solver<'a> {
        // 1. Create the set S of 1296 possible codes, 1111,1112,.., 6666.
        let possible_codes = BitSet::from_bit_vec(
            BitVec::from_fn(Pattern::cardinality() as usize, |_| true));
        // 2. Start with initial guess 1122
        let initial_guess = [Red, Red, Orn, Orn];
        Solver { codemaker: codemaker,
                 s: possible_codes,
                 guessed: vec![Pattern::new(initial_guess)] }
    }

    // Return Some(winning_pattern) or None if we need another turn.
    fn play(self: &mut Self) -> Option<Pattern> {
        // 3. Play the guess to get a response of colored and white pegs.
        let guess = self.guessed[self.guessed.len() - 1];
        let d = (self.codemaker)(&guess);

        // If the response is four colored pegs, the game is won, the algorithm terminates.
        if d.win() {
            Some(guess)
        } else {
            // 5. Otherwise, remove from S any code that would not
            // give the same response if it (the guess) were the code.
            self.remove_mismatches(d);

            // TODO:
    // From the set of guesses with the maximum score, select one as
    // the next guess, choosing a member of S whenever
    // possible. (Knuth follows the convention of choosing the guess
    // with the least numeric value e.g. 2345 is lower than
    // 3456. Knuth also gives an example showing that in some cases no
    // member of S will be among the highest scoring guesses and thus
    // the guess cannot win on the next turn, yet will be necessary to
    // assure a win in five.)
            self.choose_guess();

            None
        }
    }

    // 5. Otherwise, remove from S any code that would not
    // give the same response if it (the guess) were the code.
    fn remove_mismatches(self: &mut Self, d: Distance) {
        let to_keep = {
            let same_response = || {
                self.s.iter().filter(|g| (self.codemaker)(&Pattern::ith(*g)) == d)
            };
            BitSet::from_iter(same_response())
        };

        self.s.intersect_with(&to_keep);
    }

    // For each possible guess, that is, any unused code of the 1296
    // not just those in S, calculate how many possibilities in S
    // would be eliminated for each possible colored/white peg score.
    // The score of a guess is the minimum number of possibilities it
    // might eliminate from S. A single pass through S for each unused
    // code of the 1296 will provide a hit count for each
    // colored/white peg score found; the colored/white peg score with
    // the highest hit count will eliminate the fewest possibilities;
    fn unused_guess_scores(self: &Self)
        -> HashMap<usize, Vec<Pattern>>
    {
        // TODO: figure out how to move iter_d to Distance::iter
        let qty = Pattern::size();
        let iter_d = || {
            (0..qty+1)
                .flat_map(|blacks: usize| {
                    (0..qty+1)
                        .filter(move |whites| blacks + whites <= qty)
                        .map(move |whites: usize| Distance { blacks: blacks, whites: whites })
                })
        };

        // calculate the score of a guess by using "minimum eliminated" =
        // "count of elements in S" - (minus) "highest hit count".
        let guess_score = |gix| {
            let mut dist_by_hits = HashMap::new();
            for other_ix in self.s.iter() {
                let g = Pattern::ith(gix);
                let other = Pattern::ith(other_ix);
                let d = g.score(other);
                *(dist_by_hits.entry(d).or_insert(0)) += 1;
            }

            let highest_hit_count = dist_by_hits.values().max().unwrap();
            self.s.len() - highest_hit_count
        };

        let mut guesses_with_score = HashMap::new();
        for guess_ix in 0..Pattern::cardinality() {
            if !self.guessed.contains(&Pattern::ith(guess_ix)) {
                let score = guess_score(guess_ix);
                let guess = Pattern::ith(guess_ix);
                guesses_with_score.entry(score).or_insert(vec![]).push(guess)
            }
        }
        guesses_with_score
    }

    fn choose_guess(self: &mut Self) {
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

    let maker = |guess: &Pattern| secret.score(*guess);

    let mut breaker = Solver::new(&maker);
    breaker.play();
    println!("Five guess codebreaker: {:?}", breaker.guessed);
}
