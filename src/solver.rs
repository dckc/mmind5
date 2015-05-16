use std::collections::{BitSet, BitVec, HashMap};

use pattern::{Pattern, Distance};
use pattern::CodePeg::*;


pub struct Solver<'a> {
    codemaker: &'a Fn(&Pattern) -> Distance,
    guessed: Vec<Pattern>,
    s: BitSet,
}

// http://en.wikipedia.org/wiki/Mastermind_%28board_game%29#Five-guess_algorithm
impl<'a> Solver<'a> {
    pub fn new(codemaker: &'a Fn(&Pattern) -> Distance) -> Solver<'a> {
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
    pub fn play(self: &mut Self) -> Option<Pattern> {
        // 3. Play the guess to get a response of colored and white pegs.
        let guess = self.guessed[self.guessed.len() - 1];
        let d = (self.codemaker)(&guess);

        // If the response is four colored pegs, the game is won, the algorithm terminates.
        if d.win() {
            Some(guess)
        } else {
            // 5. Otherwise, remove from S any code that would not
            // give the same response if it (the guess) were the code.
            self.remove_mismatches(d, guess);

            // From the set of guesses with the maximum score, select one as
            // the next guess ...
            self.choose_guess();

            None
        }
    }

    pub fn last_guess(self: &Self) -> Pattern {
        *self.guessed.last().expect("guesses starts with 1 and never shrinks")
    }

    // 5. Otherwise, remove from S any code that would not
    // give the same response if it (the guess) were the code.
    fn remove_mismatches(self: &mut Self, d: Distance, guess: Pattern) {
        // println!("remove_mismatches: before: {}", self.s.len());

        for code in 0..Pattern::cardinality() {
            if self.s.contains(&code) {
                let p = Pattern::ith(code);
                let pd = guess.score(p);
                if pd != d {
                    // println!("removing {:?}: {:?} != {:?}", p, pd, d);
                    self.s.remove(&code);
                }
            }
        }
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
        assert!(self.s.len() > 0);

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

            let highest_hit_count = dist_by_hits
                .values()
                .max()
                .expect("no max hit count: empty S? already won?");
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


    // 6. Apply minimax technique to find a next guess as follows ...
    fn choose_guess(self: &mut Self) {
        // From the set of guesses with the maximum score, ...
        let best_guesses = {
            let guesses_by_score = self.unused_guess_scores();
            let best_score = guesses_by_score.keys().max()
                .expect("no guess scores; empty S? already won?");
            let mut guesses = guesses_by_score[best_score].clone();

            // (Knuth follows the convention of choosing the guess
            // with the least numeric value)
            guesses.sort();
            guesses
        };
                
        // ... select one as
        // the next guess, choosing a member of S whenever
        // possible.
        let best_s = {
            let in_s = |guess: &Pattern| match *guess {
                Pattern(ix) => self.s.contains(&ix)
            };
            best_guesses.iter().find(|g| in_s(*g))
        };

        match best_s {
            Some(g) => self.guessed.push(*g),
            None => self.guessed.push(best_guesses[0]) // TODO: .expect()
        }
    }
}
