use std::hash::{Hash, Hasher};
use std::fmt::{self, Debug, Formatter};
use rand::{Rand, Rng};

//#[derive_Rand]
#[derive(PartialEq, Eq, Copy, Clone)]
#[derive(Debug)]
// this used to work, no? #[derive(FromPrimitive)]
pub enum CodePeg {
  Red, Orn, Yel,
  Grn, Blu, Wht
}
use self::CodePeg::*;

#[derive(PartialEq, Eq, Copy, Clone)]
#[derive(PartialOrd, Ord)]
pub struct Pattern (pub usize);

#[derive(Debug)]
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Distance {
    blacks: usize,
    whites: usize
}

impl Distance {
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
    pub fn size() -> usize {
        4
    }

    #[inline(always)]
    pub fn radix() -> usize {
        6 // CodePeg::Wht.to_usize().unwrap() + 1
    }

    pub fn cardinality() -> usize {
        Pattern::radix().pow(Pattern::size() as u32) as usize
    }

    pub fn ith(lex_ix: usize) -> Pattern {
        assert!(lex_ix <= Pattern::cardinality());
        Pattern(lex_ix)
    }

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


impl Rand for Pattern {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let which = rng.gen::<u32>() % Pattern::cardinality() as u32;
        Pattern::ith(which as usize)
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
