#![feature(collections)]
#![feature(plugin, custom_derive)]
#![feature(debug_builders)]

extern crate rand;

use rand::distributions::{IndependentSample, Range};

mod pattern;
mod solver;

use pattern::{Pattern};
use solver::{Solver};


#[cfg_attr(test, allow(dead_code))]
fn main() {
    use rand::{thread_rng};

    let secret = {
        let rng = &mut thread_rng();
        let r = Range::new(0, Pattern::cardinality());
        let x = r.ind_sample(rng);
        Pattern::ith(x)
    };
    println!("codemaker: {:?}", secret);

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
