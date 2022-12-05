use crate::utils; 

//extract the common letter in a the two half of a string 
fn extract_common_letter(string: &String) -> char{

    //Split the string into two equal parts
    let (half_1, half_2)  =  string.split_at(string.len()/2);

    //Store the chars of the first half in a bool array of size 58 [a..z,some_ut8_symb,A..Z],
    //To simplify and use the utf8 encoding as indices of the array
    let mut one_encoding = [false; 58];

    for character in half_1.chars(){
        //Since A -> 65 in utf8 we minus 67 to the utf8 encoding
        one_encoding[(character as usize) - 65] = true;
    }
    //for each characters of the second helf we check if the value of one_encoding[char] is true,
    //this will indicate that the character wasin the first half.
    for character in half_2.chars(){
        if one_encoding[(character as usize) - 65]{
            return character
        }
    }
    '0'
}
//extract the common letter of a vec of 3 strings
fn extract_common_letter_three(strings: &Vec<String>) ->char{

    //Store the chars of the first half in a bool array of size 58 [a..z,some_ut8_symb,A..Z],
    //To simplify and use the utf8 encoding as indices of the array
    let mut one_encoding = [false; 58];

    for character in strings[0].chars(){
        //Since A -> 65 in utf8 we minus 67 to the utf8 encoding
        one_encoding[(character as usize) - 65] = true;
    }
    
    //We find the common letters between the two first strings and store it in a similar array as one_encoding
    let mut one_encoding_first_two = [false; 58];
    for character in strings[1].chars(){
        if one_encoding[(character as usize) - 65] {
            one_encoding_first_two[(character as usize) - 65] = true
        }
    }
    //We search among all the characters in the last string if its on the common letter of the first 2
    for character in strings[2].chars(){
        if one_encoding_first_two[(character as usize) - 65] {
            return character
        }
    }
    '0'
    
}

//function giving the priorities of the chars:
fn priorities(c: char) ->u32{
    //to utf8 encoding
    let c_utf = c as u32;

    //if the character is lowercase we simply return the utf8 encoding -96 as a..z->97... 
    //Otherwise we return c-65 + 27 = c - 38 as A..Z->90
    if c_utf >= 97{
        return c_utf - 96
    } else {
        return c_utf - 38
    }

}

pub fn solve_1() -> u32{

    //read the input, Vec of strings
    let input_lines = utils::read_input_of_the_day_lines(3);

    //we iterate on all the strings, and extrct the common letter
    let split_input: Vec<char> = input_lines.into_iter()
        .map(|backpack|extract_common_letter(&backpack))
        .collect();
    
    //we extract the priorities of the characters and sum them:
    let sum_priorities: u32 = split_input.into_iter().map(|c| priorities(c)).sum();

    sum_priorities
}

pub fn solve_2() -> u32{
    //read the input, Vec of strings
    let input_lines = utils::read_input_of_the_day_lines(3);

    //We iterate on groups of 3 strings and find the common letter:
    let common_letter: Vec<char> = input_lines.chunks(3)
                                   .map(|group| extract_common_letter_three(&group.to_vec()))
                                   .collect();
    
    //we extract the priorities of the characters and sum them:
    let sum_priorities: u32 = common_letter.into_iter().map(|c| priorities(c)).sum();
    
    sum_priorities
}