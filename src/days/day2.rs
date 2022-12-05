use crate::utils;   
use std::collections::HashMap;

    
//Solve the first half of the days, return the solution, here we expect an integer
pub fn solve_1() -> u32{

    let loup_table:[[u32; 3];3] = [[3+1, 6+2, 0+3],[0+1, 3+2, 6+3],[6+1, 0+2, 3+3]];
    
    let mut indices = HashMap::new();

    indices.insert(String::from("A"), 0);
    indices.insert(String::from("B"), 1);
    indices.insert(String::from("C"), 2);
    indices.insert(String::from("X"), 0);
    indices.insert(String::from("Y"), 1);
    indices.insert(String::from("Z"), 2);



    let vec_input = utils::read_input_of_the_day_lines(2);
    
    let split_input = vec_input.into_iter()
                                                 .map(|str|str.split_whitespace().map(String::from).collect::<Vec<String>>())
                                                 .collect::<Vec<_>>();
    
    let input_to_indices = split_input.into_iter()
                                                      .map(|vec|(indices.get(&vec[0]).unwrap(), indices.get(&vec[1]).unwrap()))
                                                      .collect::<Vec<_>>();

    let scores_of_rounds: Vec<u32> = input_to_indices.into_iter().map(|(a, b)| loup_table[*a][*b]).collect();

    let sum_of_scores = scores_of_rounds.iter().sum();
    
    sum_of_scores
}

pub fn solve_2() -> u32{

    let loup_table:[[u32; 3];3] = [[3, 1+3, 2+6],
                                  [1, 2+3, 3+6],
                                  [2, 3+3, 1+6]];
    
    let mut indices = HashMap::new();

    indices.insert(String::from("A"), 0);
    indices.insert(String::from("B"), 1);
    indices.insert(String::from("C"), 2);
    indices.insert(String::from("X"), 0);
    indices.insert(String::from("Y"), 1);
    indices.insert(String::from("Z"), 2);

    let vec_input = utils::read_input_of_the_day_lines(2);
    //let vec_input = Vec::from([String::from("A Y"),String::from("B X"),String::from("C Z")]);
    
    let split_input = vec_input.into_iter()
                                                 .map(|str|str.split_whitespace().map(String::from).collect::<Vec<String>>())
                                                 .collect::<Vec<_>>();

    let input_to_indices = split_input.into_iter()
                                                 .map(|vec|(indices.get(&(vec[0])).unwrap(), indices.get(&vec[1]).unwrap()))
                                                 .collect::<Vec<_>>();
    //println!("{:?}", input_to_indices);

    let scores_of_rounds: Vec<u32> = input_to_indices.into_iter().map(|(a, b)| loup_table[*a][*b]).collect();

    //println!("{:?}", scores_of_rounds);

    let sum_of_scores = scores_of_rounds.iter().sum();

    sum_of_scores
}

