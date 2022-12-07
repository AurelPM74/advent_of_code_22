#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

mod days;
mod utils;
mod test;
use std::time::{Duration, Instant};

use days::{day1, day2, day3, day4, day5, day6, day7};



fn main() {
    //println!("The solution 1 for day 1 is: {}, the second is {}", day1::solve_1(), day1::solve_2());
    //println!("The solution 1 for day 2 is: {}, the second is {}", day2::solve_1(), day2::solve_2());
    //println!("The solution 1 for day 3 is: {}, the second is {}", day3::solve_1(), day3::solve_2());
    //println!("The solution 1 for day 4 is: {}, the second is {}", day4::solve_1(), day4::solve_2());
    //println!("The solution 1 for day 5 is: {}, the second is {}", day5::solve_1(), day5::solve_2());
    //println!("The solution 1 for day 6 is: {}, the second is {}", day6::solve_1(), day6::solve_2());
    let start = Instant::now();
    println!("The solution 1 for day 7 is: {}, the second is {}", day7::solve_1(), day7::solve_2());
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}


