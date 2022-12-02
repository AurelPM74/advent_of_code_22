#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod days;
mod utils;

use days::{day1, day2};



fn main() {
    println!("The solution 1 for day 1 is: {}, the second is {}", day1::solve_1(), day1::solve_2());
    println!("The solution 1 for day 2 is: {}, the second is {}", day2::solve_1(), day2::solve_2());
}
