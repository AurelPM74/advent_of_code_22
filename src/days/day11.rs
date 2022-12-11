use std::borrow::Borrow;
use primefactor;
use itertools::Dedup;

const big_number: i128 = 2*3*5*7*11*13*17*19;

//Monkey structure, it will have a list of currently hold items
//The number of the monkey,
//An operation (string = "plus" or "power"),
//A number of inspections done,
//A test: [test divisible, monkey if true, monkey of false]
#[derive(Debug)]
struct Monkey{
    number: i128,
    items: Vec<i128>,
    operation_type: char,
    operation_value: i128,
    n_inspect: i128,
    test: [i128; 3]
}

//Do a round of monkey exchanges, without worry (divided by 3)
fn round_no_worry(monkeys: &mut Vec<Monkey>){

    //iterate over all the monkeys:
    for monkey_i in 0..=monkeys.len()-1 {

        //For each item
        if monkeys[monkey_i].items.len() == 0 {continue;}

        for index in 0..=monkeys[monkey_i].items.len()-1 {
            
            let item = monkeys[monkey_i].items[index];

            //We change its worry number
            let mut new_item = monkeys[monkey_i].change_worry_number(&item);

            //We do an inspection
            monkeys[monkey_i].n_inspect += 1;
            
            //We divide the worry number    
            let float_item = new_item as f32;
            new_item = (float_item/3.0).floor() as i128;

            
            //We do the test
            let monkey_to_send_to = monkeys[monkey_i].div_test(&new_item);

            
            //we push it to right monkey
            monkeys[(monkey_to_send_to) as usize].items.push(new_item);
        }
        //We remove all the items(since it throws them all)
        monkeys[monkey_i].items.clear();
    }
}

//Round but the worry is not divided by 3
//The trick to not be overflowed, it to keep the worry number in a reasonable range,
//without changing the divisibility tests
//To do that, since the only numbers we need to check divisibility are 2, 3, 5, 7, 11, 17, 19
//and those numbers are prime, then we only need to keep track of this prime factors of a number
//WARNING THIS IS FALSE, ADDITION DOES NOT BEHAVE WELL WITH PRIME DECOMPOSITION, OTHERWISE IT WAS FINE :(
//---without + it would have been fine   
//---comming back to my original idea of juste mod the numbers by 2*3*5*7*11*17*19
//It works bc:
//--- (a mod pq) mod p = a mod p so a mod 2 (or 2, 3, 4....) = a mod 2*3*5*7*11*17*19 mod 2 so div rule stay the same
//--- As before we must be careful with addition and multiplication, but modulo preserve congruences.

fn round_worry(monkeys: &mut Vec<Monkey>){

    //iterate over all the monkeys:
    for monkey_i in 0..=monkeys.len()-1 {
        

        //For each item
        if monkeys[monkey_i].items.len() == 0 {continue;}

        for index in 0..=monkeys[monkey_i].items.len()-1 {
            
            let item = monkeys[monkey_i].items[index];

            //We change its worry number
            let mut new_item = monkeys[monkey_i].change_worry_number(&item);

            //We do an inspection
            monkeys[monkey_i].n_inspect += 1;

            new_item = new_item % big_number;

            //We do the test
            let monkey_to_send_to = monkeys[monkey_i].div_test(&new_item);

            //we push it to right monkey
            monkeys[(monkey_to_send_to) as usize].items.push(new_item);
        }
        //We remove all the items(since it throws them all)
        monkeys[monkey_i].items.clear();
    }
}

//given a number n, find the decomposition \prod p^{alpha_i}_i of n, remove p_i > 19, and remove alpha_i > 1 (alpha_i) becomes 1
fn remove_prime_factor(item: &i128) -> i128{
    //prime factor decomposition
    let mut prime_factor = primefactor::PrimeFactors::from(*item as u128).to_vec();
    //remove the biggest prime factors
    //let mut new_factor = prime_factor.iter().filter(|p| **p > 19 as u128).map(|p| *p).collect::<Vec<u128>>();
    //remove the duplicate
    prime_factor.dedup();
    
    let product = prime_factor.iter().fold(1, |p, q| p * *q);
    return product as i128
} 

//method for monkey
impl Monkey{
    //method for changing the worry number
    fn change_worry_number(&self, worry_number: &i128) -> i128{
        let new_number = match self.operation_type {
            '+' => {
                worry_number + self.operation_value
            },
            '*' => {
                worry_number * self.operation_value
            },
            '^' => {
                worry_number * worry_number
            }
            _ => panic!("Not an operation")
        };
        new_number
    }
    
    //Method to do the divisiblity test, return the number of the monkey to send the item to
    fn div_test(&self, item_number: &i128) -> i128{

        let test: bool = item_number%self.test[0] == 0;
        //  println!("{} is {} by {}", item_number, test,self.test[0]);
        if test {
            return self.test[1]
        } else {
            return self.test[2]
        }
    }
}



pub fn solve_1() -> i128{
    //Create the monkeys structs
    let monkey_0 = Monkey{
        number: 0,
        items: vec![89, 95, 92, 64, 87, 68],
        n_inspect: 0,
        operation_type: '*',
        operation_value: 11,
        test: [2, 7, 4],
    };
    let monkey_1 = Monkey{
        number: 1,
        items: vec![87, 67],
        n_inspect: 0,
        operation_type: '+',
        operation_value: 1,
        test: [13, 3, 6],
    };
    let monkey_2 = Monkey{
        number: 2,
        items: vec![95, 79, 92, 82, 60],
        n_inspect: 0,
        operation_type: '+',
        operation_value: 6,
        test: [3, 1, 6],
    };
    let monkey_3 = Monkey{
        number: 3,
        items: vec![67, 97, 56],
        n_inspect: 0,
        operation_type: '^',
        operation_value: 1,
        test: [17, 7, 0],
    };
    let monkey_4 = Monkey{
        number: 4,
        items: vec![80, 68, 87, 94, 61, 59, 50, 68],
        n_inspect: 0,
        operation_type: '*',
        operation_value: 7,
        test: [19, 5, 2],
    };
    let monkey_5 = Monkey{
        number: 5,
        items: vec![73, 51, 76, 59],
        n_inspect: 0,
        operation_type: '+',
        operation_value: 8,
        test: [7, 2, 1],
    };
    let monkey_6 = Monkey{
        number: 6,
        items: vec![92],
        n_inspect: 0,
        operation_type: '+',
        operation_value: 5,
        test: [11, 3, 0],
    };
    let monkey_7 = Monkey{
        number: 6,
        items: vec![99, 76, 78, 76, 79, 90, 89],
        n_inspect: 0,
        operation_type: '+',
        operation_value: 7,
        test: [5, 4, 5],
    };

    let mut monkeys = vec![monkey_0, monkey_1, monkey_2, monkey_3, monkey_4, monkey_5, monkey_6, monkey_7];
    //Do 20 rounds
    for i in 1..=20{
        println!("round {}", i);
        round_no_worry(&mut monkeys);
    }


    let mut sorted = monkeys.iter().map(|m| m.n_inspect).collect::<Vec<i128>>();
    sorted.sort();
    let power = sorted[sorted.len() - 1] * sorted[sorted.len() - 2];

    power

}

pub fn solve_2() -> u128{
    //Create the monkeys structs
    let monkey_0 = Monkey{
        number: 0,
        items: vec![89, 95, 92, 64, 87, 68],
        n_inspect: 0,
        operation_type: '*',
        operation_value: 11,
        test: [2, 7, 4],
    };
    let monkey_1 = Monkey{
        number: 1,
        items: vec![87, 67],
        n_inspect: 0,
        operation_type: '+',
        operation_value: 1,
        test: [13, 3, 6],
    };
    let monkey_2 = Monkey{
        number: 2,
        items: vec![95, 79, 92, 82, 60],
        n_inspect: 0,
        operation_type: '+',
        operation_value: 6,
        test: [3, 1, 6],
    };
    let monkey_3 = Monkey{
        number: 3,
        items: vec![67, 97, 56],
        n_inspect: 0,
        operation_type: '^',
        operation_value: 1,
        test: [17, 7, 0],
    };
    let monkey_4 = Monkey{
        number: 4,
        items: vec![80, 68, 87, 94, 61, 59, 50, 68],
        n_inspect: 0,
        operation_type: '*',
        operation_value: 7,
        test: [19, 5, 2],
    };
    let monkey_5 = Monkey{
        number: 5,
        items: vec![73, 51, 76, 59],
        n_inspect: 0,
        operation_type: '+',
        operation_value: 8,
        test: [7, 2, 1],
    };
    let monkey_6 = Monkey{
        number: 6,
        items: vec![92],
        n_inspect: 0,
        operation_type: '+',
        operation_value: 5,
        test: [11, 3, 0],
    };
    let monkey_7 = Monkey{
        number: 6,
        items: vec![99, 76, 78, 76, 79, 90, 89],
        n_inspect: 0,
        operation_type: '+',
        operation_value: 7,
        test: [5, 4, 5],
    };

    let mut monkeys = vec![monkey_0, monkey_1, monkey_2, monkey_3, monkey_4, monkey_5, monkey_6, monkey_7];
    //Do 10.000 rounds
    for i in 1..=10000 {
        //println!("round {}", i);
        round_worry(&mut monkeys);
    }

    let mut sorted = monkeys.iter().map(|m| m.n_inspect).collect::<Vec<i128>>();
    sorted.sort();
    let power = sorted[sorted.len() - 1] as u128 * sorted[sorted.len() - 2] as u128;

    power

}





mod tests {
    
    use itertools::Itertools;

    use super::*;
    //Test first part of the puzzle
    #[test]
    fn test_solve_1(){
        //Create the monkeys structs
        let monkey_0 = Monkey{
            number: 0,
            items: vec![79, 98],
            n_inspect: 0,
            operation_type: '*',
            operation_value: 19,
            test: [23, 2, 3],
        };
        let monkey_1 = Monkey{
            number: 1,
            items: vec![54, 65, 75, 74],
            n_inspect: 0,
            operation_type: '+',
            operation_value: 6,
            test: [19, 2, 0],
        };
        let monkey_2 = Monkey{
            number: 2,
            items: vec![79, 60, 97],
            n_inspect: 0,
            operation_type: '^',
            operation_value: 1,
            test: [13, 1, 3],
        };
        let monkey_3 = Monkey{
            number: 3,
            items: vec![74],
            n_inspect: 0,
            operation_type: '+',
            operation_value: 3,
            test: [17, 0, 1],
        };
        let mut monkeys = vec![monkey_0, monkey_1, monkey_2, monkey_3];
        //Do 20 rounds
        for i in 1..=20{
            //println!("round {}", i);
            round_no_worry(&mut monkeys);
        }

        let mut sorted = monkeys.iter().map(|m| m.n_inspect).collect::<Vec<i128>>();
        sorted.sort();
        println!("{:?}", sorted);
        let power = sorted[sorted.len() - 1] * sorted[sorted.len() - 2];

        assert_eq!(power, 10605)
    }

    #[test]
    fn test_solve_2(){
        //Create the monkeys structs
        let monkey_0 = Monkey{
            number: 0,
            items: vec![79, 98],
            n_inspect: 0,
            operation_type: '*',
            operation_value: 19,
            test: [23, 2, 3],
        };
        let monkey_1 = Monkey{
            number: 1,
            items: vec![54, 65, 75, 74],
            n_inspect: 0,
            operation_type: '+',
            operation_value: 6,
            test: [19, 2, 0],
        };
        let monkey_2 = Monkey{
            number: 2,
            items: vec![79, 60, 97],
            n_inspect: 0,
            operation_type: '^',
            operation_value: 1,
            test: [13, 1, 3],
        };
        let monkey_3 = Monkey{
            number: 3,
            items: vec![74],
            n_inspect: 0,
            operation_type: '+',
            operation_value: 3,
            test: [17, 0, 1],
        };
        let mut monkeys = vec![monkey_0, monkey_1, monkey_2, monkey_3];
        //Do 10.000 rounds
        for i in 1..=10000{
            round_worry(&mut monkeys);
        }

        let mut sorted = monkeys.iter().map(|m| m.n_inspect).collect::<Vec<i128>>();
        sorted.sort();
        println!("{:?}", sorted);
        let power = sorted[sorted.len() - 1] as u128 * sorted[sorted.len() - 2] as u128;

        assert_eq!(power, 2713310158)
    }
}