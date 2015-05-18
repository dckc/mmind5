//! Mastermind board game solver using Knuth's five guess algorithm
//!
//! With four pegs and six colors, there are 6^4 = 1296 different patterns (allowing duplicate colors).
//!
//! ```rust
//! use self::mastermind::gameplay::{CodePeg, Pattern};
//!
//! assert_eq!((CodePeg::colors() as u32).pow(Pattern::size() as u32), 1296);
//! assert_eq!(Pattern::cardinality(), 1296);
//! ```
//!
//! In 1977, Donald Knuth demonstrated that the codebreaker can solve
//! the pattern in five moves or fewer, using an algorithm that
//! progressively reduced the number of possible patterns. The
//! algorithm works as follows:
//!
//! 1. Create the set S of 1296 possible codes, 1111,1112,.., 6666.
//! 2. Start with initial guess 1122 (Knuth gives examples showing
//!    that some other first guesses such as 1123, 1234 do not win in
//!    five tries on every code).
//! 3. Play the guess to get a response of colored and white pegs.
//! 4. If the response is four colored pegs, the game is won, the
//!    algorithm terminates.
//! 5. Otherwise, remove from S any code that would not give the same
//!    response if it (the guess) were the code.
//! 6. Apply minimax technique to find a next guess as follows: For each
//!    possible guess, that is, any unused code of the 1296 not just those
//!    in S, calculate how many possibilities in S would be eliminated for
//!    each possible colored/white peg score. The score of a guess is the
//!    minimum number of possibilities it might eliminate from S. A single
//!    pass through S for each unused code of the 1296 will provide a hit
//!    count for each colored/white peg score found; the colored/white peg
//!    score with the highest hit count will eliminate the fewest
//!    possibilities; calculate the score of a guess by using "minimum
//!    eliminated" = "count of elements in S" - (minus) "highest hit
//!    count". From the set of guesses with the maximum score, select one
//!    as the next guess, choosing a member of S whenever possible. (Knuth
//!    follows the convention of choosing the guess with the least numeric
//!    value e.g. 2345 is lower than 3456. Knuth also gives an example
//!    showing that in some cases no member of S will be among the highest
//!    scoring guesses and thus the guess cannot win on the next turn, yet
//!    will be necessary to assure a win in five.)
//! 7. Repeat from step 3.
//!
//! ```rust
//! use self::mastermind::gameplay::{Pattern, KeyPegs};
//! use self::mastermind::solver::Solver;
//!
//! let code1 = Pattern::from_digits(['1', '1', '2', '2']);
//! let codemaker_easy = Box::new(move |guess: &Pattern| code1.score(*guess));
//!
//! let s = Solver::possible_codes();
//! assert_eq!(s.len(), 1296);
//! assert_eq!(format!("{},{},.., {}",
//!                     Pattern::ith(0),
//!                     Pattern::ith(1),
//!                     Pattern::ith(Pattern::cardinality() - 1)),
//!            "1111,1112,.., 6666");
//!
//! assert_eq!(format!("{}", Solver::initial_guess()), "1122");
//!
//! let mut breaker1 = Solver::new(codemaker_easy);
//! match breaker1.play() {
//!   None => panic!("0 guesses from breaker1?!"),
//!   Some(g) => {
//!     let response = code1.score(g);
//!     assert!(response.win())
//!   }
//! }
//! assert_eq!(breaker1.play(), None);
//! ```
//!
//! ```rust
//! use self::mastermind::gameplay::{Pattern, KeyPegs};
//! use self::mastermind::solver::Solver;
//! let code2 = Pattern::from_digits(['1', '1', '2', '3']);
//! let codemaker_harder = Box::new(move |guess: &Pattern| code2.score(*guess));
//!
//! let mut breaker2 = Solver::new(codemaker_harder);
//! let guess1 = breaker2.play().expect("0 guesses!?");
//! let response = code2.score(guess1);
//! assert_eq!(response.win(), false);
//! assert_eq!(response, KeyPegs::new().blacks(3));
//!
//! breaker2.retain_same_response(response);
//! assert!(!breaker2.s.contains(&Pattern::from_digits(['5', '2', '2', '3'])));
//! let keep = Pattern::from_digits(['5', '1', '2', '2']);
//! assert_eq!(guess1.score(keep), response);
//! assert!( breaker2.s.contains(&keep));
//! ```
//!
//! TODO: test for steps 5, 6, 7
//!
//! [Knuth's five guess algorithm][wp5]
//! [wp5]: http://en.wikipedia.org/wiki/Mastermind_%28board_game%29#Five-guess_algorithm

use std::collections::{BitSet, BitVec, HashMap};

use gameplay::{Pattern, KeyPegs, Shield};


pub struct Solver {
    codemaker: Shield,
    pub guessed: Vec<Pattern>,
    pub s: PatternSet,
}

impl Solver {
    /// - 1. Create the set S of 1296 possible codes, 1111,1112,.., 6666.
    pub fn possible_codes() -> PatternSet {
        PatternSet::all()
    }

    pub fn new(codemaker: Shield) -> Solver {
        Solver { codemaker: codemaker,
                 s: Solver::possible_codes(),
                 guessed: vec![] }
    }

    /// Start with initial guess 1122
    pub fn initial_guess() -> Pattern {
        Pattern::from_digits(['1', '1', '2', '2'])
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
            let guess = Self::initial_guess();
            self.guessed.push(guess);
            Some(guess)
        } else {
            let prev = self.last_guess();
            // 3. Play the guess to get a response of colored and white pegs.
            let response = (self.codemaker)(&prev);

            // If the response is four colored pegs, the game is won, the algorithm terminates.
            if response.win() {
                None
            } else {
                // 5. Otherwise, remove from S any code that would not
                // give the same response if it (the guess) were the code.
                self.retain_same_response(response);

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
    //    give the same response if it (the guess) were the code.
    pub fn retain_same_response(&mut self, response: KeyPegs) {
        let the_guess = self.last_guess();

        self.s.filter_with(&|p: &Pattern| the_guess.score(*p) == response)
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

        let unused = |p: &Pattern| !self.guessed.contains(p);
        let (_, high_scoring_guesses) = Pattern::range()
            .filter(unused)
            .fold((0, vec![]), highest);

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

    pub fn filter_with(&mut self, predicate: &Fn(&Pattern) -> bool) {
        for p in Pattern::range() {
            let ix = p.index() as usize;
            if self.indexes.contains(&ix) && !predicate(&p) {
                self.indexes.remove(&ix);
            }
        }
    }

    pub fn remove(&mut self, p: &Pattern) -> bool {
        let ix = p.index() as usize;
        self.indexes.remove(&ix)
    }
}
