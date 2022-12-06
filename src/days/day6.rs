use crate::utils::{self, read_input_string};
use itertools::Itertools;

//Brute force algorithm, check for each windows if the all letter are different by comparing them all
fn find_block(string: &String, size_block: usize) -> u32{
    //We transform the string into an array of chars
    let characters = string.chars().collect::<Vec<char>>();

    //We do a rolling window on the string, along with an enumerate to keep track of the index
    
    let rolling = characters.windows(size_block).enumerate();

    for (index, window) in rolling{
        //Check if all the letters are different by comparing the len of the window with the set of this window
        if window.iter().unique().collect::<Vec<&char>>().len() == size_block {
            return (index + size_block) as u32
        }
    }
    0
}

//TO DO: FIND A WAY TO SKIP STEPS ON A ITERATOR
//"Smart" method for detecting the first 4 characteres are diff and where
//We try to use the least comparaisons possible:
//fn detect_diff(string: &String) -> u32{
//    //We transform the string into an array of chars
//    let characters = string.chars().collect::<Vec<char>>();
//
//    //We do a rolling window on the string, along with an enumerate to keep track of the index
//    
//    let rolling = characters.windows(4).enumerate();
//
//    loop {
//        //We check 4 letters,
//        //if the third is the same as the forth, then we can directly skip 3 windows in the future 
//        let win = rolling.next().unwrap();
//        if win.1[2] == win.1[3] {
//            std::iter::repeat(rolling.next()).take(4);
//        }
//
//    }
//    0
//}


pub fn solve_1() -> u32{
    //We read the input as a string:
    let string_input = read_input_string(6);

    //Brute force method to check for each windows if the letters are different,
    let index = find_block(&string_input, 4);

    //"Smart" method:
    //TO DO, find a way to iterate on an operator and skip some steps when specific letters are equal
    
    index
}

pub fn solve_2() -> u32{
    //We read the input as a string:
    let string_input = read_input_string(6);

    //Brute force method to check for each windows if the letters are different,
    let index = find_block(&string_input, 14);

    //"Smart" method:
    //TO DO, find a way to iterate on an operator and skip some steps when specific letters are equal
    
    index
}

//Tests with the given example

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_block_1(){
        let string_to_test_1 = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let string_to_test_2 = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        let string_to_test_3 = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");
        let string_to_test_4 = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");

        let index_1 = find_block(&string_to_test_1, 4);
        assert_eq!(index_1, 5);

        let index_2 = find_block(&string_to_test_2, 4);
        assert_eq!(index_2, 6);

        let index_3 = find_block(&string_to_test_3, 4);
        assert_eq!(index_3, 10);

        let index_4 = find_block(&string_to_test_4, 4);
        assert_eq!(index_4, 11);
    }
    #[test]
    fn test_find_block_2(){
        let string_to_test_1 = String::from("mjqjpqmgbljsphdztnvjfqwrcgsmlb");
        let string_to_test_2 = String::from("bvwbjplbgvbhsrlpgdmjqwftvncz");
        let string_to_test_3 = String::from("nppdvjthqldpwncqszvftbrmjlhg");
        let string_to_test_4 = String::from("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg");

        let index_1 = find_block(&string_to_test_1, 14);
        assert_eq!(index_1, 19);

        let index_2 = find_block(&string_to_test_2, 14);
        assert_eq!(index_2, 23);

        let index_3 = find_block(&string_to_test_3, 14);
        assert_eq!(index_3, 23);

        let index_4 = find_block(&string_to_test_4, 14);
        assert_eq!(index_4, 29);
    }
    
}
