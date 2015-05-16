#![feature(collections)]
#![feature(plugin, custom_derive)]
#![plugin(rand_macros)]
#![feature(debug_builders)]

extern crate rand;

use rand::{Rand};

mod pattern;
mod solver;

use pattern::{CodePeg, Pattern, Distance};
use pattern::CodePeg::*;

use solver::{Solver};


fn main() {
    use rand::{thread_rng};

/* TODO: turn this into a unit test.
    use CodePeg::*;
    let (s, g) = (Pattern::new([Orn, Grn, Grn, Blu]),
                  Pattern::new([Red, Red, Orn, Orn]));
    let t1 = s.score(g);
    println!("t1: {:?} : {:?} => {:?}", s, g, t1);
*/

    let mut rng = thread_rng();
    let secret = Pattern::rand(&mut rng);
    println!("codemaker: {:?}", secret);

/* random guesses to test .score()
    for _ in 0..10 {
        let guess = Pattern::rand(&mut rng);
        println!("guess    : {:?}", guess);
        println!("...........feedback: {:?}", secret.score(guess));
    }
*/

    let maker = |guess: &Pattern| secret.score(*guess);

    let mut breaker = Solver::new(&maker);
    for turn in 1..10 {
        let g = breaker.last_guess();
        println!("turn {}:    {:?}  {:?}",
                 turn, g, secret.score(g));
        let try = breaker.play();
        if try.is_some() {
            break;
        }
    }
}
