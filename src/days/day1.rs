use crate::utils;   


//Solve the first half of the days, return the solution, here we expect an integer
pub fn solve_1() -> u32{

    //Load the input as a vector of strings
    let string_input = utils::read_input_of_the_day_lines(1);

    //Encapsulate the input to get a vec of vec, containing the calories each elf hold
    let encapsulated_input = utils::sub_array_block_from_delim(&string_input, "".to_string());

    //Convert each "word" string into int.
    let input : Vec<Vec<u32>> = encapsulated_input.into_iter()
                                  .map(|arr| utils::array_string_to_int(&arr))
                                  .collect();


                                
    //Sum each sub vec and extract the max
    let maximum: u32 = input.into_iter()
                               .map(|vec| {let sum: u32 = vec.iter().sum(); sum})
                               .max()
                               .unwrap();
    

    maximum
}

pub fn solve_2() -> u32{
     //Load the input as a vector of strings
     let string_input = utils::read_input_of_the_day_lines(1);

     //Encapsulate the input to get a vec of vec, containing the calories each elf hold
     let encapsulated_input = utils::sub_array_block_from_delim(&string_input, "".to_string());
 
     //Convert each "word" string into int.
     let input : Vec<Vec<u32>> = encapsulated_input.into_iter()
                                   .map(|arr| utils::array_string_to_int(&arr))
                                   .collect();
 
    let mut summed: Vec<u32> = input.into_iter()
                               .map(|vec| {let sum: u32 = vec.iter().sum(); sum})
                               .collect();
    summed.sort_by(|a, b| b.cmp(a));
    
    summed[0] + summed[1] + summed[2]
     

}

