//! Mastermind is a code-breaking game for two players.
//!
//! The game is played using:
//!
//!  - a *decoding board*, with a shield at one end covering a row of
//!    four large holes, and twelve (or ten, or eight, or six)
//!    additional rows containing four large holes next to a set of
//!    four small holes;
//!  - *code pegs* of six (or more; see Variations below) different
//!    colors, with round heads, which will be placed in the large holes
//!    on the board; and
//!  - *key pegs*, some colored black, some white, which are flat-headed
//!    and smaller than the code pegs; they will be placed in the small
//!    holes on the board.
//!
//! ```rust
//! use self::mastermind::gameplay::{DecodingBoard, CodePeg, KeyPegs};
//!
//! assert_eq!(DecodingBoard::default().rows, 12);
//! assert_eq!(CodePeg::colors(), 6);
//!
//! let b1w2 = KeyPegs::new().blacks(1).whites(2);
//! assert_eq!(format!("{}", b1w2), "BWW");
//! ```
//!
//! The two players decide in advance how many games they will play, which
//! must be an even number. One player becomes the codemaker, the other
//! the codebreaker. The codemaker chooses a pattern of four code
//! pegs. Duplicates are allowed, so the player could even choose four
//! code pegs of the same color. The chosen pattern is placed in the four
//! holes covered by the shield, visible to the codemaker but not to the
//! codebreaker. The codebreaker may have a very hard time finding out the
//! code.
//!
//! The codebreaker tries to guess the pattern, in both order and
//! color, within twelve (or ten, or eight) turns. Each guess is made
//! by placing a row of code pegs on the decoding board. Once placed,
//! the codemaker provides feedback by placing from zero to four key
//! pegs in the small holes of the row with the guess. A colored or
//! black key peg is placed for each code peg from the guess which is
//! correct in both color and position. A white key peg indicates the
//! existence of a correct color code peg placed in the wrong
//! position.
//!
//! ```rust
//! use self::mastermind::gameplay::{Pattern, KeyPegs};
//!
//! let code = Pattern::from_digits(['4', '3', '3', '2']);
//! let codemaker = Box::new(move |guess: &Pattern| code.score(*guess));
//!
//! let guess = Pattern::from_digits(['1', '2', '3', '4']);
//! let feedback = codemaker(&guess);
//!
//! assert_eq!(feedback, KeyPegs::new().whites(2).blacks(1));
//! ```

// TODO: test for "The two players decide in advance how many games
// they will play, which must be an even number."

use std::hash::{Hash, Hasher};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::iter;
use std::ops::Range;

pub struct DecodingBoard {
    pub rows: u8
}

impl Default for DecodingBoard {
    fn default() -> Self {
        DecodingBoard { rows: 12 }
    }
}

pub enum CodePeg {}
impl CodePeg {
    #[inline(always)]
    /// The game is played using code pegs of six different colors.
    pub fn colors() -> u8 {
        6
    }
}


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
pub struct KeyPegs {
    blacks: u8,
    whites: u8
}

impl KeyPegs {
    /// If the response is four colored pegs, the game is won.
    pub fn win(&self) -> bool {
        self.blacks as usize == Pattern::size()
    }

    pub fn new() -> KeyPegs {
        KeyPegs { blacks: 0, whites: 0 }
    }

    pub fn blacks(self, blacks: u8) -> KeyPegs {
        assert!(blacks as usize + self.whites as usize <= Pattern::size());
        KeyPegs { blacks: blacks, .. self }
    }

    pub fn whites(self, whites: u8) -> KeyPegs {
        assert!(self.blacks as usize + whites as usize <= Pattern::size());
        KeyPegs { whites: whites, .. self }
    }
}

impl Display for KeyPegs {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        let s = iter::repeat('B').take(self.blacks as usize)
            .chain(iter::repeat('W').take(self.whites as usize))
            .collect::<String>();
        fmt.write_str(&s)
    }
}


impl Hash for KeyPegs {
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

    /// Size of the set 1296 possible codes, 1111,1112,.., 6666
    pub fn cardinality() -> u32 {
        (CodePeg::colors() as u32).pow(Pattern::size() as u32)
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
        let base = CodePeg::colors() as u32;
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
            let remainder = (ith % CodePeg::colors() as u32) as u8;
            let digit = (('1' as u8) + remainder) as char;
            ith = ith / CodePeg::colors() as u32;
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
    pub fn score(self: &Pattern, guess: Pattern) -> KeyPegs {
        let s = self.to_digits();
        let g = guess.to_digits();

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

        KeyPegs::new().blacks(blacks as u8).whites(whites as u8)
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

/// ... a shield at one end covering a row of four large holes ...
pub type Shield = Box<Fn(&Pattern) -> KeyPegs>;



#[cfg(test)]
mod tests {
    use super::{Pattern, KeyPegs};

    #[test]
    fn scoring() {
        let (s, g) = (Pattern::from_digits(['1', '2', '3', '4']),
                      Pattern::from_digits(['2', '5', '5', '5']));
        let t1 = s.score(g);
        assert_eq!(t1, KeyPegs::new().blacks(0).whites(1));
    }

    #[test]
    fn scoring_win() {
        let (s, g) = (Pattern::from_digits(['1', '2', '3', '4']),
                      Pattern::from_digits(['1', '2', '3', '4']));
        let t1 = s.score(g);
        assert_eq!(t1, KeyPegs::new().blacks(4).whites(0));
    }
}