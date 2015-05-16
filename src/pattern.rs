//! The game is played using *code pegs* of six different colors.
//! The codemaker chooses a pattern of four code pegs.
use std::hash::{Hash, Hasher};
use std::fmt::{self, Debug, Formatter};


/// The game is played using *code pegs* of six different colors.
#[derive(PartialEq, Eq, Copy, Clone)]
#[derive(Debug)]
// this used to work, no? #[derive(FromPrimitive)]
pub enum CodePeg {
  Red, Orn, Yel,
  Grn, Blu, Wht
}
use self::CodePeg::*;


/// The codemaker chooses a pattern of four code pegs. Duplicates are
/// allowed, so the player could even choose four code pegs of the same
/// color.
#[derive(PartialEq, Eq, Copy, Clone)]
#[derive(PartialOrd, Ord)]
pub struct Pattern (pub usize);


/// A colored or black key peg is placed for each code peg from
/// the guess which is correct in both color and position. A white key
/// peg indicates the existence of a correct color code peg placed in
/// the wrong position.
#[derive(Debug)]
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Distance {
    blacks: usize,
    whites: usize
}

impl Distance {
    /// If the response is four colored pegs, the game is won.
    pub fn win(self) -> bool {
        self.blacks == Pattern::size()
    }
}

impl Hash for Distance {
    fn hash<H>(&self, state: &mut H) 
        where H: Hasher {
        (self.blacks, self.whites).hash(state)
    }
}


impl Pattern {
    #[inline(always)]
    /// The codemaker chooses a pattern of four code pegs.
    pub fn size() -> usize {
        4
    }

    #[inline(always)]
    /// The game is played using code pegs of six different colors.
    pub fn radix() -> usize {
        6 // CodePeg::Wht.to_usize().unwrap() + 1
    }

    /// Size of the set 1296 possible codes, 1111,1112,.., 6666
    pub fn cardinality() -> usize {
        Pattern::radix().pow(Pattern::size() as u32) as usize
    }

    /// Construct a pattern from a lexical index.
    pub fn ith(lex_ix: usize) -> Pattern {
        assert!(lex_ix <= Pattern::cardinality());
        Pattern(lex_ix)
    }

    /// Construct a Pattern from CodePegs.
    pub fn new(pegs: [CodePeg; 4]) -> Pattern {

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

    /// Decode a Pattern into CodePegs.
    pub fn pegs(&self) -> [CodePeg; 4] {
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

    /// The codemaker provides feedback by placing
    /// from zero to four key pegs in the small holes of the row with the
    /// guess. A colored or black key peg is placed for each code peg from
    /// the guess which is correct in both color and position. A white key
    /// peg indicates the existence of a correct color code peg placed in
    /// the wrong position.
    pub fn score(self: &Pattern, guess: Pattern) -> Distance {
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


#[cfg(test)]
mod tests {
    use super::{Pattern, Distance};
    use super::CodePeg::*;

    #[test]
    fn scoring() {
        let (s, g) = (Pattern::new([Orn, Grn, Grn, Blu]),
                      Pattern::new([Red, Red, Orn, Orn]));
        let t1 = s.score(g);
        assert_eq!(t1, Distance { blacks: 0, whites: 1 });
    }
}
