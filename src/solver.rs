//! Mastermind board game solver using [Knuth's five guess algorithm][wp5]
//!
//! [wp5]: http://en.wikipedia.org/wiki/Mastermind_%28board_game%29#Five-guess_algorithm

use std::collections::{BitSet, BitVec, HashMap};

use gameplay::{Pattern, KeyPegs, Shield};


pub struct Solver {
    codemaker: Shield,
    guessed: Vec<Pattern>,
    s: PatternSet,
}

impl Solver {

    /// - 1. Create the set S of 1296 possible codes, 1111,1112,.., 6666.
    pub fn new(codemaker: Shield) -> Solver {
        let possible_codes = PatternSet::all();

        Solver { codemaker: codemaker,
                 s: possible_codes,
                 guessed: vec![] }
    }

    /// - 2. Start with initial guess 1122
    /// - 3. Play the guess to get a response of colored and white pegs.
    /// - 4. If the response is four colored pegs, the game is won, the algorithm terminates.
    /// - 5. Otherwise, remove from S any code that would not
    ///   give the same response if it (the guess) were the code.
    ///   From the set of guesses with the maximum score, select one as
    ///   the next guess ...
    ///
    /// Return Some(guess) or None if we already won.
    pub fn play(self: &mut Self) -> Option<Pattern> {
        if self.guessed.is_empty() {
            let initial_guess = Pattern::from_digits(['1', '1', '2', '2']);
            self.guessed.push(initial_guess);
            Some(initial_guess)
        } else {
            let prev = *self.guessed.last().expect("initial guess gone?!");
            // 3. Play the guess to get a response of colored and white pegs.
            let d = (self.codemaker)(&prev);

            // If the response is four colored pegs, the game is won, the algorithm terminates.
            if d.win() {
                None
            } else {
                // 5. Otherwise, remove from S any code that would not
                // give the same response if it (the guess) were the code.
                self.remove_mismatches(d, prev);

                // From the set of guesses with the maximum score, select one as
                // the next guess ...
                let ng = self.next_guess();
                self.guessed.push(ng);

                Some(ng)
            }
        }
    }

    pub fn last_guess(self: &Self) -> Pattern {
        *self.guessed.last().expect("guesses starts with 1 and never shrinks")
    }

    // 5. Otherwise, remove from S any code that would not
    // give the same response if it (the guess) were the code.
    fn remove_mismatches(self: &mut Self, d: KeyPegs, guess: Pattern) {
        // println!("remove_mismatches: before: {}", self.s.len());

        for p in Pattern::range() {
            if self.s.contains(&p) {
                let pd = guess.score(p);
                if pd != d {
                    // println!("removing {:?}: {:?} != {:?}", p, pd, d);
                    self.s.remove(&p);
                }
            }
        }
    }

    /// - 6. Apply minimax technique to find a next guess as follows ...
    ///      From the set of guesses with the maximum score, 
    ///      select one as
    ///      the next guess, choosing a member of S whenever
    ///      possible.
    pub fn next_guess(&self) -> Pattern {
        // From the set of guesses with the maximum score, ...
        let best_guesses = self.best_guesses();
                
        // ... select one as
        // the next guess, choosing a member of S whenever
        // possible.
        let best_s = best_guesses.iter().find(|g| self.s.contains(g));

        match best_s {
            Some(g) => *g,
            None => best_guesses[0] // TODO: .expect()
        }
    }


    /// For each possible guess, that is, any unused code of the 1296
    /// not just those in S, calculate how many possibilities in S
    /// would be eliminated for each possible colored/white peg score.
    /// The score of a guess is the minimum number of possibilities it
    /// might eliminate from S. A single pass through S for each unused
    /// code of the 1296 will provide a hit count for each
    /// colored/white peg score found; the colored/white peg score with
    /// the highest hit count will eliminate the fewest possibilities;
    pub fn best_guesses(self: &Self) -> Vec<Pattern>
    {
        assert!(self.s.len() > 0);

        // calculate the score of a guess by using "minimum eliminated" =
        // "count of elements in S" - (minus) "highest hit count".
        let guess_score = |g: Pattern| {
            let mut dist_by_hits = HashMap::new();
            for other in Pattern::range().filter(|p| self.s.contains(p)) {
                let d = g.score(other);
                *(dist_by_hits.entry(d).or_insert(0)) += 1;
            }

            let highest_hit_count = dist_by_hits
                .values()
                .max()
                .expect("no max hit count: empty S? already won?");
            self.s.len() - highest_hit_count
        };

        let append = |xs: Vec<Pattern>, x| {
            let mut v = xs;
            v.push(x);
            v
        };

        let highest = |acc: (usize, Vec<Pattern>), p| {
            let (high_score, candidates) = acc;
            let score = guess_score(p);
            if score > high_score {
                (score, vec![p])
            } else if score == high_score {
                (score, append(candidates, p))
            } else {
                (high_score, candidates)
            }
        };

        let sorted = |ps: Vec<Pattern>| {
            let mut work = ps;
            work.sort();
            work
        };

        let (_, high_scoring_guesses) = Pattern::range().fold((0, vec![]), highest);
        // (Knuth follows the convention of choosing the guess
        // with the least numeric value)
        sorted(high_scoring_guesses)
    }
}


impl Iterator for Solver {
    type Item = Pattern;

    fn next(&mut self) -> Option<Pattern> {
        self.play()
    }
}


pub struct PatternSet {
    indexes: BitSet
}

impl PatternSet {
    pub fn all() -> PatternSet {
        let all_vec = BitVec::from_elem(Pattern::cardinality() as usize, true);
        let all_ix = BitSet::from_bit_vec(all_vec);

        PatternSet { indexes: all_ix }
    }

    pub fn len(&self) -> usize {
        self.indexes.len()
    }

    pub fn contains(&self, p: &Pattern) -> bool {
        let ix = p.index() as usize;
        self.indexes.contains(&ix)
    }

    pub fn remove(&mut self, p: &Pattern) -> bool {
        let ix = p.index() as usize;
        self.indexes.remove(&ix)
    }
}
