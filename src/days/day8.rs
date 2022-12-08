use crate::utils::{self, read_input_of_the_day_lines};
use std::fs;

//Compute the number of trees that can be seen horizontaly (from left to right and from right to left)
fn visible_tree_horizontaly(input: &Vec<Vec<u32>>, matrix: &mut Vec<Vec<u32>>) -> u32{
    //number of visible trees
    let mut n = 0;
    let len = input[0].len();

    //Iterate over the lines
    for line in input.iter().enumerate(){

        //Create an iterator and a reverse iterator over the tree in that line 
        let mut it_line = line.1.iter().enumerate();
        let mut it_line_rev = line.1.iter().rev().enumerate();
        

        // We loop, stop when we cannot see any tree neither from the left nor from the right

        //initialize the current tree at -1 so that the border tree always count
        let mut previous_max_right:i32 = -1;
        let mut previous_max_left:i32 = -1;

        for i in 0..=len - 1 {
            let current_left_three = match it_line.next() {
                Some(t) => t,
                None => {break;}
            };
            //If the tree is smaller than the previous max, then the max block the view
            //otherwise we see the three
            if previous_max_left < *current_left_three.1 as i32{
                //In that case we mark the three as seen:
                //We mark the tree as seen if its not already seen in the matrix
                if matrix[line.0][current_left_three.0] == 0 {
                    matrix[line.0][current_left_three.0] = 1;
                    n += 1
                }
                //We raise the max:
                previous_max_left = current_left_three.1.clone() as i32
            }

            //Same thing from the right
            let current_right_three = match it_line_rev.next() {
                Some(t) => t,
                None => {break;}
            };
            //If the tree is smaller than the previous max, then the max block the view
            //otherwise we see the three
            if previous_max_right < *current_right_three.1 as i32{
                //In that case we mark the three as seen:
                //We mark the tree as seen if its not already seen in the matrix
                if matrix[line.0][len - 1 - current_right_three.0] == 0 {
                    matrix[line.0][len - 1 -current_right_three.0] = 1;
                    n += 1
                }
                //We raise the max:
                previous_max_right = current_right_three.1.clone() as i32
            }

            

        }

    }
    return n
}


//Parse the input from vec of string to vec of vec of chars
fn parse_input(input: Vec<String>) -> Vec<Vec<u32>>{
    let parsed_input:Vec<Vec<u32>> = input.iter()
                                          .map(|string| string.chars().map(|c| c.to_digit(10).unwrap()).collect())
                                          .collect();
    parsed_input

}

fn row_to_col(input: &Vec<Vec<u32>>) ->Vec<Vec<u32>>{
    let i_columns = 0..= input[0].len()-1; 
    let mut columns = Vec::new();
    //Not really optimized, could not think of a good way for now
    for i in i_columns{
        let col:Vec<u32> = input.iter()
                                .map(|x| x[i])
                                .collect();
        columns.push(col);
    }
    columns
}

pub fn solve_1() -> u32{
    //We read the input as vec of string corresponding to lines
    let input = read_input_of_the_day_lines(8);

    //We parse the vec of string into a vec of vec of u32
    let parsed = parse_input(input);

    //First we compute the tree that can be seen from left to right and from right to left
    //We use a matrix of zero to keep track of already visible tree
    let len = parsed[0].len();
    let mut matrix = std::vec::from_elem(std::vec::from_elem(0 as u32, len), len);

    let visible_horizon = visible_tree_horizontaly(&parsed, &mut matrix);

    //To compute the visible tree from top to bottom and reversely we flip the rows into columns and do the same thing
    //We compute the columns 
    let columns = row_to_col(&parsed);
    let mut matrix_columns = row_to_col(&matrix);
    //Compute the vertical visible tree
    let visible_vert = visible_tree_horizontaly(&columns, &mut matrix_columns);


    return visible_horizon + visible_vert

}

//Compute the scenery score of a given position
fn scenery_score((i, j): (usize, usize), input: &Vec<Vec<u32>>) -> u32 {
    
    let tree_height = input[i][j];
    //Numbers of tree in each direction (L, R, U, D)
    let mut view_direction = [1, 1, 1, 1];
    //Compute the number of tree seen in each direction:
    //Looking right:
    let mut current_tree_r = j - 1;
    //While we did not reach the border and the trees are smaller than the tree height, we increase the score
    while current_tree_r != 0 && tree_height > input[i][current_tree_r]{
        view_direction[0] += 1;
        current_tree_r -= 1
    }
    //Looking left:
    let mut current_tree_l = j + 1;
    //While we did not reach the border and the trees are smaller than the tree height, we increase the score
    while current_tree_l != input[0].len()-1 && tree_height > input[i][current_tree_l]{
        view_direction[1] += 1;
        current_tree_l += 1
    }
    //Looking up:
    let mut current_tree_u = i - 1;
    //While we did not reach the border and the trees are smaller than the tree height, we increase the score
    while current_tree_u != 0 && tree_height > input[current_tree_u][j]{
        view_direction[2] += 1;
        current_tree_u -= 1
    }
    //Looking down:
    let mut current_tree_d = i + 1;
    //While we did not reach the border and the trees are smaller than the tree height, we increase the score
    while current_tree_d != input[0].len()-1 && tree_height > input[current_tree_d][j]{
        view_direction[3] += 1;
        current_tree_d += 1
    }
    //println!("{:?}", (i, j, tree_height, view_direction));
    return view_direction[0]*view_direction[1]*view_direction[2]*view_direction[3]
}

pub fn solve_2() -> u32{
    //We read the input as vec of string corresponding to lines
    let input = read_input_of_the_day_lines(8);

    //We parse the vec of string into a vec of vec of u32
    let parsed = parse_input(input);

    //We iterate on all the positions possible, and call scenery_score, we ignore the border (score = 0)
    let mut current_max_scenery = 0;
    for i in 1..=parsed[0].len()-2{
        for j in 1..=parsed[0].len()-2{
            let score = scenery_score((i, j), &parsed);
            if score > current_max_scenery{
                current_max_scenery = score;
            }
        }
    }
    current_max_scenery
}

mod tests {
    
    use super::*;
    //Test first part of the puzzle
    #[test]
    fn test_solve_1(){
        //test input
        let test_input = Vec::from([String::from("30373"),
        String::from("25512"),
        String::from("65332"),
        String::from("33549"),
        String::from("35390")]);
        
        
        let trees = parse_input(test_input);
        println!("{:?}", trees);
        let len = trees[0].len();
        let mut matrix = std::vec::from_elem(std::vec::from_elem(0 as u32, len), len);

        let visib_hori = visible_tree_horizontaly(&trees, &mut matrix);
        println!("{:?}", matrix);

        let columns = row_to_col(&trees);
        let mut col_matrix = row_to_col(&matrix);
        
        let visib_vert = visible_tree_horizontaly(&columns, &mut col_matrix);

        let tot = visib_vert + visib_hori;
        println!("{:?}", row_to_col(&col_matrix));

        assert_eq!(tot, 21)


    }
    //Test for the second part
    #[test]
    fn test_solve_2(){
        //test input for first part
        let test_input = Vec::from([String::from("30373"),
        String::from("25512"),
        String::from("65332"),
        String::from("33549"),
        String::from("35390")]);
        
        
        let trees = parse_input(test_input);
        //We iterate on all the positions possible, and call scenery_score, we ignore the border (score = 0)
        let mut current_max_scenery = 0;
        let mut cur_index = (0, 0);
        for i in 1..=trees[0].len()-2{
            for j in 1..=trees[0].len()-2{
                let score = scenery_score((i, j), &trees);
                if score > current_max_scenery{
                    current_max_scenery = score;
                    cur_index = (i, j);
                }
            }
        }
        println!("{:?}", cur_index);
        assert_eq!(current_max_scenery, 8)


    }
}
