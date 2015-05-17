//! The game is played using *code pegs* of six different colors.
//! The codemaker chooses a pattern of four code pegs.
use std::hash::{Hash, Hasher};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::iter;
use std::ops::Range;

/// The codemaker chooses a pattern of four code pegs. Duplicates are
/// allowed, so the player could even choose four code pegs of the same
/// color.
#[derive(PartialEq, Eq, Copy, Clone)]
#[derive(PartialOrd, Ord)]
pub struct Pattern (u32);


/// A colored or black key peg is placed for each code peg from
/// the guess which is correct in both color and position. A white key
/// peg indicates the existence of a correct color code peg placed in
/// the wrong position.
#[derive(Debug)]
#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Distance {
    blacks: u8,
    whites: u8
}

impl Distance {
    /// If the response is four colored pegs, the game is won.
    pub fn win(self) -> bool {
        self.blacks as usize == Pattern::size()
    }
}

impl Display for Distance {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let s = iter::repeat('B').take(self.blacks as usize)
            .chain(iter::repeat('W').take(self.whites as usize))
            .collect::<String>();
        fmt.write_str(&s)
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
    pub fn radix() -> u8 {
        6
    }

    /// Size of the set 1296 possible codes, 1111,1112,.., 6666
    pub fn cardinality() -> u32 {
        (Pattern::radix() as u32).pow(Pattern::size() as u32)
    }

    /// Construct a pattern from a lexical index.
    pub fn ith(lex_ix: u32) -> Pattern {
        assert!(lex_ix <= Pattern::cardinality());
        Pattern(lex_ix)
    }

    pub fn index(&self) -> u32 {
        return self.0
    }

    pub fn range() -> iter::Map<Range<u32>, fn(u32) -> Pattern > {
        (0..Pattern::cardinality()).map(Pattern::ith)
    }

    /// Construct a Pattern from digits 1-6.
    /// Characters other than 1-6 are treated as '1'.
    pub fn from_digits(digits: [char; 4]) -> Pattern {
        let base = Pattern::radix() as u32;
        let digit = |pos: usize| digits[pos].to_digit(base).unwrap_or(1) - 1;
        let ix = digit(0) + base * (digit(1) + base * (digit(2) + base * digit(3)));
        Pattern(ix)
    }

    /// Decode a Pattern into digits
    pub fn to_digits(&self) -> [char; 4] {
        let arb = '1';
        let mut out = [arb; 4];
        let mut ith = self.0;

        for pos in 0..Pattern::size() {
            let remainder = (ith % Pattern::radix() as u32) as u8;
            let digit = (('1' as u8) + remainder) as char;
            ith = ith / Pattern::radix() as u32;
            out[pos as usize] = digit;
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
        let s = self.to_digits();
        let g = guess.to_digits();

        let right_place = |pos: &usize| s[*pos] == g[*pos];
        let g_used: Vec<_> = (0..Pattern::size()).filter(right_place).collect();
        let blacks = g_used.len() as u8;
        
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
        let whites = s_used.len() as u8 - blacks;
        
        Distance { blacks: blacks, whites: whites }
    }
}


impl Debug for Pattern {
    fn fmt(self: &Pattern, fmt: &mut Formatter) -> fmt::Result {
        let digits = self.to_digits();
        fmt.write_fmt(format_args!("{}{}{}{}", digits[0], digits[1], digits[2], digits[3]))
    }
}


impl Display for Pattern {
    fn fmt(self: &Pattern, fmt: &mut Formatter) -> fmt::Result {
        let digits = self.to_digits();
        fmt.write_fmt(format_args!("{}{}{}{}", digits[0], digits[1], digits[2], digits[3]))
    }
}


#[cfg(test)]
mod tests {
    use super::{Pattern, Distance};

    #[test]
    fn scoring() {
        let (s, g) = (Pattern::from_digits(['1', '2', '3', '4']),
                      Pattern::from_digits(['2', '5', '5', '5']));
        let t1 = s.score(g);
        assert_eq!(t1, Distance { blacks: 0, whites: 1 });
    }

    #[test]
    fn scoring_win() {
        let (s, g) = (Pattern::from_digits(['1', '2', '3', '4']),
                      Pattern::from_digits(['1', '2', '3', '4']));
        let t1 = s.score(g);
        assert_eq!(t1, Distance { blacks: 4, whites: 0 });
    }
}
