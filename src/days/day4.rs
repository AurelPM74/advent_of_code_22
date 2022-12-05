use crate::utils::{self, read_input_of_the_day_lines, array_string_to_int};

//Extract the range of the two elves from the string, return a vec of int
//"8-17,16-49" --> Vec[8,17,16,49]
fn extract_ranges(string: &String) -> Vec<u32>{
    //splice th string at "-" and ','
    let splited:Vec<String> = string.split(['-', ',']).map(String::from).collect();
    //parse the string into integer and return it
    array_string_to_int(&splited)

}

//Given a vec of ranges [start_1, end_1, start_2, end_2], return true if one 
//of the two contain the other
fn is_contained(range: &Vec<u32>) -> bool{
    if range[0] <= range[2]{
        return range[1] >= range[3] || range[0] == range[2]
    } else {
        return range[1] <= range[3] || range[0] == range[2]
    }

}

//return true if the pair of ranges [start_1, end_1, start_2, end_2] overlap, i.e.
fn is_overlaping(range: &Vec<u32>) -> bool{
    //if start_1 = start_2, then there is an overlapp
    //if start_1 < start_2, there is an overlap only if end_1>= start_2
    //else there is an overlap only if end_2>= start_1
    if range[0] <= range[2]{
        return range[1] >= range[2] || range[0] == range[2]
    } else {
        return range[3] >= range[0] || range[0] == range[2]
    }
}

pub fn solve_1() -> u32{

    //We read the inputs, Vec of Strings:
    let input = read_input_of_the_day_lines(4);
    
    //We extract the range of each pair:
    let ranges: Vec<Vec<u32>> = input.iter().map(|string| extract_ranges(string)).collect();

    //We check if one range contains the other, and count all the occurences.
    let number_contain = ranges.iter().map(|range|is_contained(range) as u32).sum();

    return number_contain
}

pub fn solve_2() -> u32{

    //We read the inputs, Vec of Strings:
    let input = read_input_of_the_day_lines(4);
    
    //We extract the range of each pair:
    let ranges: Vec<Vec<u32>> = input.iter().map(|string| extract_ranges(string)).collect();

    //We check if one range contains the other, and count all the occurences.
    let number_overlap = ranges.iter().map(|range|is_overlaping(range) as u32).sum();

    return number_overlap
}